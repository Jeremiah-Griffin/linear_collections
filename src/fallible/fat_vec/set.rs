use crate::fallible::FallibleLinearSet;

use super::map::FatMap;

#[derive(Debug, PartialEq, Eq, Hash)]
///A set type backed by a FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining items overflow onto the heap.
pub struct FatSet<T: Eq, const STACK_CAPACITY: usize> {
    map: FatMap<T, (), STACK_CAPACITY>,
}

impl<T: Eq, const STACK_CAPACITY: usize> FatSet<T, STACK_CAPACITY> {
    ///Creates a new, empty `FatSet`. Without allocating on the heap.
    ///This can contain up to `STACK_CAPACITY` elements without performing any
    ///heap allocations.        
    pub fn new() -> Self {
        Self { map: FatMap::new() }
    }

    ///Creates a new, empty `FatSet` with space to hold at least `capacity` elements without reallocating.
    ///Upon return, this `FatSet` will be able to hold `STACK_CAPACITY + `capacity` elements without
    ///re-allocating.
    pub fn with_heap_capacity(capacity: usize) -> Self {
        Self {
            map: FatMap::with_heap_capacity(capacity),
        }
    }

    ///**Please only use this method to create maps at compile time if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new FatVecSet from the
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical value beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_map_unchecked(map: FatMap<T, (), STACK_CAPACITY>) -> Self {
        Self { map }
    }
}

impl<T: Eq, const STACK_CAPACITY: usize> FallibleLinearSet<T> for FatSet<T, STACK_CAPACITY> {
    type BACKING = FatMap<T, (), STACK_CAPACITY>;

    fn map(&self) -> &Self::BACKING {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::BACKING {
        &mut self.map
    }
}
