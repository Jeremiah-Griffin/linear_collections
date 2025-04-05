use crate::{panicking::PanickingLinearMap, MapIterMut};

use super::FatVec;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
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

    //Pending with_array workaround

    /*
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
    }*/
    ///Creates a new, empty `FatMap` with space to hold at least `capacity` elements without reallocating.
    ///Upon return, this `FatMap` will be able to hold `STACK_CAPACITY + `capacity` elements without
    ///re-allocating.
    pub fn with_heap_capacity(capacity: usize) -> Self {
        Self {
            fatvec: FatVec::with_heap_capacity(capacity),
        }
    }
}

impl<K: Eq, V, const STACK_CAPACITY: usize> PanickingLinearMap<K, V>
    for FatMap<K, V, STACK_CAPACITY>
{
    type Backing = FatVec<(K, V), STACK_CAPACITY>;

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut iter = self.fatvec.iter_mut();
        match iter.find(|(k, _)| *k == key) {
            Some((_, v)) => Some(std::mem::replace(v, value)),
            None => {
                //need to manually drop because the Result gets created as a temporary (?)
                drop(iter);
                self.fatvec.push((key, value));
                None
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
        crate::serde::panicking::serialize_panicking_map(self, serializer)
    }
}
