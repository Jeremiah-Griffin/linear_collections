#[cfg(test)]
pub mod test;
use std::{collections::TryReserveError, hash::Hash, intrinsics::transmute_unchecked};

use crate::array_vec::RawArrayVec;

pub mod map;
pub mod set;

#[derive(Debug)]
///A vector which allocates at least `STACK_CAPACITY` elements onto the stack.
pub struct FatVec<T, const STACK_CAPACITY: usize> {
    array: RawArrayVec<T, STACK_CAPACITY>,
    //TODO: should replace this vec with an other implementation.
    //TODO: fallibele collections: replace this with a custom fallible vec implementation.
    ///For now, with panicking operations we call some method that ensures the next call will not panic. This is a bit flimsy.
    ///Vec includes its own `len`, which isn't necessary for us to track two.
    ///RawVec seems to basically work for this
    vec: Vec<T>,
    ///this tracks both the number of elements inside the vec as well as the array.
    len: usize,
}

impl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {
    //***constructors***

    ///Creates a new, empty `FatVec`. Without allocating on the heap.
    ///This can contain up to `STACK_CAPACITY` elements without performing any
    ///heap allocations.
    pub fn new() -> Self {
        Self {
            array: RawArrayVec::uninit(),
            vec: Vec::new(),
            len: 0,
        }
    }

