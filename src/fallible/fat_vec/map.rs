use crate::LinearMap;

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

    fn into_inner(self) -> Self::Backing {
        self.fat_vec
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.fat_vec.iter()
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.fat_vec.iter_mut()
    }
}
