use std::{
    array,
    hash::Hash,
    mem::{ManuallyDrop, MaybeUninit},
    ops::Deref,
    ptr::{addr_of, addr_of_mut, copy},
};

#[cfg(feature = "serde")]
mod serde;

use error::PushError;

pub mod error;
#[cfg(test)]
mod test;
///NOTES ON IMPLS:
///While we can implement Clone cheaply, with a cheakc
///Clones an eq need to over all the *initialized* elements of the array to check for equivalence.
//while we could iiterate ofer the entire capacity, this would be expensive for large capacites and small lengths.
//additionally, we would need to zero ever uninit both initially and avfter every drop. That is a wast of writes.
//
#[derive(Debug)]
///We need the core functionality of ArrayVec throughout the crate, but don't need the overhead
///of tracking `length` internally. So we don't!
pub(crate) struct RawStackList<T, const CAPACITY: usize> {
    array: [MaybeUninit<T>; CAPACITY],
}

///Safe as
fn t_to_maybeuninit<T, const LENGTH: usize>(array: [T; LENGTH]) -> [MaybeUninit<T>; LENGTH] {
    #[cfg(feature = "nightly")]
    unsafe {
        core::intrinsics::transmute_unchecked(array)
    }

    //TODO: this is a disgusting hack that results in doubled stack usage.
    #[cfg(not(feature = "nightly"))]
    {
        ///We use ManuallyDrop to ensure that whil the copied T get deallocated, we dont call drop both for the copy at the end of the scope
        ///*and* for the returned MaybeUninit<T>
        let old_array = ManuallyDrop::new(array);
        unsafe { (old_array.as_ptr() as *const [MaybeUninit<T>; LENGTH]).read() }
    }
}

impl<T, const CAPACITY: usize> RawStackList<T, CAPACITY> {
    //**constructors**//
    ///initializes all elements of this array to MaybeUninit::uninit.
    pub fn uninit() -> Self {
        Self {
            array: array::from_fn(|_| MaybeUninit::uninit()),
        }
    }

    ////Creates a RawStaticList from an array.
    ///LIMITATIONS:
    ///Unfortunately, because of the instability of const generic expressions, we can't assert statically that
    ///the length of the array is <= CAPACITY. I am also not comfortable using the const_generic_expr feature
    ///in production code. When that feature stabilizes, this restriction will be loosened and  lists with lengths shorter
    ///than their capacity will become possible to write in safe code.
    pub fn from_array(array: [T; CAPACITY]) -> Self {
        RawStackList {
            //SAFETY:
            //The representation fo a MaybeUninity T and T are identical.
            //The lengths are the same in this case as well.
            array: t_to_maybeuninit(array),
        }
    }

    //**methods**//

    ///SAFETY: UB if `limit` is beyond CAPACITY.
    ///Drops all elements up to `limit`, exclusive.
    pub unsafe fn clear_to(&mut self, limit: usize) {
        self.array[0..limit]
            .iter_mut()
            .for_each(|t| unsafe { t.assume_init_drop() });
    }

    ///TODO: figure this out in the face of tryclone. What does the sig look like.
    pub unsafe fn clone_to(&self, limit: usize) -> Self
    where
        T: Clone,
    {
        let mut array: [MaybeUninit<T>; CAPACITY] = array::from_fn(|_| MaybeUninit::uninit());

        self.array[0..limit]
            .iter()
            .enumerate()
            .for_each(|(idx, t)| {
                let referenced = unsafe { t.assume_init_ref().clone() };

                unsafe { array.get_unchecked_mut(idx).write(referenced) };
            });

        Self { array }
    }

    ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized memory.
    pub unsafe fn get(&self, index: usize) -> &T {
        //SAFETY: addressed by the disclosure on the function signature
        unsafe { self.array.get_unchecked(index).assume_init_ref() }
    }

    ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized memory.
    pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
        //SAFETY: addressed by the disclosure on the function signature
        unsafe { self.array.get_unchecked_mut(index).assume_init_mut() }
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

