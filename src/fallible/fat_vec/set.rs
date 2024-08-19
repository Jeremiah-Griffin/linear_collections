use crate::panicking::InfallibleLinearSet;

use super::map::FatMap;

#[derive(Debug, PartialEq, Eq)]
///A set type backed by a FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining items overflow onto the heap.
pub struct FatSet<T: Eq, const STACK_CAPACITY: usize> {
    map: FatMap<T, (), STACK_CAPACITY>,
}
/*
impl<T: Eq, const STACK_CAPACITY: usize> LinearSet<T> for FatSet<T, STACK_CAPACITY> {
    type BACKING = FatMap<T, (), STACK_CAPACITY>;

    fn map(&self) -> &Self::BACKING {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::BACKING {
        &mut self.map
    }

    fn insert(&mut self, value: T) -> bool {
        self.map.insert(T, ()).is_none()
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        todo!()
    }
}*/