    ///Creates a `FatVec` with the provided array as the stack resident elements.
    ///The length of the supplied array will become the `STACK_CAPCITY` of the returned `FatVec` *AND* the length of the array.
    ///There is no interface to mutate the length without manipulating the elements on the stack.
    ///
    ///
    ///Does not allocate to the heap.
    //We use a seperate L so we don't have to declare the STACK_CAPACITY both in the array and the function type parameter.
    pub const fn with_array(array: [T; STACK_CAPACITY]) -> FatVec<T, STACK_CAPACITY> {
        Self {
            //SAFETY:
            //MaybeUninit T and T are guaranteed to have the same size, layout, and alignment.
            //We can't transmute because of a compiler bug [47966](https://github.com/rust-lang/rust/issues/47966)
            //so we're forced to use transmut_unchecked instead, which doesnt do that broken compile-time check.
            //TODO: REMOVE THIS USE OF TRANSMUTE_UNCHECKED AND CORE_INTRINSICS
            //doing a pointer cast breaks the drop handling of stack resident values of T. We use a `transmute_unchecked` to ensure
            //no copy is made so no `drop` gets run following the assignment.
            array: unsafe { transmute_unchecked(array) },
            vec: Vec::new(),
            len: STACK_CAPACITY,
        }
    }

    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements without reallocating.
    ///Upon return, this `FatVec` will be able to hold `STACK_CAPACITY + `capacity` elements without
    ///re-allocating.
    pub fn with_heap_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        Ok(Self {
            array: RawArrayVec::uninit(),
            vec: Vec::try_with_capacity(capacity)?,
            len: 0,
        })
    }

    //***methods***

    pub fn array_len(&self) -> usize {
        match self.len <= STACK_CAPACITY {
            true => self.len(),
            false => STACK_CAPACITY,
        }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        let len = self.array_len();

        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec
        unsafe { self.array.iter_to(len) }.chain(self.vec.iter())
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        let len = self.array_len();

        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec
        unsafe { self.array.iter_mut_to(len) }.chain(self.vec.iter_mut())
    }

    ///Returns the number of items in this `FatVec`
    pub const fn len(&self) -> usize {
        self.len
    }

    ///Returns the maximum number of items this `FatVec` can hold both on the stack and heap without reallocating.
    ///Note this is not the remaing space in the `FatVec`, but a count of all capacity, consumed or not.
    pub fn capacity(&self) -> usize {
        self.vec.capacity() + STACK_CAPACITY
    }

    ///TODO: how to test and ensure that each value gets dropped?
    pub fn clear(&mut self) {
        //Ensure that all  elements are dropped. Bounded by array len means this cannot find uninitalized
        //memory.
        unsafe { self.array.clear_to(self.array_len()) }
        self.vec.clear();
        self.len = 0;
    }

    ///Appends the element to this `FatVec`, returning an error on failure.
    pub fn push(&mut self, value: T) -> Result<(), TryReserveError> {
        //We don't need to check if we're within the bounds of the collection as reserve will do this
        //for us.
        let new_len = self.len.saturating_add(1);

        match STACK_CAPACITY > self.len() {
            true => unsafe { self.array.insert_at(self.len, value) },
            false => {
                //call reserve on the vec as necessary to ensure pushing to it doesn't panic.
                if self.vec.capacity() < new_len {
                    self.reserve(1)?;
                }

                self.vec.push(value);
            }
        }
        self.len = new_len;
        Ok(())
    }

    /// Removes the last element from a `FatVec` and returns it, or [`None`] if the `FatVec` is empty.
    pub fn pop(&mut self) -> Option<T> {
        //we keep this seperate from remove(self.len) as in the
        //stack resident arm we can avoid having to shift the elements on the array to the left
        //as we can just decrement the "stack pointer" and leave the type as is. Drop handling is done by moving
        //the T out of pop. Remove can't do this as it needs to be able to pull elements arbitrarily from within the array
        //meaning it needs to shift left to keep the values contiguous.
        let r = match self.len() {
            0 => None,
            idx if idx <= STACK_CAPACITY => unsafe {
                Some(self.array.remove(idx, self.array_len()))
            },
            _ => self.vec.pop(),
        };

        self.len = self.len.saturating_sub(1);

        r
    }
    /// Removes the element from a `FatVec` and returns it, or [`None`] if the `FatVec` is empty.
    ///TODO: miri testing stuff.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        match self.len() {
            //SAFETY:
            //Do not remove this arm without auditing the unsafe blocks below.
            //guaranteeing that we return on a len of 0 means we can safely and cheaply subtract from the index without underflowing.
            0 => None,
            //value is resident on stack
            _ if index <= STACK_CAPACITY => {
                let r = unsafe { self.array.remove(index, self.array_len()) };
                self.len -= 1;
                Some(r)
            }
            //value is resident on heap
            _ => {
                let vec_idx = index - STACK_CAPACITY;
                self.len -= 1;
                match vec_idx > self.vec.len() {
                    true => None,
                    false => Some(self.vec.remove(vec_idx)),
                }
            }
        }
    }

    ///Gets a shared reference to the item at the requested, returning `None` if idx is outside the range of the `FatVec`.
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            return None;
        }

        match STACK_CAPACITY > idx {
            //SAFETY:
            //Because we maintain length seperately of the vec and array, we can rely on IDX not to be out of bounds for
            //either these accesses.
            true => unsafe { Some(self.array.get(idx)) },
            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's
            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.
            false => self.vec.get(idx - STACK_CAPACITY),
        }
    }

    ///Gets a unique reference to the item at the requested, returning `None` if idx is outside the range of the `FatVec`.
    pub fn get_mut<'a>(&'a mut self, idx: usize) -> Option<&'a mut T> {
        if idx > self.len {
            return None;
        }
        match STACK_CAPACITY > idx {
            //SAFETY:
            //Because we maintain length seperately from the vec and array, we can rely on IDX not to be out of bounds for
            //either these accesses.
            true => unsafe { Some(self.array.get_mut(idx)) },
            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's
            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.
            false => self.vec.get_mut(idx - STACK_CAPACITY),
        }
    }

    /// Tries to reserve the minimum capacity for at least `additional`
    /// elements to be inserted in the given `Vec<T>`.
    /// After calling `reserve`, capacity will be
    /// equal to `self.len() + additional` if it returns `Ok(())`.
    /// Does nothing if the capacity is already sufficient.
    pub fn reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        //We use try_reserve_exact to keep memory use as compact as possible at the expense of throughput.
        self.vec.try_reserve_exact(additional).map_err(|e| e.into())
    }

    ///Shrinks the heap storage of this `FatVec` to match capacity.
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }
}

impl<const STACK_CAPACITY: usize, T: PartialEq> PartialEq for FatVec<T, STACK_CAPACITY> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len()
            && self
                .iter()
                .enumerate()
                .all(|(i, this)| other.get(i).is_some_and(|o| *o == *this))
    }
}

impl<const STACK_CAPACITY: usize, T: Eq> Eq for FatVec<T, STACK_CAPACITY> {}

impl<const STACK_CAPACITY: usize, T: Hash> Hash for FatVec<T, STACK_CAPACITY> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|t| t.hash(state))
    }
}