    pub unsafe fn iter_mut_to<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut T> {
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
        let t = unsafe { self.array.get_unchecked(index).assume_init_read() };

        //shift values right of `r` left.
        let elements_after_index = (length.saturating_sub(index)).saturating_sub(1);

        copy(
            (addr_of!(self.array) as *const MaybeUninit<T>).add(index + 1),
            (addr_of_mut!(self.array) as *mut MaybeUninit<T>).add(index),
            elements_after_index,
        );

        /*
        if elements_after_index > 0 {
            std::ptr::copy(
                self.array.get_unchecked(index + 1).as_ptr(),
                self.array.get_unchecked_mut(index).as_mut_ptr(),
                elements_after_index,
            );
        }*/

        t
    }

    ///SAFETY: UB if index >= CAPACITY.
    pub unsafe fn insert_at(&mut self, index: usize, value: T) {
        //SAFETY: addressed by the disclosure on the function signature
        unsafe { self.array.get_unchecked_mut(index).write(value) };
    }
}

/*
impl<const CAPACITY: usize, T: Clone> RawStackList<T, CAPACITY> {
    fn clone(&self) -> Self {
        //SAFETY:
        //CAPACITY on both types is guaranteed to be identical
        //The representation of two MaybeUninit<T> where T == T are identical.
        let array = unsafe {
            self.array
                .as_ptr()
                .cast::<[MaybeUninit<T>; CAPACITY]>()
                .read()
        };

        //SAFETY:
        //we're getting from the array's own index.
        let array: [MaybeUninit<T>; CAPACITY] = array::from_fn(|i| unsafe {
            let reference = self.array.get_unchecked(i).clone();
            reference
        });

        Self { array }
    }
}*/
#[derive(Debug)]
pub struct StackList<T, const CAPACITY: usize> {
    raw: RawStackList<T, CAPACITY>,
    length: usize,
}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {
    pub fn new() -> Self {
        Self {
            raw: RawStackList::uninit(),
            length: 0,
        }
    }

    pub fn clear(&mut self) {
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { self.raw.clear_to(self.length) };
        self.length = 0;
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { self.raw.iter_to(self.length) }
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { self.raw.iter_mut_to(self.length) }
    }

    pub const fn len(&self) -> usize {
        self.length
    }

    ////Creates a StaticList from an array, with the StaticList assuming the length of the array as its Capacity.
    ///LIMITATIONS:
    ///Unfortunately, because of the instability of const generic expressions, we can't assert statically that
    ///the length of the array is <= CAPACITY. I am also not comfortable using the const_generic_expr feature
    ///in production code. When that feature stabilizes, this restriction will be loosened and  lists with lengths shorter
    ///than their capacity will become possible to write in safe code.
    pub fn from_array(array: [T; CAPACITY]) -> Self {
        Self {
            raw: RawStackList::from_array(array),
            length: CAPACITY,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.remove(self.length)
    }

    pub fn push(&mut self, value: T) -> Result<(), PushError> {
        match self.length < CAPACITY {
            true => {
                unsafe { self.raw.insert_at(self.length, value) };
                self.length += 1;

                Ok(())
            }
            false => Err(PushError::WouldExceedCapacity),
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

impl<const STACK_CAPACITY: usize, T: PartialEq> PartialEq for StackList<T, STACK_CAPACITY> {
    fn eq(&self, other: &Self) -> bool {
        //just want to explicitly evaluate this first as it's much cheaper.
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .enumerate()
            .all(|(i, this)| other.get(i).is_some_and(|o| *o == *this))
    }
}

impl<const STACK_CAPACITY: usize, T: Eq> Eq for StackList<T, STACK_CAPACITY> {}

impl<const STACK_CAPACITY: usize, T: Hash> Hash for StackList<T, STACK_CAPACITY> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|t| t.hash(state))
    }
}
