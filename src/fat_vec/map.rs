use crate::{FallibleLinearMap, MapIterMut};
use std::collections::TryReserveError;

use super::FatVec;

#[derive(Eq, PartialEq, Debug, Hash)]
///A map type backed by an FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining overflow onto the heap.
pub struct FatMap<K: Eq, V, const STACK_CAPACITY: usize> {
    fatvec: FatVec<(K, V), STACK_CAPACITY>,
}

impl<K: Eq, V, const STACK_CAPACITY: usize> FatMap<K, V, STACK_CAPACITY> {
    ///Creates a new, empty `FatMap`. Without allocating on the heap.
    ///This can contain up to `STACK_CAPACITY` elements without performing any
    ///heap allocations.    
    pub fn new() -> Self {
        Self {
            fatvec: FatVec::new(),
        }
    }

    ///Creates a `FatMap` with the provided array as the stack resident elements.
    ///The length of the supplied array will become the `STACK_CAPCITY` of the returned `FatVec` *AND* the length of the array.
    ///There is no interface to mutate the length without manipulating the elements on the stack.
    ///
    ///
    ///Does not allocate to the heap.
    ///SAFETY: improper use of this method - initializing with duplicate values - will NOT create memory unsafety, but will result in every
    ///identical value beyond the first never getting accessed as LinearMaps short circuit on the first match.
    pub unsafe fn with_array(array: [(K, V); STACK_CAPACITY]) -> Self {
        Self {
            fatvec: FatVec::with_array(array),
        }
    }

    ///**Please only use this method to create maps at compile time if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new FatVecMap from the
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical key beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_fatvec_unchecked(fatvec: FatVec<(K, V), STACK_CAPACITY>) -> Self {
        Self { fatvec }
    }

    ///Creates a new, empty `FatMap` with space to hold at least `capacity` elements without reallocating
    ///If `capacity` is less than or equal to `STACK_CAPACITY` the total capacity of this `FatVec` will be equal to `STACK_CAPACITY`.
    pub fn with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        FatVec::with_capacity(capacity).map(|fatvec| FatMap { fatvec })
    }

    ///Creates a new, empty `FatMap` with space to hold at least `capacity` elements without reallocating.
    ///Upon return, this `FatMap` will be able to hold `STACK_CAPACITY + `capacity` elements without
    ///re-allocating.
    pub fn with_heap_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        FatVec::with_heap_capacity(capacity).map(|fatvec| FatMap { fatvec })
    }
}

impl<K: Eq, V, const STACK_CAPACITY: usize> FallibleLinearMap<K, V>
    for FatMap<K, V, STACK_CAPACITY>
{
    type Backing = FatVec<(K, V), STACK_CAPACITY>;
    type InsertionError = TryReserveError;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::InsertionError> {
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

    fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .fatvec
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        self.fatvec.remove(idx)
    }

    fn len(&self) -> usize {
        self.fatvec.len()
    }
}

impl<K: Eq, V, const STACK_CAPACITY: usize> MapIterMut<K, V> for FatMap<K, V, STACK_CAPACITY> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.fatvec.iter_mut()
    }
}
#[cfg(feature = "serde")]
impl<
        'a,
        K: Eq + serde::Serialize,
        V: PartialEq + serde::Serialize,
        const STACK_CAPACITY: usize,
    > serde::Serialize for FatMap<K, V, STACK_CAPACITY>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::serde::serialize_fallible_map(self, serializer)
    }
}

/*
pub struct IntoIterator<T, const STACK_CAPACITY: usize> {
    fv: FatVec<MaybeUninit<T>, STACK_CAPACITY>,
    current: usize,
}

impl<T, const STACK_CAPACITY: usize> Iterator for IntoIterator {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.fv.len() {
            len if self.current > len => None,
            len if len == 0 => None,
            _ => {
                let res = self.fv.get(self.current)
            }
        }
    }
}*/
