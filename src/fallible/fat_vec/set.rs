use std::collections::TryReserveError;

use crate::fallible::{FallibleLinearMap, FallibleLinearSet};

use super::map::FatMap;

#[derive(Debug, PartialEq, Eq)]
///A set type backed by a FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining items overflow onto the heap.
pub struct FatSet<T: Eq, const STACK_CAPACITY: usize> {
    map: FatMap<T, (), STACK_CAPACITY>,
}

impl<T: Eq, const STACK_CAPACITY: usize> FatSet<T, STACK_CAPACITY> {
    pub fn new() -> Self {
        Self { map: FatMap::new() }
    }

    pub fn with_heap_capacity(capacity: usize) -> Self {
        Self {
            map: FatMap::with_heap_capacity(capacity),
        }
    }

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
