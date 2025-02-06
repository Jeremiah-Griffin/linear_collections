#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
pub mod test;

use std::hash::Hash;

use crate::stack_list::RawStackList;

pub mod map;
pub mod set;

#[derive(Debug)]
///A vector which allocates at least `STACK_CAPACITY` elements onto the stack.
pub struct FatVec<T, const STACK_CAPACITY: usize> {
    stack_list: RawStackList<T, STACK_CAPACITY>,
    //TODO: should replace this vec with an other implementation.
    //TODO: fallibele collections: replace this with a custom fallible vec implementation.
    ///For now, with panicking operations we call some method that ensures the next call will not panic. This is a bit flimsy.
    ///Vec includes its own `len`, which isn't necessary for us to track two.
    ///RawVec seems to basically work for this
    vec: Vec<T>,
    ///this tracks both the number of elements inside the vec as well as the array.
    len: usize,
}

impl<T: Clone, const STACK_CAPACITY: usize> Clone for FatVec<T, STACK_CAPACITY> {
    fn clone(&self) -> Self {
        Self {
            //SAFETY:
            //bound by length so no risk of reading into uninitialized memory, or memory not owned by the stack list.
            stack_list: unsafe { self.stack_list.clone_to(self.len) },
            vec: self.vec.clone(),
            len: self.len,
        }
    }
}

pub struct Iter<'a, T, const STACK_CAPACITY: usize> {
    idx: usize,
    fatvec: &'a FatVec<T, STACK_CAPACITY>,
}

impl<'a, T, const STACK_CAPACITY: usize> Iterator for Iter<'a, T, STACK_CAPACITY> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx.checked_add(1)?;
        self.fatvec.get(self.idx)
    }
}

impl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {
    //***constructors***

    ///Creates a new, empty `FatVec`. Without allocating on the heap.
    ///This can contain up to `STACK_CAPACITY` elements without performing any
    ///heap allocations.
    pub fn new() -> Self {
        Self {
            stack_list: RawStackList::uninit(),
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
            //MaybeUninit T and T are guaranteed to have the same size, layout, and alignment.
            //We can't transmute because of a compiler bug [47966](https://github.com/rust-lang/rust/issues/47966)
            //so we're forced to use transmut_unchecked instead, which doesnt do that broken compile-time check.
            //TODO: REMOVE THIS USE OF TRANSMUTE_UNCHECKED AND CORE_INTRINSICS
            //doing a pointer cast breaks the drop handling of stack resident values of T. We use a `transmute_unchecked` to ensure
            //no copy is made so no `drop` gets run following the assignment.
            stack_list: RawStackList::from_array(array),
            vec: Vec::new(),
            len: STACK_CAPACITY,
        }
    }

    ///Gets the length of the array of the stack
    const fn array_len(&self) -> usize {
        match self.len <= STACK_CAPACITY {
            true => self.len,
            false => STACK_CAPACITY,
        }
    }

    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements without reallocating
    ///If `capacity` is less than or equal to `STACK_CAPACITY` the total capacity of this `FatVec` will be equal to `STACK_CAPACITY`.
    pub fn with_capacity(capacity: usize) -> Self {
        let heap_capacity = capacity.saturating_sub(STACK_CAPACITY);

        Self::with_heap_capacity(heap_capacity)
    }

    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements without reallocating.
    ///Upon return, this `FatVec` will be able to hold `STACK_CAPACITY + `capacity` elements without
    ///re-allocating.
    pub fn with_heap_capacity(capacity: usize) -> Self {
        Self {
            stack_list: RawStackList::uninit(),
            vec: Vec::with_capacity(capacity),
            len: 0,
        }
    }

    //***methods***
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        Iter {
            idx: 0,
            fatvec: &self,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        let len = self.array_len();
        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec
        unsafe { self.stack_list.iter_mut_to(len) }.chain(self.vec.iter_mut())
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
        //SAFETY:
        //Ensure that all  elements are dropped. Bounded by array len means this cannot find uninitalized
        //memory.

        unsafe { self.stack_list.clear_to(self.array_len()) }

        self.vec.clear();
        self.len = 0;
    }

    ///Appends the element to this `FatVec`, returning an error on failure.
    pub fn push(&mut self, value: T) {
        //We don't need to check if we're within the bounds of the collection as reserve will do this
        //for us.
        let new_len = self.len.saturating_add(1);

        match STACK_CAPACITY > self.len() {
            true => unsafe { self.stack_list.insert_at(self.len, value) },
            false => {
                self.vec.push(value);
            }
        }
        self.len = new_len;
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
                Some(self.stack_list.remove(idx, self.array_len()))
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
            //guaranteeing that we return None on a len of 0 means we can safely and cheaply subtract from the index without underflowing.
            0 => None,
            //value is resident on stack
            _ if index <= STACK_CAPACITY => {
                //SAFETY
                //bound by index means we are guaranteed to be within the initialized portion of the stack list.
                let r = unsafe { self.stack_list.remove(index, self.array_len()) };

                //Shift elements from heap to stack, if necessary.
                if self.vec.len() > 0 {
                    let elem = self.vec.remove(0);
                    //SAFETY:
                    //STACK_CAPACITY - 1 is always guaranteed to be last element in the RawStackList.
                    //Further, we know both that that last element is going to be unoccupied - the prior call to remove guarantees that all elements before it have shifted left -
                    //and we also knwo that the the RawStackList has space for only one element now, because the vec is non empty.
                    //
                    //The only time the RawStackList will have < CAPACITY elements is when no elements have overflowed onto the heap.
                    unsafe {
                        //using staurating sub as it's going to be evaluated at compile time anyway, but we technically
                        //can't bound the  STACK_CAPACITY > 0
                        self.stack_list
                            .insert_at(STACK_CAPACITY.saturating_sub(1), elem)
                    };
                }

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
            true => unsafe { Some(self.stack_list.get(idx)) },
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
            true => unsafe { Some(self.stack_list.get_mut(idx)) },
            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's
            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.
            false => self.vec.get_mut(idx - STACK_CAPACITY),
        }
    }

    /// Tries to reserve the minimum capacity for at least `additional`
    /// elements to be inserted in the given `Vec<T>`.
    /// After calling `reserve`, capacity will be
    /// equal to `self.len() + additional`.
    /// Does nothing if the capacity is already sufficient.
    pub fn reserve(&mut self, additional: usize) {
        //We use try_reserve_exact to keep memory use as compact as possible at the expense of throughput.
        self.vec.reserve_exact(additional)
    }

    ///Shrinks the heap storage of this `FatVec` to match capacity.
    pub fn shrink_to_fit(&mut self) {
        self.vec.shrink_to_fit()
    }
}

impl<const STACK_CAPACITY: usize, T: PartialEq> PartialEq for FatVec<T, STACK_CAPACITY> {
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

impl<const STACK_CAPACITY: usize, T: Eq> Eq for FatVec<T, STACK_CAPACITY> {}

impl<const STACK_CAPACITY: usize, T: Hash> Hash for FatVec<T, STACK_CAPACITY> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.iter().for_each(|t| t.hash(state))
    }
}
