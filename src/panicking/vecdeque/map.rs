use std::collections::VecDeque;

use crate::{panicking::PanickingLinearMap, MapIterMut};

///A map type backed by a VecDeque. Useful for small collections whose size can change.
pub struct DequeMap<K: Eq, V: Sized + PartialEq> {
    vecdeque: VecDeque<(K, V)>,
}
impl<K: Eq, V: Sized + PartialEq> DequeMap<K, V> {
    ///Creates a new, empty VecDequeMap.
    ///Calls Vec::new() internally.
    pub fn new() -> DequeMap<K, V> {
        DequeMap {
            vecdeque: VecDeque::new(),
        }
    }

    ///**Please only use this method to create maps at compile time if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new VecDequeMap from the
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical key beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_vecdeque_unchecked(vecdeque: VecDeque<(K, V)>) -> DequeMap<K, V> {
        DequeMap { vecdeque }
    }

    ///Creates a new, empty VecDequeMap with capacity set to the provide value.
    ///Calls Vec::with_capacity() internally.
    pub fn with_capacity(capacity: usize) -> DequeMap<K, V> {
        DequeMap {
            vecdeque: VecDeque::with_capacity(capacity),
        }
    }
}

impl<K: Eq, V: Sized + PartialEq> PanickingLinearMap<K, V> for DequeMap<K, V> {
    type Backing = VecDeque<(K, V)>;

    ///Inserts the provided value into the VecDequeMap. If the provided key is
    ///found it will update the value. and return the old value. If not, this will allocate for a new key value pair.    
    fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.vecdeque.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => Some(std::mem::replace(v, value)),
            None => {
                self.vecdeque.push_back((key, value));
                None
            }
        }
    }
    fn into_inner(self) -> Self::Backing {
        self.vecdeque
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.vecdeque.iter()
    }

    fn len(&self) -> usize {
        self.vecdeque.len()
    }

    ///Tries to remove the entry associated with the given key, returning None if it is not found.
    fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .vecdeque
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        self.vecdeque.remove(idx)
    }
}

impl<K: Eq, V: Sized + PartialEq> MapIterMut<K, V> for DequeMap<K, V> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.vecdeque.iter_mut()
    }
}
