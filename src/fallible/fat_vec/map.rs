use crate::{AsMutSlice, LinearMap};

use super::FatVec;

#[derive(Debug, PartialEq, Eq)]
///A map type backed by an FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining overflow onto the heap.
pub struct FatMap<K, V, const STACK_CAPACITY: usize> {
    fat_vec: FatVec<(K, V), STACK_CAPACITY>,
}

impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> LinearMap<K, V>
    for FatMap<K, V, STACK_CAPACITY>
{
    type Backing = FatVec<(K, V), STACK_CAPACITY>;
    fn as_slice(&self) -> &[(K, V)] {
        &self.fat_vec.as_slice()

        unimplemented!()
    }

    fn into_inner(self) -> Self::Backing {
        self.fat_vec
    }
}

impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> AsMutSlice<K, V>
    for FatMap<K, V, STACK_CAPACITY>
{
    fn as_mut_slice(&mut self) -> &mut [(K, V)] {
        unimplemented!()
        //&mut self.fat_vec
    }
}
