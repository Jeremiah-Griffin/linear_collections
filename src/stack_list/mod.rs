#[cfg(feature = "serde")]
mod serde;

pub mod error;
pub mod map;
pub mod raw_stack_list;
pub mod set;
#[cfg(test)]
mod test;
#[cfg(kani)]
mod verification;

use error::StackListError;
pub use raw_stack_list::RawStackList;
use std::{
    hash::Hash,
    num::NonZero,
    ops::{Deref, DerefMut},
};

use crate::list::List;

#[derive(Default, Debug)]
///TODO: is this unsound? can kani::Arbitrary produce `MaybeUninit<T>` which are both `uninit` and invalid? Or
/// can `Arbitrary` only produce valid values of it?
#[cfg_attr(kani, derive(kani::Arbitrary))]
///A list growable to `CAPACITY` which places all its items on the stack.
pub struct StackList<T, const CAPACITY: usize> {
    raw: RawStackList<T, CAPACITY>,
    length: usize,
}

impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {
    ///TODO: add verification and description
    pub fn as_slice(&self) -> &[T] {
        let pointer = self.raw.as_slice().as_ptr() as *const T;

        //SAFETY:
        //MaybeUninit<T> and T have the same memory layout
        //Later, when we create the slice reference we bind by the length parameter,
        //ensuring we only access initialized elements
        //the length parameter ensures all elements are valid.
        unsafe { std::slice::from_raw_parts(pointer, self.length) }
    }
    ///TODO: add verification and description
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        let pointer = self.raw.as_mut_slice().as_mut_ptr() as *mut T;

        //SAFETY:
        //MaybeUninit<T> and T have the same memory layout
        //Later, when we create the slice reference we bind by the length parameter,
        //ensuring we only access initialized elements
        //the length parameter ensures all elements are valid.
        unsafe { std::slice::from_raw_parts_mut(pointer, self.length) }
    }

    ///Returns true if the store is empty and false otherwise.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    ////Creates a StaticList from an array, with the StaticList assuming the length of the array as its Capacity.
    pub fn from_array(array: [T; CAPACITY]) -> Self {
        Self {
            raw: RawStackList::from_array(array),
            length: CAPACITY,
        }
    }

    ///Removes the element at `index` from this `StackList`.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        //check if index is in bounds
        match CAPACITY > index && self.length > 0 {
            true => {
                let raw = &mut self.raw;
                let length = self.length;

                //SAFETY: we track len and know it is not > CAPACITY in this arm
                //so there is no possibility of UB
                let r = unsafe { raw_stack_list::remove!(raw, index, length) };

                self.length = self.length.checked_sub(1)?;

                Some(r)
            }
            false => None,
        }
    }

    ///Retrieves element at `index`, returning `None` if `index` is out of bounsd.
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = &self.raw;
        match CAPACITY > index && index < self.length {
            //SAFETY: we track len and know it is not > CAPACITY in this arm
            //so there is no possibility of UB
            true => Some(unsafe { raw_stack_list::get!(raw, index) }),
            false => None,
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let list = &mut self.raw;
        match CAPACITY > index && index < self.length {
            true => Some(unsafe { raw_stack_list::get_mut!(list, index) }),
            //SAFETY: we track len and know it is not > CAPACITY in this arm
            //so there is no possibility of UB
            //true => Some(unsafe {raw_stack_list::get!(raw, index)}),
            false => None,
        }
    }
}

impl<T, const CAPACITY: usize> List<T> for StackList<T, CAPACITY> {
    type Error = StackListError;

    fn new() -> Self {
        Self {
            raw: RawStackList::uninit(),
            length: 0,
        }
    }
    fn capacity(&self) -> usize {
        CAPACITY
    }

    fn clear(&mut self) {
        match NonZero::new(self.length) {
            Some(len) => {
                let ref mut list = self.raw;
                //SAFETY:
                //bound by length so will not go out of bounds or into uninit memory
                unsafe { raw_stack_list::clear_to!(list, len) };
                self.length = 0;
            }

            None => return,
        }
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        let raw = &self.raw;
        let length = self.length;
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { raw_stack_list::iter_to!(raw, length) }
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        let raw = &mut self.raw;
        let length = self.length;
        //SAFETY:
        //bound by length so will not go out of bounds or into uninit memory
        unsafe { raw_stack_list::iter_mut_to!(raw, length) }
    }

    fn len(&self) -> usize {
        self.length
    }

    fn pop(&mut self) -> Option<T> {
        self.length.checked_sub(1).and_then(|new| self.remove(new))
    }

    fn push(&mut self, item: T) -> Result<(), Self::Error> {
        match self.length.checked_add(1) {
            Some(new) => {
                let raw = &mut self.raw;
                let len = self.length;
                //SAFETY:
                //Guaranteed to be in bounds as length is never out of bounds.
                unsafe { raw_stack_list::insert_at!(raw, len, item) };
                self.length = new;
                Ok(())
            }
            None => Err(StackListError::WouldExceedCapacity),
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

impl<T, const CAPACITY: usize> Deref for StackList<T, CAPACITY> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const CAPACITY: usize> DerefMut for StackList<T, CAPACITY> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}
