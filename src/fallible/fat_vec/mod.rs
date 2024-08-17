#[cfg(test)]
pub mod test;
use std::{array, intrinsics::transmute_unchecked, mem::MaybeUninit};
pub mod map {
    use crate::{AsMutSlice, LinearMap};

    use super::FatVec;

    #[derive(Debug, PartialEq, Eq)]
    ///A map type backed by an FatVec, a vector with stack space to hold up to
    ///`STACK_CAPACITY` items on the stack. The remaining overflow onto the heap.
    pub struct FatVecMap<K, V, const STACK_CAPACITY: usize> {
        fat_vec: FatVec<(K, V), STACK_CAPACITY>,
    }

    impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> LinearMap<K, V>
        for FatVecMap<K, V, STACK_CAPACITY>
    {
        type Backing = FatVec<(K, V), STACK_CAPACITY>;
        fn as_slice(&self) -> &[(K, V)] {
            /*
            &self.fat_vec.as_slice()
            */

            unimplemented!()
        }

        fn into_inner(self) -> Self::Backing {
            self.fat_vec
        }
    }

    impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> AsMutSlice<K, V>
        for FatVecMap<K, V, STACK_CAPACITY>
    {
        fn as_mut_slice(&mut self) -> &mut [(K, V)] {
            unimplemented!()
            //&mut self.fat_vec
        }
    }
}
pub mod set {
    use super::map::FatVecMap;

    #[derive(Debug, PartialEq, Eq)]
    ///A set type backed by an FatVec, a vector with stack space to hold up to
    ///`STACK_CAPACITY` items on the stack. The remaining overflow onto the heap.
    pub struct FatVecSet<K, const STACK_CAPACITY: usize> {
        fat_vec: FatVecMap<K, (), STACK_CAPACITY>,
    }
}

use super::error::AllocationError;

#[derive(Debug)]
///A vector which allocates at least `STACK_CAPACITY` elements onto the stack.
pub struct FatVec<T, const STACK_CAPACITY: usize> {
    array: [MaybeUninit<T>; STACK_CAPACITY],
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
            array: array::from_fn(|_| MaybeUninit::uninit()),
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
    pub fn with_array(array: [T; STACK_CAPACITY]) -> FatVec<T, STACK_CAPACITY> {
        Self {
            //SAFETY:
            //MaybeUninit T and T are guarenteed t o have th same size, layout, and alignment.
            //We can't transmute because of a compiler bug [47966](https://github.com/rust-lang/rust/issues/47966)
            //so we're forced to use transmut_unchecked instead, which doesnt do that broken compile-time check.
            //TODO: REMOVE THIS USE OF TRANSMUTE_UNCHECKED
            //doing a pointer cast breaks the drop handling of stack resident values of T. We use a `transmute_unchecked` to ensure
            //no copy is made so no `drop` gets run following the assignment.
            array: unsafe { transmute_unchecked(array) },
            vec: Vec::new(),
            len: STACK_CAPACITY,
        }
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
        //If there are more items than can be held on the stack
        //the stack length is capped at STACK_CAPACITY.
        let array_len = match self.len() > STACK_CAPACITY {
            true => STACK_CAPACITY,
            false => self.len(),
        };

        //SAFETY:
        //Ensure that all  elements are dropped. Bounded by array len means this cannot find uninitalized
        //memory.
        unsafe {
            self.array[0..array_len]
                .iter_mut()
                .for_each(|t| t.assume_init_drop())
        }
        self.vec.clear();
        self.len = 0;
    }

    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements without reallocating.
    ///Upon return, this `StackVec` will be able to hold `STACK_CAPACITY + `capacity` elements without
    ///re-allocating.
    pub fn with_heap_capacity(capacity: usize) -> Result<Self, AllocationError> {
        Ok(Self {
            array: array::from_fn(|_| MaybeUninit::uninit()),
            vec: Vec::try_with_capacity(capacity)?,
            len: 0,
        })
    }

    ///Creates an iterator over the values inside tis `FatVec`.
    pub fn iter<'a>(&'a self) -> Iter<'a, STACK_CAPACITY, T> {
        Iter {
            idx: 0,
            svec: &self,
        }
    }

    ///Appends the element to this `FatVec`, returning an error on failure.
    pub fn push(&mut self, value: T) -> Result<(), AllocationError> {
        //We don't need to check if we're within the bounds of the collection as reserve will do this
        //for us.
        let new_len = self.len.saturating_add(1);

        match STACK_CAPACITY > self.len() {
            true => {
                unsafe { self.array.get_unchecked_mut(self.len).write(value) };
            }
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

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.    
    pub fn pop(&mut self) -> Option<T> {
        let r = match self.len() {
            0 => None,
            l if l <= STACK_CAPACITY => unsafe {
                Some(self.array.get_unchecked(l).assume_init_read())
            },
            _ => self.vec.pop(),
        };

        self.len = self.len.saturating_sub(1);

        r
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
            true => unsafe { Some(self.array.get_unchecked(idx).assume_init_ref()) },
            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's
            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.
            false => self.vec.get(idx - STACK_CAPACITY),
        }
    }

    ///Gets a unique reference to the item at the requested, returning `None` if idx is outside the range of the `FatVec`.
    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx > self.len {
            return None;
        }
        match STACK_CAPACITY > idx {
            //SAFETY:
            //Because we maintain length seperately from the vec and array, we can rely on IDX not to be out of bounds for
            //either these accesses.
            true => unsafe { Some(self.array.get_unchecked_mut(idx).assume_init_mut()) },
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
    pub fn reserve(&mut self, additional: usize) -> Result<(), AllocationError> {
        //We use try_reserve_exact to keep memory use as compact as possible at the expense of throughput.
        self.vec.try_reserve_exact(additional).map_err(|e| e.into())
    }

    ///Shrinks the heap storage of this `FatVec` to match capacity.
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }
}

pub struct Iter<'a, const STACK_CAPACITY: usize, T> {
    idx: usize,
    svec: &'a FatVec<T, STACK_CAPACITY>,
}

impl<'a, const STACK_CAPACITY: usize, T> Iterator for Iter<'a, STACK_CAPACITY, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.svec.get(self.idx);
        self.idx += 1;
        t
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
