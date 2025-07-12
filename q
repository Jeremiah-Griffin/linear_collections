[1mdiff --git a/src/fat_vec/map.rs b/src/fat_vec/map.rs[m
[1mindex 3a343ea..f22f52d 100644[m
[1m--- a/src/fat_vec/map.rs[m
[1m+++ b/src/fat_vec/map.rs[m
[36m@@ -1,4 +1,4 @@[m
[31m-use crate::{Map, map::MapIterMut};[m
[32m+[m[32muse crate::{Map, list::List, map::MapIterMut};[m
 use std::collections::TryReserveError;[m
 [m
 use super::FatVec;[m
[1mdiff --git a/src/fat_vec/mod.rs b/src/fat_vec/mod.rs[m
[1mindex ed2f3df..31b010b 100644[m
[1m--- a/src/fat_vec/mod.rs[m
[1m+++ b/src/fat_vec/mod.rs[m
[36m@@ -4,7 +4,10 @@[m [mmod serde;[m
 #[cfg(test)][m
 pub mod test;[m
 [m
[31m-use crate::stack_list::{RawStackList, raw_stack_list};[m
[32m+[m[32muse crate::{[m
[32m+[m[32m    list::List,[m
[32m+[m[32m    stack_list::{RawStackList, raw_stack_list},[m
[32m+[m[32m};[m
 use std::{[m
     array, collections::TryReserveError, hash::Hash, intrinsics::transmute_unchecked,[m
     mem::MaybeUninit, num::NonZero,[m
[36m@@ -28,18 +31,140 @@[m [mpub struct FatVec<T, const STACK_CAPACITY: usize> {[m
 }[m
 [m
 impl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {[m
[31m-    //***constructors***[m
[31m-    ///Creates a new, empty `FatVec`. Without allocating on the heap.[m
[31m-    ///This can contain up to `STACK_CAPACITY` elements without performing any[m
[31m-    ///heap allocations.[m
[31m-    pub fn new() -> Self {[m
[31m-        Self {[m
[31m-            stack_list: RawStackList::uninit(),[m
[31m-            vec: Vec::new(),[m
[31m-            len: 0,[m
[32m+[m[32m    pub fn array_len(&self) -> usize {[m
[32m+[m[32m        match self.len() <= STACK_CAPACITY {[m
[32m+[m[32m            true => self.len(),[m
[32m+[m[32m            false => STACK_CAPACITY,[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    ///Returns a shared reference to the item at the requested, returning `None` if idx is outside the range of the `FatVec`.[m
[32m+[m[32m    pub fn get(&self, idx: usize) -> Option<&T> {[m
[32m+[m[32m        if idx >= self.len {[m
[32m+[m[32m            return None;[m
[32m+[m[32m        }[m
[32m+[m
[32m+[m[32m        //SAFETY:[m
[32m+[m[32m        //the early return above guarantees we do not access[m
[32m+[m[32m        //out of bounds[m
[32m+[m[32m        unsafe { Some(self.get_unchecked(idx)) }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    ///Returns a unique reference to the item at `idx`, returning `None` if idx is outside the range of the `FatVec`.[m
[32m+[m[32m    pub fn get_mut<'a>(&'a mut self, idx: usize) -> Option<&'a mut T> {[m
[32m+[m[32m        if idx >= self.len {[m
[32m+[m[32m            return None;[m
         }[m
[32m+[m[32m        //SAFETY:[m
[32m+[m[32m        //the early return above guarantees we do not access[m
[32m+[m[32m        //out of bounds[m
[32m+[m[32m        unsafe { Some(self.get_unchecked_mut(idx)) }[m
     }[m
 [m
[32m+[m[32m    ///Returns a shared reference to the item at `idx`.[m
[32m+[m[32m    ///SAFETY:[m
[32m+[m[32m    ///UB if idx is >= the length of this `FatVec`.[m
[32m+[m[32m    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {[m
[32m+[m[32m        let list = &self.stack_list;[m
[32m+[m[32m        match STACK_CAPACITY > idx {[m
[32m+[m[32m            //SAFETY:[m
[32m+[m[32m            //Because we maintain length seperately of the vec and array, we can rely on IDX not to be out of bounds for[m
[32m+[m[32m            //either these accesses.[m
[32m+[m[32m            true => unsafe { raw_stack_list::get!(list, idx) },[m
[32m+[m[32m            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's[m
[32m+[m[32m            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.[m
[32m+[m[32m            false => unsafe { self.vec.get_unchecked(idx - STACK_CAPACITY) },[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    //***methods***[m
[32m+[m
[32m+[m[32m    ///Returns a unique reference to the item at `idx`.[m
[32m+[m[32m    ///SAFETY:[m
[32m+[m[32m    ///UB if idx is >= the length of this `FatVec`.[m
[32m+[m[32m    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut T {[m
[32m+[m[32m        let list = &mut self.stack_list;[m
[32m+[m[32m        match STACK_CAPACITY > idx {[m
[32m+[m[32m            //SAFETY:[m
[32m+[m[32m            //Because we maintain length seperately from the vec and array, we can rely on IDX not to be out of bounds for[m
[32m+[m[32m            //either these accesses.[m
[32m+[m[32m            true => unsafe { raw_stack_list::get_mut!(list, idx) },[m
[32m+[m[32m            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's[m
[32m+[m[32m            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.[m
[32m+[m[32m            false => unsafe { self.vec.get_unchecked_mut(idx - STACK_CAPACITY) },[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    /// Removes the element at `idx` from this `FatVec` and returns it, or `None` if the `FatVec` is empty or if `idx` is greater than or equal to its length.[m
[32m+[m[32m    pub fn remove(&mut self, idx: usize) -> Option<T> {[m
[32m+[m[32m        match self.len() {[m
[32m+[m[32m            0 => None,[m
[32m+[m[32m            l if idx >= l => None,[m
[32m+[m[32m            //SAFETY:[m
[32m+[m[32m            //we check both that there are elements in this FatVec,[m
[32m+[m[32m            //and that `idx` is in bounds in the match arms above.[m
[32m+[m[32m            _ => Some(unsafe { self.remove_unchecked(idx) }),[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    ///Removes the element at `idx` from this `FatVec`.[m
[32m+[m[32m    ///SAFETY:[m
[32m+[m[32m    ///Undefined Behavior if `idx` is greater than or equal to the length of this `FatVec`.[m
[32m+[m[32m    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> T {[m
[32m+[m[32m        let r = match idx <= STACK_CAPACITY {[m
[32m+[m[32m            //value is resident on stack[m
[32m+[m[32m            true => {[m
[32m+[m[32m                let len = self.array_len();[m
[32m+[m[32m                let list = &mut self.stack_list;[m
[32m+[m[32m                //SAFETY[m
[32m+[m[32m                //upheld by caller. See function documentation.[m
[32m+[m[32m                let r = unsafe { raw_stack_list::remove!(list, idx, len) };[m
[32m+[m
[32m+[m[32m                //Shift elements from heap to stack, if necessary.[m
[32m+[m[32m                if self.vec.len() > 0 {[m
[32m+[m[32m                    let ref mut list = self.stack_list;[m
[32m+[m
[32m+[m[32m                    let idx = STACK_CAPACITY.saturating_sub(1);[m
[32m+[m
[32m+[m[32m                    let elem = self.vec.remove(0);[m
[32m+[m
[32m+[m[32m                    //SAFETY:[m
[32m+[m[32m                    //STACK_CAPACITY - 1 is always guaranteed to be last element in the RawStackList.[m
[32m+[m[32m                    //Further, we know both that that last element is going to be unoccupied - the prior call to remove guarantees that all elements before it have shifted left -[m
[32m+[m[32m                    //and we also know that the the RawStackList has space for only one element now, because the vec is non empty.[m
[32m+[m[32m                    //[m
[32m+[m[32m                    //The only time the RawStackList will have less than STACK_CAPACITY elements is when no elements have overflowed onto the heap.[m
[32m+[m[32m                    unsafe { raw_stack_list::insert_at!(list, idx, elem) }[m
[32m+[m[32m                }[m
[32m+[m
[32m+[m[32m                r[m
[32m+[m[32m            }[m
[32m+[m[32m            //value is resident on heap[m
[32m+[m[32m            false => {[m
[32m+[m[32m                let vec_idx = idx - STACK_CAPACITY;[m
[32m+[m[32m                self.vec.remove(vec_idx)[m
[32m+[m[32m            }[m
[32m+[m[32m        };[m
[32m+[m[32m        self.len -= 1;[m
[32m+[m[32m        r[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    /// Tries to reserve the minimum capacity for at least `additional`[m
[32m+[m[32m    /// elements to be inserted in the given `Vec<T>`.[m
[32m+[m[32m    /// After calling `reserve`, capacity will be[m
[32m+[m[32m    /// equal to `self.len() + additional` if it returns `Ok(())`.[m
[32m+[m[32m    /// Does nothing if the capacity is already sufficient.[m
[32m+[m[32m    pub fn reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {[m
[32m+[m[32m        //We use try_reserve_exact to keep memory use as compact as possible at the expense of throughput.[m
[32m+[m[32m        self.vec.try_reserve_exact(additional).map_err(|e| e.into())[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    ///Shrinks the heap storage of this `FatVec` to match capacity.[m
[32m+[m[32m    pub fn shrink_to_fit(&mut self) {[m
[32m+[m[32m        self.vec.shrink_to_fit()[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    //***constructors***//[m
     ///Creates a `FatVec` with the provided array as the stack resident elements.[m
     ///The length of the supplied array will become the `STACK_CAPCITY` of the returned `FatVec` *AND* the length of the array.[m
     ///There is no interface to mutate the length without manipulating the elements on the stack.[m
[36m@@ -55,6 +180,25 @@[m [mimpl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {[m
         }[m
     }[m
 [m
[32m+[m[32m    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements without reallocating[m
[32m+[m[32m    ///If `capacity` is less than or equal to `STACK_CAPACITY` the total capacity of this `FatVec` will be equal to `STACK_CAPACITY`.[m
[32m+[m[32m    pub fn with_capacity(capacity: usize) -> Result<Self, TryReserveError> {[m
[32m+[m[32m        let heap_capacity = capacity.saturating_sub(STACK_CAPACITY);[m
[32m+[m
[32m+[m[32m        Self::with_heap_capacity(heap_capacity)[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements *on the heap* without reallocating.[m
[32m+[m[32m    ///Upon return, the total capacity of this `FatVec` will be STACK_CAPACITY + `capacity`[m
[32m+[m[32m    ///use `with_capacity` if you need[m
[32m+[m[32m    pub fn with_heap_capacity(capacity: usize) -> Result<Self, TryReserveError> {[m
[32m+[m[32m        Ok(Self {[m
[32m+[m[32m            stack_list: RawStackList::uninit(),[m
[32m+[m[32m            vec: Vec::try_with_capacity(capacity)?,[m
[32m+[m[32m            len: 0,[m
[32m+[m[32m        })[m
[32m+[m[32m    }[m
[32m+[m
     ///Create a `StackList` from an array with fewer elements than `STACK_CAPACITY`, permitting infallible construction.[m
     ///If `ITEMS` is always guaranteed to be identical to `STACK_CAPACITY`, it's best to use `with_array` instead.[m
     pub fn with_partial_array<const ITEMS: usize>(items: [T; ITEMS]) -> FatVec<T, STACK_CAPACITY>[m
[36m@@ -87,63 +231,24 @@[m [mimpl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {[m
             len: ITEMS,[m
         }[m
     }[m
[32m+[m[32m}[m
 [m
[31m-    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements without reallocating[m
[31m-    ///If `capacity` is less than or equal to `STACK_CAPACITY` the total capacity of this `FatVec` will be equal to `STACK_CAPACITY`.[m
[31m-    pub fn with_capacity(capacity: usize) -> Result<Self, TryReserveError> {[m
[31m-        let heap_capacity = capacity.saturating_sub(STACK_CAPACITY);[m
[31m-[m
[31m-        Self::with_heap_capacity(heap_capacity)[m
[31m-    }[m
[31m-[m
[31m-    ///Creates a new, empty `FatVec` with space to hold at least `capacity` elements *on the heap* without reallocating.[m
[31m-    ///Upon return, the total capacity of this `FatVec` will be STACK_CAPACITY + `capacity`[m
[31m-    ///use `with_capacity` if you need[m
[31m-    pub fn with_heap_capacity(capacity: usize) -> Result<Self, TryReserveError> {[m
[31m-        Ok(Self {[m
[32m+[m[32mimpl<T, const STACK_CAPACITY: usize> List<T> for FatVec<T, STACK_CAPACITY> {[m
[32m+[m[32m    type Error = TryReserveError;[m
[32m+[m[32m    //***constructors***//[m
[32m+[m[32m    fn new() -> Self {[m
[32m+[m[32m        Self {[m
             stack_list: RawStackList::uninit(),[m
[31m-            vec: Vec::try_with_capacity(capacity)?,[m
[32m+[m[32m            vec: Vec::new(),[m
             len: 0,[m
[31m-        })[m
[31m-    }[m
[31m-[m
[31m-    //***methods***[m
[31m-[m
[31m-    pub fn array_len(&self) -> usize {[m
[31m-        match self.len() <= STACK_CAPACITY {[m
[31m-            true => self.len(),[m
[31m-            false => STACK_CAPACITY,[m
         }[m
     }[m
 [m
[31m-    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {[m
[31m-        let len = self.array_len();[m
[31m-        let list = &self.stack_list;[m
[31m-[m
[31m-        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec[m
[31m-        unsafe { raw_stack_list::iter_to!(list, len) }.chain(self.vec.iter())[m
[31m-    }[m
[31m-[m
[31m-    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {[m
[31m-        let len = self.array_len();[m
[31m-        let list = &mut self.stack_list;[m
[31m-[m
[31m-        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec[m
[31m-        unsafe { raw_stack_list::iter_mut_to!(list, len) }.chain(self.vec.iter_mut())[m
[31m-    }[m
[31m-[m
[31m-    #[inline(always)][m
[31m-    ///Returns the number of items in this `FatVec`[m
[31m-    pub const fn len(&self) -> usize {[m
[31m-        self.len[m
[31m-    }[m
[31m-[m
[31m-    ///Returns the maximum number of items this `FatVec` can hold both on the stack and heap without reallocating.[m
[31m-    ///Note this is not the remaing space in the `FatVec`, but a count of all capacity, consumed or not.[m
[31m-    pub fn capacity(&self) -> usize {[m
[32m+[m[32m    fn capacity(&self) -> usize {[m
         self.vec.capacity() + STACK_CAPACITY[m
     }[m
[31m-    pub fn clear(&mut self) {[m
[32m+[m
[32m+[m[32m    fn clear(&mut self) {[m
         match NonZero::new(self.len) {[m
             //SAFETY:[m
             //Ensure that all  elements are dropped. Bounded by array len means this cannot find uninitalized[m
[36m@@ -158,38 +263,33 @@[m [mimpl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {[m
         }[m
     }[m
 [m
[31m-    ///Appends the element to this `FatVec`, returning an error on failure.[m
[31m-    pub fn push(&mut self, value: T) -> Result<(), TryReserveError> {[m
[31m-        //We don't need to check if we're within the bounds of the collection as reserve will do this[m
[31m-        //for us.[m
[31m-        let new_len = self.len.saturating_add(1);[m
[32m+[m[32m    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>[m
[32m+[m[32m    where[m
[32m+[m[32m        T: 'a,[m
[32m+[m[32m    {[m
[32m+[m[32m        let len = self.array_len();[m
[32m+[m[32m        let list = &self.stack_list;[m
 [m
[31m-        match STACK_CAPACITY > self.len() {[m
[31m-            true => {[m
[31m-                let ref mut list = self.stack_list;[m
[31m-                let old_len = self.len;[m
[31m-                unsafe { raw_stack_list::insert_at!(list, old_len, value) }[m
[31m-            }[m
[31m-            false => {[m
[31m-                //call reserve on the vec as necessary to ensure pushing to it doesn't panic.[m
[31m-                if self.vec.capacity() < new_len {[m
[31m-                    self.reserve(1)?;[m
[31m-                }[m
[32m+[m[32m        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec[m
[32m+[m[32m        unsafe { raw_stack_list::iter_to!(list, len) }.chain(self.vec.iter())[m
[32m+[m[32m    }[m
 [m
[31m-                self.vec.push(value);[m
[31m-            }[m
[31m-        }[m
[31m-        self.len = new_len;[m
[31m-        Ok(())[m
[32m+[m[32m    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>[m
[32m+[m[32m    where[m
[32m+[m[32m        T: 'a,[m
[32m+[m[32m    {[m
[32m+[m[32m        let len = self.array_len();[m
[32m+[m[32m        let list = &mut self.stack_list;[m
[32m+[m
[32m+[m[32m        //SAFETY: len is guaranteed to be within the initialized contents of the RawVec[m
[32m+[m[32m        unsafe { raw_stack_list::iter_mut_to!(list, len) }.chain(self.vec.iter_mut())[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn len(&self) -> usize {[m
[32m+[m[32m        self.len[m
     }[m
 [m
[31m-    /// Removes the last element from a `FatVec` and returns it, or [`None`] if the `FatVec` is empty.[m
[31m-    pub fn pop(&mut self) -> Option<T> {[m
[31m-        //we keep this seperate from remove(self.len) as in the[m
[31m-        //stack resident arm we can avoid having to shift the elements on the array to the left[m
[31m-        //as we can just decrement the "stack pointer" and leave the type as is. Drop handling is done by moving[m
[31m-        //the T out of pop. Remove can't do this as it needs to be able to pull elements arbitrarily from within the array[m
[31m-        //meaning it needs to shift left to keep the values contiguous.[m
[32m+[m[32m    fn pop(&mut self) -> Option<T> {[m
         match self.len() {[m
             0 => None,[m
             //resident on stack[m
[36m@@ -210,128 +310,28 @@[m [mimpl<const STACK_CAPACITY: usize, T> FatVec<T, STACK_CAPACITY> {[m
         }[m
     }[m
 [m
[31m-    /// Removes the element at `idx` from this `FatVec` and returns it, or `None` if the `FatVec` is empty or if `idx` is greater than or equal to its length.[m
[31m-    pub fn remove(&mut self, idx: usize) -> Option<T> {[m
[31m-        match self.len() {[m
[31m-            0 => None,[m
[31m-            l if idx >= l => None,[m
[31m-            //SAFETY:[m
[31m-            //we check both that there are elements in this FatVec,[m
[31m-            //and that `idx` is in bounds in the match arms above.[m
[31m-            _ => Some(unsafe { self.remove_unchecked(idx) }),[m
[31m-        }[m
[31m-    }[m
[32m+[m[32m    fn push(&mut self, item: T) -> Result<(), Self::Error> {[m
[32m+[m[32m        //We don't need to check if we're within the bounds of the collection as reserve will do this[m
[32m+[m[32m        //for us.[m
[32m+[m[32m        let new_len = self.len.saturating_add(1);[m
 [m
[31m-    ///Removes the element at `idx` from this `FatVec`.[m
[31m-    ///SAFETY:[m
[31m-    ///Undefined Behavior if `idx` is greater than or equal to the length of this `FatVec`.[m
[31m-    pub unsafe fn remove_unchecked(&mut self, idx: usize) -> T {[m
[31m-        let r = match idx <= STACK_CAPACITY {[m
[31m-            //value is resident on stack[m
[32m+[m[32m        match STACK_CAPACITY > self.len() {[m
             true => {[m
[31m-                let len = self.array_len();[m
[31m-                let list = &mut self.stack_list;[m
[31m-                //SAFETY[m
[31m-                //upheld by caller. See function documentation.[m
[31m-                let r = unsafe { raw_stack_list::remove!(list, idx, len) };[m
[31m-[m
[31m-                //Shift elements from heap to stack, if necessary.[m
[31m-                if self.vec.len() > 0 {[m
[31m-                    let ref mut list = self.stack_list;[m
[31m-[m
[31m-                    let idx = STACK_CAPACITY.saturating_sub(1);[m
[31m-[m
[31m-                    let elem = self.vec.remove(0);[m
[31m-[m
[31m-                    //SAFETY:[m
[31m-                    //STACK_CAPACITY - 1 is always guaranteed to be last element in the RawStackList.[m
[31m-                    //Further, we know both that that last element is going to be unoccupied - the prior call to remove guarantees that all elements before it have shifted left -[m
[31m-                    //and we also know that the the RawStackList has space for only one element now, because the vec is non empty.[m
[31m-                    //[m
[31m-                    //The only time the RawStackList will have less than STACK_CAPACITY elements is when no elements have overflowed onto the heap.[m
[31m-                    unsafe { raw_stack_list::insert_at!(list, idx, elem) }[m
[31m-                }[m
[31m-[m
[31m-                r[m
[32m+[m[32m                let ref mut list = self.stack_list;[m
[32m+[m[32m                let old_len = self.len;[m
[32m+[m[32m                unsafe { raw_stack_list::insert_at!(list, old_len, item) }[m
             }[m
[31m-            //value is resident on heap[m
             false => {[m
[31m-                let vec_idx = idx - STACK_CAPACITY;[m
[31m-                self.vec.remove(vec_idx)[m
[31m-            }[m
[31m-        };[m
[31m-        self.len -= 1;[m
[31m-        r[m
[31m-    }[m
[31m-[m
[31m-    ///Returns a shared reference to the item at the requested, returning `None` if idx is outside the range of the `FatVec`.[m
[31m-    pub fn get(&self, idx: usize) -> Option<&T> {[m
[31m-        if idx >= self.len {[m
[31m-            return None;[m
[31m-        }[m
[31m-[m
[31m-        //SAFETY:[m
[31m-        //the early return above guarantees we do not access[m
[31m-        //out of bounds[m
[31m-        unsafe { Some(self.get_unchecked(idx)) }[m
[31m-    }[m
[31m-[m
[31m-    ///Returns a unique reference to the item at `idx`, returning `None` if idx is outside the range of the `FatVec`.[m
[31m-    pub fn get_mut<'a>(&'a mut self, idx: usize) -> Option<&'a mut T> {[m
[31m-        if idx >= self.len {[m
[31m-            return None;[m
[31m-        }[m
[31m-        //SAFETY:[m
[31m-        //the early return above guarantees we do not access[m
[31m-        //out of bounds[m
[31m-        unsafe { Some(self.get_unchecked_mut(idx)) }[m
[31m-    }[m
[31m-[m
[31m-    ///Returns a shared reference to the item at `idx`.[m
[31m-    ///SAFETY:[m
[31m-    ///UB if idx is >= the length of this `FatVec`.[m
[31m-    pub unsafe fn get_unchecked(&self, idx: usize) -> &T {[m
[31m-        let list = &self.stack_list;[m
[31m-        match STACK_CAPACITY > idx {[m
[31m-            //SAFETY:[m
[31m-            //Because we maintain length seperately of the vec and array, we can rely on IDX not to be out of bounds for[m
[31m-            //either these accesses.[m
[31m-            true => unsafe { raw_stack_list::get!(list, idx) },[m
[31m-            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's[m
[31m-            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.[m
[31m-            false => unsafe { self.vec.get_unchecked(idx - STACK_CAPACITY) },[m
[31m-        }[m
[31m-    }[m
[32m+[m[32m                //call reserve on the vec as necessary to ensure pushing to it doesn't panic.[m
[32m+[m[32m                if self.vec.capacity() < new_len {[m
[32m+[m[32m                    self.reserve(1)?;[m
[32m+[m[32m                }[m
 [m
[31m-    ///Returns a unique reference to the item at `idx`.[m
[31m-    ///SAFETY:[m
[31m-    ///UB if idx is >= the length of this `FatVec`.[m
[31m-    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut T {[m
[31m-        let list = &mut self.stack_list;[m
[31m-        match STACK_CAPACITY > idx {[m
[31m-            //SAFETY:[m
[31m-            //Because we maintain length seperately from the vec and array, we can rely on IDX not to be out of bounds for[m
[31m-            //either these accesses.[m
[31m-            true => unsafe { raw_stack_list::get_mut!(list, idx) },[m
[31m-            //subtract as the first element of vec is 0, but in the whole `FatVec`, it's[m
[31m-            //always STACK_CAPACITY + idx. The subtraction accounts for this for this.[m
[31m-            false => unsafe { self.vec.get_unchecked_mut(idx - STACK_CAPACITY) },[m
[32m+[m[32m                self.vec.push(item);[m
[32m+[m[32m            }[m
         }[m
[31m-    }[m
[31m-[m
[31m-    /// Tries to reserve the minimum capacity for at least `additional`[m
[31m-    /// elements to be inserted in the given `Vec<T>`.[m
[31m-    /// After calling `reserve`, capacity will be[m
[31m-    /// equal to `self.len() + additional` if it returns `Ok(())`.[m
[31m-    /// Does nothing if the capacity is already sufficient.[m
[31m-    pub fn reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {[m
[31m-        //We use try_reserve_exact to keep memory use as compact as possible at the expense of throughput.[m
[31m-        self.vec.try_reserve_exact(additional).map_err(|e| e.into())[m
[31m-    }[m
[31m-[m
[31m-    ///Shrinks the heap storage of this `FatVec` to match capacity.[m
[31m-    pub fn shrink_to_fit(&mut self) {[m
[31m-        self.vec.shrink_to_fit()[m
[32m+[m[32m        self.len = new_len;[m
[32m+[m[32m        Ok(())[m
     }[m
 }[m
 [m
[1mdiff --git a/src/fat_vec/serde/mod.rs b/src/fat_vec/serde/mod.rs[m
[1mindex e3b3be6..ec7f535 100644[m
[1m--- a/src/fat_vec/serde/mod.rs[m
[1m+++ b/src/fat_vec/serde/mod.rs[m
[36m@@ -1,4 +1,6 @@[m
[31m-use serde::{ser::SerializeSeq, Serialize};[m
[32m+[m[32muse serde::{Serialize, ser::SerializeSeq};[m
[32m+[m
[32m+[m[32muse crate::list::List;[m
 [m
 use super::FatVec;[m
 [m
[1mdiff --git a/src/fat_vec/test.rs b/src/fat_vec/test.rs[m
[1mindex c624ab6..e9675de 100644[m
[1m--- a/src/fat_vec/test.rs[m
[1m+++ b/src/fat_vec/test.rs[m
[36m@@ -1,6 +1,6 @@[m
 use std::intrinsics::transmute_unchecked;[m
 [m
[31m-use crate::stack_list::RawStackList;[m
[32m+[m[32muse crate::{list::List, stack_list::RawStackList};[m
 [m
 use super::FatVec;[m
 [m
[1mdiff --git a/src/fat_vec/trybuild/empty_items.stderr b/src/fat_vec/trybuild/empty_items.stderr[m
[1mindex 37fb2e2..3a08bd2 100644[m
[1m--- a/src/fat_vec/trybuild/empty_items.stderr[m
[1m+++ b/src/fat_vec/trybuild/empty_items.stderr[m
[36m@@ -19,6 +19,7 @@[m [mnote: required by a bound in `FatVec::<T, STACK_CAPACITY>::with_partial_array`[m
   |       pub fn with_partial_array<const ITEMS: usize>(items: [T; ITEMS]) -> FatVec<T, STACK_CAPACITY>[m
   |              ------------------ required by a bound in this associated function[m
   |       where[m
[31m-  | /         [(); STACK_CAPACITY.checked_sub(ITEMS)[m
[32m+[m[32m  | /         [(); STACK_CAPACITY[m
[32m+[m[32m  | |             .checked_sub(ITEMS)[m
   | |             .expect("The length of the items must be less than or equal to STACK_CAPACITY.")]:,[m
   | |_____________________________________________________________________________________________^ required by this bound in `FatVec::<T, STACK_CAPACITY>::with_partial_array`[m
[1mdiff --git a/src/fat_vec/trybuild/items_gt_stack_capacity.stderr b/src/fat_vec/trybuild/items_gt_stack_capacity.stderr[m
[1mindex cb8216a..c936305 100644[m
[1m--- a/src/fat_vec/trybuild/items_gt_stack_capacity.stderr[m
[1m+++ b/src/fat_vec/trybuild/items_gt_stack_capacity.stderr[m
[36m@@ -1,8 +1,9 @@[m
 error[E0080]: evaluation panicked: The length of the items must be less than or equal to STACK_CAPACITY.[m
  --> src/fat_vec/mod.rs[m
   |[m
[31m-  |           [(); STACK_CAPACITY.checked_sub(ITEMS)[m
[32m+[m[32m  |           [(); STACK_CAPACITY[m
   |  ______________^[m
[32m+[m[32m  | |             .checked_sub(ITEMS)[m
   | |             .expect("The length of the items must be less than or equal to STACK_CAPACITY.")]:,[m
   | |____________________________________________________________________________________________^ evaluation of `linear_collections::FatVec::<u8, 10>::with_partial_array::<11>::{constant#0}` failed here[m
 [m
[36m@@ -18,6 +19,7 @@[m [mnote: required by a bound in `FatVec::<T, STACK_CAPACITY>::with_partial_array`[m
   |       pub fn with_partial_array<const ITEMS: usize>(items: [T; ITEMS]) -> FatVec<T, STACK_CAPACITY>[m
   |              ------------------ required by a bound in this associated function[m
   |       where[m
[31m-  | /         [(); STACK_CAPACITY.checked_sub(ITEMS)[m
[32m+[m[32m  | /         [(); STACK_CAPACITY[m
[32m+[m[32m  | |             .checked_sub(ITEMS)[m
   | |             .expect("The length of the items must be less than or equal to STACK_CAPACITY.")]:,[m
   | |_____________________________________________________________________________________________^ required by this bound in `FatVec::<T, STACK_CAPACITY>::with_partial_array`[m
[1mdiff --git a/src/list/mod.rs b/src/list/mod.rs[m
[1mindex 2495535..5fe3415 100644[m
[1m--- a/src/list/mod.rs[m
[1m+++ b/src/list/mod.rs[m
[36m@@ -15,15 +15,12 @@[m [mpub trait List<T>: Sized {[m
     fn capacity(&self) -> usize;[m
 [m
     ///Calls `drop` on all elements of this `List`, leaving it empty with a length of 0.[m
[31m-    fn clear(&mut self) {[m
[31m-        for _ in 0..self.len() {[m
[31m-            let item = self.pop();[m
[31m-            drop(item);[m
[31m-        }[m
[31m-    }[m
[32m+[m[32m    fn clear(&mut self);[m
 [m
[31m-    ///Returns an iterator that, when exhausted, leaves this `List` empty.[m
[31m-    fn drain<'a>(self) -> impl Iterator<Item = T>;[m
[32m+[m[32m    ///Returns true if this `List`'s length is 0. False otherwise.[m
[32m+[m[32m    fn is_empty(&self) -> bool {[m
[32m+[m[32m        self.len() == 0[m
[32m+[m[32m    }[m
 [m
     ///Returns an iterator of references of the elements of this `List`.[m
     fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>[m
[36m@@ -47,5 +44,5 @@[m [mpub trait List<T>: Sized {[m
     }[m
 [m
     ///Appends `item` to the end of this `List`.[m
[31m-    fn push(item: T) -> Result<(), Self::Error>;[m
[32m+[m[32m    fn push(&mut self, item: T) -> Result<(), Self::Error>;[m
 }[m
[1mdiff --git a/src/stack_list/error.rs b/src/stack_list/error.rs[m
[1mindex c620a32..2761fc9 100644[m
[1m--- a/src/stack_list/error.rs[m
[1m+++ b/src/stack_list/error.rs[m
[36m@@ -1,16 +1,16 @@[m
 use std::{error::Error, fmt::Display};[m
 [m
 #[derive(Clone, Copy, Debug)][m
[31m-pub enum PushError {[m
[32m+[m[32mpub enum StackListError {[m
     WouldExceedCapacity,[m
 }[m
 [m
[31m-impl Display for PushError {[m
[32m+[m[32mimpl Display for StackListError {[m
     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {[m
         match self {[m
[31m-            PushError::WouldExceedCapacity => write!(f, "push would exceed capacity"),[m
[32m+[m[32m            StackListError::WouldExceedCapacity => write!(f, "push would exceed capacity"),[m
         }[m
     }[m
 }[m
 [m
[31m-impl Error for PushError {}[m
[32m+[m[32mimpl Error for StackListError {}[m
[1mdiff --git a/src/stack_list/map.rs b/src/stack_list/map.rs[m
[1mindex fc4b6ec..d755691 100644[m
[1m--- a/src/stack_list/map.rs[m
[1m+++ b/src/stack_list/map.rs[m
[36m@@ -1,7 +1,8 @@[m
 use crate::{[m
     Map,[m
[32m+[m[32m    list::List,[m
     map::MapIterMut,[m
[31m-    stack_list::{StackList, error::PushError},[m
[32m+[m[32m    stack_list::{StackList, error::StackListError},[m
 };[m
 [m
 ///A map backed by a `StackList`[m
[36m@@ -19,7 +20,7 @@[m [mimpl<K: Eq, V, const CAPACITY: usize> StackMap<K, V, CAPACITY> {[m
 [m
 impl<K: Eq, V, const CAPACITY: usize> Map<K, V> for StackMap<K, V, CAPACITY> {[m
     type Backing = StackList<(K, V), CAPACITY>;[m
[31m-    type InsertionError = PushError;[m
[32m+[m[32m    type InsertionError = StackListError;[m
 [m
     fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::InsertionError> {[m
         let mut iter = self.stack_list.iter_mut();[m
[1mdiff --git a/src/stack_list/mod.rs b/src/stack_list/mod.rs[m
[1mindex 999dc30..11a7227 100644[m
[1m--- a/src/stack_list/mod.rs[m
[1m+++ b/src/stack_list/mod.rs[m
[36m@@ -10,7 +10,7 @@[m [mmod test;[m
 #[cfg(kani)][m
 mod verification;[m
 [m
[31m-use error::PushError;[m
[32m+[m[32muse error::StackListError;[m
 pub use raw_stack_list::RawStackList;[m
 use std::{[m
     hash::Hash,[m
[36m@@ -18,6 +18,8 @@[m [muse std::{[m
     ops::{Deref, DerefMut},[m
 };[m
 [m
[32m+[m[32muse crate::list::List;[m
[32m+[m
 #[derive(Default, Debug)][m
 ///TODO: is this unsound? can kani::Arbitrary produce `MaybeUninit<T>` which are both `uninit` and invalid? Or[m
 /// can `Arbitrary` only produce valid values of it?[m
[36m@@ -28,17 +30,7 @@[m [mpub struct StackList<T, const CAPACITY: usize> {[m
     length: usize,[m
 }[m
 [m
[31m-impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {}[m
[31m-[m
 impl<T, const CAPACITY: usize> StackList<T, CAPACITY> {[m
[31m-    ///Creates a new, empty `StackList`.[m
[31m-    pub fn new() -> Self {[m
[31m-        Self {[m
[31m-            raw: RawStackList::uninit(),[m
[31m-            length: 0,[m
[31m-        }[m
[31m-    }[m
[31m-[m
     ///TODO: add verification and description[m
     pub fn as_slice(&self) -> &[T] {[m
         let pointer = self.raw.as_slice().as_ptr() as *const T;[m
[36m@@ -62,48 +54,11 @@[m [mimpl<T, const CAPACITY: usize> StackList<T, CAPACITY> {[m
         unsafe { std::slice::from_raw_parts_mut(pointer, self.length) }[m
     }[m
 [m
[31m-    ///Calls `drop` on all elements in this list, in place.[m
[31m-    pub fn clear(&mut self) {[m
[31m-        match NonZero::new(self.length) {[m
[31m-            Some(len) => {[m
[31m-                let ref mut list = self.raw;[m
[31m-                //SAFETY:[m
[31m-                //bound by length so will not go out of bounds or into uninit memory[m
[31m-                unsafe { raw_stack_list::clear_to!(list, len) };[m
[31m-                self.length = 0;[m
[31m-            }[m
[31m-[m
[31m-            None => return,[m
[31m-        }[m
[31m-    }[m
[31m-[m
     ///Returns true if the store is empty and false otherwise.[m
     pub fn is_empty(&self) -> bool {[m
         self.length == 0[m
     }[m
 [m
[31m-    ///Returns an iterator over the elements of this `StackList`.[m
[31m-    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {[m
[31m-        let raw = &self.raw;[m
[31m-        let length = self.length;[m
[31m-        //SAFETY:[m
[31m-        //bound by length so will not go out of bounds or into uninit memory[m
[31m-        unsafe { raw_stack_list::iter_to!(raw, length) }[m
[31m-    }[m
[31m-    ///Returns an iterator over the elements of this `StackList where each element is mutable.[m
[31m-    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {[m
[31m-        let raw = &mut self.raw;[m
[31m-        let length = self.length;[m
[31m-        //SAFETY:[m
[31m-        //bound by length so will not go out of bounds or into uninit memory[m
[31m-        unsafe { raw_stack_list::iter_mut_to!(raw, length) }[m
[31m-    }[m
[31m-[m
[31m-    ///Returns the number of items in this `StackList`.[m
[31m-    pub const fn len(&self) -> usize {[m
[31m-        self.length[m
[31m-    }[m
[31m-[m
     ////Creates a StaticList from an array, with the StaticList assuming the length of the array as its Capacity.[m
     pub fn from_array(array: [T; CAPACITY]) -> Self {[m
         Self {[m
[36m@@ -112,27 +67,6 @@[m [mimpl<T, const CAPACITY: usize> StackList<T, CAPACITY> {[m
         }[m
     }[m
 [m
[31m-    ///Removes the last element in this `StackList`, returning `None` if this list is empty.[m
[31m-    pub fn pop(&mut self) -> Option<T> {[m
[31m-        self.length.checked_sub(1).and_then(|new| self.remove(new))[m
[31m-    }[m
[31m-    //self.remove(self.length)[m
[31m-[m
[31m-    pub fn push(&mut self, value: T) -> Result<(), PushError> {[m
[31m-        match self.length.checked_add(1) {[m
[31m-            Some(new) => {[m
[31m-                let raw = &mut self.raw;[m
[31m-                let len = self.length;[m
[31m-                //SAFETY:[m
[31m-                //Guaranteed to be in bounds as length is never out of bounds.[m
[31m-                unsafe { raw_stack_list::insert_at!(raw, len, value) };[m
[31m-                self.length = new;[m
[31m-                Ok(())[m
[31m-            }[m
[31m-            None => Err(PushError::WouldExceedCapacity),[m
[31m-        }[m
[31m-    }[m
[31m-[m
     ///Removes the element at `index` from this `StackList`.[m
     pub fn remove(&mut self, index: usize) -> Option<T> {[m
         //check if index is in bounds[m
[36m@@ -176,6 +110,79 @@[m [mimpl<T, const CAPACITY: usize> StackList<T, CAPACITY> {[m
     }[m
 }[m
 [m
[32m+[m[32mimpl<T, const CAPACITY: usize> List<T> for StackList<T, CAPACITY> {[m
[32m+[m[32m    type Error = StackListError;[m
[32m+[m
[32m+[m[32m    fn new() -> Self {[m
[32m+[m[32m        Self {[m
[32m+[m[32m            raw: RawStackList::uninit(),[m
[32m+[m[32m            length: 0,[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m[32m    fn capacity(&self) -> usize {[m
[32m+[m[32m        CAPACITY[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn clear(&mut self) {[m
[32m+[m[32m        match NonZero::new(self.length) {[m
[32m+[m[32m            Some(len) => {[m
[32m+[m[32m                let ref mut list = self.raw;[m
[32m+[m[32m                //SAFETY:[m
[32m+[m[32m                //bound by length so will not go out of bounds or into uninit memory[m
[32m+[m[32m                unsafe { raw_stack_list::clear_to!(list, len) };[m
[32m+[m[32m                self.length = 0;[m
[32m+[m[32m            }[m
[32m+[m
[32m+[m[32m            None => return,[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>[m
[32m+[m[32m    where[m
[32m+[m[32m        T: 'a,[m
[32m+[m[32m    {[m
[32m+[m[32m        let raw = &self.raw;[m
[32m+[m[32m        let length = self.length;[m
[32m+[m[32m        //SAFETY:[m
[32m+[m[32m        //bound by length so will not go out of bounds or into uninit memory[m
[32m+[m[32m        unsafe { raw_stack_list::iter_to!(raw, length) }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>[m
[32m+[m[32m    where[m
[32m+[m[32m        T: 'a,[m
[32m+[m[32m    {[m
[32m+[m[32m        let raw = &mut self.raw;[m
[32m+[m[32m        let length = self.length;[m
[32m+[m[32m        //SAFETY:[m
[32m+[m[32m        //bound by length so will not go out of bounds or into uninit memory[m
[32m+[m[32m        unsafe { raw_stack_list::iter_mut_to!(raw, length) }[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn len(&self) -> usize {[m
[32m+[m[32m        self.length[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn pop(&mut self) -> Option<T> {[m
[32m+[m[32m        self.length.checked_sub(1).and_then(|new| self.remove(new))[m
[32m+[m[32m    }[m
[32m+[m
[32m+[m[32m    fn push(&mut self, item: T) -> Result<(), Self::Error> {[m
[32m+[m[32m        match self.length.checked_add(1) {[m
[32m+[m[32m            Some(new) => {[m
[32m+[m[32m                let raw = &mut self.raw;[m
[32m+[m[32m                let len = self.length;[m
[32m+[m[32m                //SAFETY:[m
[32m+[m[32m                //Guaranteed to be in bounds as length is never out of bounds.[m
[32m+[m[32m                unsafe { raw_stack_list::insert_at!(raw, len, item) };[m
[32m+[m[32m                self.length = new;[m
[32m+[m[32m                Ok(())[m
[32m+[m[32m            }[m
[32m+[m[32m            None => Err(StackListError::WouldExceedCapacity),[m
[32m+[m[32m        }[m
[32m+[m[32m    }[m
[32m+[m[32m}[m
[32m+[m
 impl<const CAPACITY: usize, T: PartialEq> PartialEq for StackList<T, CAPACITY> {[m
     fn eq(&self, other: &Self) -> bool {[m
         //just want to explicitly evaluate this first as it's much cheaper.[m
[1mdiff --git a/src/stack_list/raw_stack_list/mod.rs b/src/stack_list/raw_stack_list/mod.rs[m
[1mindex d51e20f..257096c 100644[m
[1m--- a/src/stack_list/raw_stack_list/mod.rs[m
[1m+++ b/src/stack_list/raw_stack_list/mod.rs[m
[36m@@ -196,7 +196,7 @@[m [mimpl<T, const CAPACITY: usize> RawStackList<T, CAPACITY> {[m
     }[m
 }[m
 [m
[31m-use crate as linear_collections;[m
[32m+[m[32muse crate::{self as linear_collections};[m
 [m
 ///`clear_to!(list: ident, limit: ident OR literal)`[m
 ///[m
[1mdiff --git a/src/stack_list/test.rs b/src/stack_list/test.rs[m
[1mindex 6c486e4..f112f5e 100644[m
[1m--- a/src/stack_list/test.rs[m
[1m+++ b/src/stack_list/test.rs[m
[36m@@ -1,4 +1,4 @@[m
[31m-use crate::verification_utils::Dropper;[m
[32m+[m[32muse crate::{list::List, verification_utils::Dropper};[m
 [m
 use super::{RawStackList, StackList};[m
 [m
[1mdiff --git a/src/stack_list/verification.rs b/src/stack_list/verification.rs[m
[1mindex e078ae0..df8b48f 100644[m
[1m--- a/src/stack_list/verification.rs[m
[1m+++ b/src/stack_list/verification.rs[m
[36m@@ -1,3 +1,4 @@[m
[32m+[m[32muse crate::list::List;[m
 use crate::stack_list::{StackList, raw_stack_list::RawStackList};[m
 use crate::verification_utils::Dropper;[m
 use std::mem::MaybeUninit;[m
[36m@@ -29,16 +30,6 @@[m [mfn clear() {[m
     assert_eq!(list.length, 0);[m
 }[m
 [m
[31m-#[proof][m
[31m-fn is_empty() {[m
[31m-    let list: StackList<u8, 5> = kani::any();[m
[31m-[m
[31m-    match list.len() == 0 {[m
[31m-        true => assert!(list.is_empty()),[m
[31m-        false => assert!(!list.is_empty()),[m
[31m-    }[m
[31m-}[m
[31m-[m
 #[proof][m
 fn len() {[m
     let list: StackList<u8, 5> = kani::any();[m
[36m@@ -108,3 +99,9 @@[m [mfn remove() {[m
         i += 1;[m
     }[m
 }[m
[32m+[m
[32m+[m[32m#[proof][m
[32m+[m[32mfn is_empty() {[m
[32m+[m[32m    let list: StackList<u8, 10> = kani::any();[m
[32m+[m[32m    assert_eq!(list.len() == 0, list.is_empty());[m
[32m+[m[32m}[m
