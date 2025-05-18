#[cfg(feature = "serde")]
mod serde;
mod raw;
pub mod set;
pub mod map;
pub mod error;
#[cfg(test)]
mod test;
#[cfg(kani)]
mod verification;

use error::PushError;
pub use raw::RawStackList;
use std::{hash::Hash, num::NonZero};

#[derive(Debug)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
///A list growable to `CAPACITY` which places all its items on the stack.
pub struct StackList<T, const CAPACITY: usize> {
    raw: RawStackList<T, CAPACITY>,
    length: usize,
}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {
    ///Creates a new, empty `StackList`.
    pub fn new() -> Self {
        Self {
            raw: RawStackList::uninit(),
            length: 0,
        }
    }

    ///Calls `drop` on all elements in this list, in place.
    pub fn clear(&mut self) {
        match NonZero::new(self.length){
            Some(len) => {
                //SAFETY:
                //bound by length so will not go out of bounds or into uninit memory
                unsafe { self.raw.clear_to(len)};
                self.length = 0;
            },

            None => return,
        }
    }

    ///Returns an iterator over the elements of this `StackList`.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { self.raw.iter_to(self.length) }
    }
    ///Returns an iterator over the elements of this `StackList where each element is mutable.
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { self.raw.iter_mut_to(self.length) }
    }

    ///Returns the number of items in this `StackList`.
    pub const fn len(&self) -> usize {
        self.length
    }

    ////Creates a StaticList from an array, with the StaticList assuming the length of the array as its Capacity.
    pub fn from_array(array: [T; CAPACITY]) -> Self {
        Self {
            raw: RawStackList::from_array(array),
            length: CAPACITY,
        }
    }

    ///Removes the last element in this `StackList`, returning `None` if this list is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.length
            .checked_sub(1)
            .and_then(|new| self.remove(new))
        }
        //self.remove(self.length)
    

    pub fn push(&mut self, value: T) -> Result<(), PushError> {
        match self.length.checked_add(1){
            Some(new) => {
                unsafe { self.raw.insert_at(self.length, value) };
                self.length = new;
                Ok(())
            },
            None => Err(PushError::WouldExceedCapacity),
        }
    }

    ///Removes the element at `index` from this `StackList`.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        match CAPACITY > index && self.length > 0 {
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

    ///Retrieves element at `index`, returning `None` if `index` is out of bounsd. 
    pub fn get(&self, index: usize) -> Option<&T> {
        match CAPACITY > index  && index < self.length {
            //SAFETY: we track len and know it is not > CAPACITY in this arm
            //so there is no possibility of UB
            true => unsafe { Some(self.raw.get(index)) },
            false => None,
        }
    }


    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match CAPACITY > index && index < self.length {
            //SAFETY: we track len and know it is not > CAPACITY in this arm
            //so there is no possibility of UB
            true => unsafe { Some(self.raw.get_mut(index)) },
            false => None,
        }
    }

}

impl<const CAPACITY: usize, T: PartialEq> PartialEq for StackList<T, CAPACITY> {
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

impl<const CAPACITY: usize, T: Eq> Eq for StackList<T, CAPACITY> {}

impl<const CAPACITY: usize, T: Hash> Hash for StackList<T, CAPACITY> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|t| t.hash(state))
    }
}
