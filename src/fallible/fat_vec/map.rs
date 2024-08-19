use crate::panicking::InfallibleLinearMap;
use std::collections::TryReserveError;

use super::FatVec;

#[derive(Debug, PartialEq, Eq)]
///A map type backed by an FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining overflow onto the heap.
pub struct FatMap<K, V, const STACK_CAPACITY: usize> {
    fatvec: FatVec<(K, V), STACK_CAPACITY>,
}

impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> FatMap<K, V, STACK_CAPACITY> {}

/*
impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> InfallibleLinearMap<K, V>
    for FatMap<K, V, STACK_CAPACITY>
{
    type Backing = FatVec<(K, V), STACK_CAPACITY>;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, TryReserveError> {
        let mut iter = self.fatvec.iter_mut();
        match iter.find(|(k, _)| *k == key) {
            Some((_, v)) => Ok(Some(std::mem::replace(v, value))),
            None => {
                //need to manually drop because the Result gets created as a temporary (?)
                drop(iter);
                self.fatvec.push((key, value))?;
                Ok(None)
            }
        }
    }
    fn into_inner(self) -> Self::Backing {
        self.fatvec
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.fatvec.iter()
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.fatvec.iter_mut()
    }

    fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .fatvec
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        Some(self.fatvec.remove(idx))
    }
}*/
