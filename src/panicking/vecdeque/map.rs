//TODO: this needs to be behind a nightly flag so we can use the CONCAT trait.
//how expensive is concat? If we used an iterator that would 1) allocate for both vecmap and arraymap
//which would be wasteful for vec and array map just to have compatibility with the two-slice api for VecDeque.
//concat instead allows us to share the same trait while foisting any potential cost to vecdeque alone.
//should document the concat bound and see what doesnt implement it.
use std::collections::VecDeque;

use crate::{AsMutSlice, LinearMap};
///A map type backed by a VecDeque. Useful for small collections whose size can change.
pub struct VecDequeMap<K: Eq, V: Sized + PartialEq> {
    vecdeque: VecDeque<(K, V)>,
}
impl<K: Eq, V: Sized + PartialEq> VecDequeMap<K, V> {
    ///Creates a new, empty VecDequeMap.
    ///Calls Vec::new() internally.
    pub fn new() -> VecDequeMap<K, V> {
        VecDequeMap {
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
    pub const unsafe fn from_vecdeque_unchecked(vecdeque: VecDeque<(K, V)>) -> VecDequeMap<K, V> {
        VecDequeMap { vecdeque }
    }

    ///Creates a new, empty VecDequeMap with capacity set to the provide value.
    ///Calls Vec::with_capacity() internally.
    pub fn with_capacity(capacity: usize) -> VecDequeMap<K, V> {
        VecDequeMap {
            vecdeque: VecDeque::with_capacity(capacity),
        }
    }
    ///Tries to remove the value associated with the given key, returning None if it is not found.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.remove_entry(key).map(|(_, v)| v)
    }

    ///Tries to remove the entry associated with the given key, returning None if it is not found.
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .vecdeque
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        self.vecdeque.remove(idx)
    }

    ///Inserts the provided value into the VecDequeMap. If the provided key is
    ///found it will update the value. and return the old value. If not, this will allocate for a new key value pair.    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.vecdeque.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => Some(std::mem::replace(v, value)),
            None => {
                self.vecdeque.push_back((key, value));
                None
            }
        }
    }
}

impl<K: Eq, V: Sized + PartialEq> LinearMap<K, V> for VecDequeMap<K, V> {
    type Backing = VecDeque<(K, V)>;
    fn as_slice(&self) -> &[(K, V)] {
        self.vecdeque.as_slices().concat()
    }

    fn into_inner(self) -> Self::Backing {
        self.vecdeque
    }
}

impl<K: Eq, V: Sized + PartialEq> AsMutSlice<K, V> for VecDequeMap<K, V> {
    fn as_mut_slice(&mut self) -> &mut [(K, V)] {
        self.vecdeque.as_mut_slices().concat()
    }
}
