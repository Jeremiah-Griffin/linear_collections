#![allow(dead_code)]
#![cfg_attr(feature = "nightly_fallible", allow(internal_features))]
#![cfg_attr(feature = "nightly_fallible", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly_fallible", feature(try_reserve_kind))]
#![cfg_attr(feature = "nightly_fallible", feature(try_with_capacity))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_ext))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_trait))]
pub mod array;
///This is in the crate root because it's used internally but we still need it throughout both
///fallible and panicking crates internally.
mod array_vec {
    use std::{array, intrinsics::transmute_unchecked, mem::MaybeUninit};

    use error::ArrayVecError;

    #[derive(Debug)]
    ///We need the core functionality of ArrayVec throughout the crate, but don't need the overhead
    ///of tracking `length` internally. So we don't!
    pub(crate) struct RawArrayVec<T, const CAPACITY: usize> {
        array: [MaybeUninit<T>; CAPACITY],
    }

    impl<T, const CAPACITY: usize> RawArrayVec<T, CAPACITY> {
        ///initializes all elements of this array to MaybeUninit::uninit.
        pub fn uninit() -> Self {
            Self {
                array: array::from_fn(|_| MaybeUninit::uninit()),
            }
        }

        ///SAFETY: UB if `limit` is beyond CAPACITY.
        ///Drops all elements up to `limit`, exclusive.
        pub unsafe fn clear_to(&mut self, limit: usize) {
            self.array[0..limit]
                .iter_mut()
                .for_each(|t| unsafe { t.assume_init_drop() });
        }

        ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized memory.
        pub unsafe fn get(&self, index: usize) -> &T {
            //SAFETY: addressed by the disclosure on the function signature
            let item = unsafe { self.array.get_unchecked(index) };

            //SAFETY: addressed by the disclosure on the function signature
            let item = unsafe { transmute_unchecked(item) };

            item
        }

        ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized memory.
        pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
            //SAFETY: addressed by the disclosure on the function signature
            let item = unsafe { self.array.get_unchecked_mut(index) };

            //SAFETY: addressed by the disclosure on the function signature
            let item = unsafe { transmute_unchecked(item) };

            item
        }

        ///Reports wether the specified index is within the capacity of this structure.
        pub const fn is_within_capacity(&self, index: usize) -> bool {
            CAPACITY > index
        }

        pub unsafe fn iter_to<'a>(&'a self, index: usize) -> impl Iterator<Item = &'a T> {
            //TODO: This can panic
            self.array[0..index]
                .iter()
                //SAFETY:
                //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
                .map(|t| unsafe { t.assume_init_ref() })
        }

        pub unsafe fn iter_mut_to<'a>(
            &'a mut self,
            index: usize,
        ) -> impl Iterator<Item = &'a mut T> {
            //TODO: This can panic
            self.array[0..index]
                .iter_mut()
                //SAFETY:
                //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
                .map(|t| unsafe { t.assume_init_mut() })
        }

        ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized element.
        pub unsafe fn remove(&mut self, index: usize, length: usize) -> T {
            //SAFETY: addressed by the disclosure on the function signature
            //take value
            let r = unsafe { self.array.get_unchecked(index).assume_init_read() };

            //shift values right of `r` left.
            //start off by one?

            let mut loop_idx = index + 1;
            //Using the length of the entire `FatVec` is okay as we've established in the match arm
            //that all elements are allocated on the array. This saves us the step of reading the next element
            //to copy for the entire stack *so long as* we ensure all successive elements beyond the array len are MaybeUninit::uninit.
            while loop_idx <= length {
                //all elements shifted
                unsafe {
                    //SAFETY: guaranteed safe as loop_idx is guaranteed by the match arm to be <= len.
                    let next = self.array.get_unchecked_mut(loop_idx) as *mut MaybeUninit<T>;
                    //SAFETY: guaranteed safe as loop_idx is guaranteed by the match arm to be <= len *AND* we subtracting one from that.
                    //we ensure that this doesn't underflow above.
                    //saturating sub just to be doubly sure.
                    let curr = self.array.get_unchecked_mut(loop_idx.saturating_sub(1))
                        as *mut MaybeUninit<T>;

                    *curr = next.read();
                }

                loop_idx += 1;
            }

            r
        }

        pub unsafe fn insert_at(&mut self, index: usize, value: T) {
            unsafe { self.array.get_unchecked_mut(index).write(value) };
        }
    }

    #[derive(Debug)]
    pub struct ArrayVec<T, const CAPACITY: usize> {
        raw: RawArrayVec<T, CAPACITY>,
        length: usize,
    }

    impl<T, const CAPACITY: usize> ArrayVec<T, CAPACITY> {
        pub fn new() -> Self {
            Self {
                raw: RawArrayVec::uninit(),
                length: 0,
            }
        }

        pub fn pop(&mut self) -> Option<T> {
            self.remove(self.length)
        }

        pub fn push(&mut self, value: T) -> Result<(), ArrayVecError> {
            match self.length < CAPACITY {
                true => {
                    self.length += 1;
                    unsafe { self.raw.insert_at(self.length, value) };

                    Ok(())
                }
                false => Err(ArrayVecError::WouldExceedCapacity),
            }
        }

        pub fn remove(&mut self, index: usize) -> Option<T> {
            match self.raw.is_within_capacity(index) && self.length > 0 {
                //SAFETY: we track len and know it is not > CAPACITY in this arm
                //so there is no possibility of UB
                true => {
                    let r = unsafe { self.raw.remove(index, self.length) };

                    self.length -= 1;

                    Some(r)
                }
                false => None,
            }
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            match self.raw.is_within_capacity(index) && index < self.length {
                //SAFETY: we track len and know it is not > CAPACITY in this arm
                //so there is no possibility of UB
                true => unsafe { Some(self.raw.get(index)) },
                false => None,
            }
        }

        pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
            match self.raw.is_within_capacity(index) && index < self.length {
                //SAFETY: we track len and know it is not > CAPACITY in this arm
                //so there is no possibility of UB
                true => unsafe { Some(self.raw.get_mut(index)) },
                false => None,
            }
        }
    }

    pub mod error {
        use std::{error::Error, fmt::Display};

        #[derive(Clone, Copy, Debug)]
        pub enum ArrayVecError {
            WouldExceedCapacity,
        }

        impl Display for ArrayVecError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    ArrayVecError::WouldExceedCapacity => write!(f, "push would exceed capacity"),
                }
            }
        }

        impl Error for ArrayVecError {}
    }
    pub mod map {}
    pub mod set {}
    #[cfg(test)]
    mod test {}
}

//We make the modules public but *not* the contained types. Certain projects need only one type or the other.
//It would be unfortunate for a low level library which can only use fallible types to be forced to specify "FallibleFatVec".
//instead, if a library user *must* use both types, they should use the qualified path up to the module, fallible or panicking.

#[cfg(feature = "nightly_fallible")]
//added but not exposed pending miri testing
//We always compile fallible as the infallible versions are just fallible with panic called on the additional methods.
pub mod fallible;

#[cfg(feature = "panicking")]
pub mod panicking;

/*
#[cfg(feature = "serde")]
mod serde;
*/
#[cfg(test)]
mod test;

///Sealed trait to provide mutable iteration without allowing consumers
///to violate the invariants of the map types
pub(crate) trait MapIterMut<K, V> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a;
}
