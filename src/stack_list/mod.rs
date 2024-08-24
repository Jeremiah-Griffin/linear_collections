use std::{array, intrinsics::transmute_unchecked, mem::MaybeUninit};

use error::StackListError;

pub mod error;
#[cfg(test)]
mod test;
#[derive(Debug)]
///We need the core functionality of ArrayVec throughout the crate, but don't need the overhead
///of tracking `length` internally. So we don't!
pub(crate) struct RawStackList<T, const CAPACITY: usize> {
    array: [MaybeUninit<T>; CAPACITY],
}

impl<T, const CAPACITY: usize> RawStackList<T, CAPACITY> {
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
        let r = unsafe { self.array.get_unchecked(index).assume_init_read() };

        //shift values right of `r` left.
        //start off by one?

        let mut loop_idx = index + 1;
        //Using the length of the entire `FatVec` is okay as we've established in the match arm
        //that all elements are allocated on the array. This saves us the step of reading the next element
        //to copy for the entire stack *so long as* we ensure all successive elements beyond the array len are MaybeUninit::uninit.
        while loop_idx < length {
            //all elements shifted
            unsafe {
                //SAFETY: guaranteed safe as loop_idx is guaranteed by the match arm to be <= len.
                let next = self.array.get_unchecked_mut(loop_idx) as *mut MaybeUninit<T>;
                //SAFETY: guaranteed safe as loop_idx is guaranteed by the match arm to be <= len *AND* we subtracting one from that.
                //we ensure that this doesn't underflow above.
                //saturating sub just to be doubly sure.
                let curr =
                    self.array.get_unchecked_mut(loop_idx.saturating_sub(1)) as *mut MaybeUninit<T>;

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
pub struct StackList<T, const CAPACITY: usize> {
    raw: RawStackList<T, CAPACITY>,
    length: usize,
}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {
    pub const fn len(&self) -> usize {
        self.length
    }
}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {
    pub fn new() -> Self {
        Self {
            raw: RawStackList::uninit(),
            length: 0,
        }
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

    /*
    pub fn from_array<const LENGTH: usize>(array: [T; LENGTH]) where LENGTH <= CAPACITY{
        unimplemented!()
    }*/

    pub fn pop(&mut self) -> Option<T> {
        self.remove(self.length)
    }

    pub fn push(&mut self, value: T) -> Result<(), StackListError> {
        match self.length < CAPACITY {
            true => {
                unsafe { self.raw.insert_at(self.length, value) };
                self.length += 1;

                Ok(())
            }
            false => Err(StackListError::WouldExceedCapacity),
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
