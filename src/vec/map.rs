use crate::{AsMutSlice, LinearMap};

///A map type backed by a Vector. Useful for small collections whose size can change.
pub struct VecMap<K: Eq, V: Sized + PartialEq> {
    vector: Vec<(K, V)>,
}
impl<K: Eq, V: Sized + PartialEq> VecMap<K, V> {
    ///Creates a new, empty VecMap.
    ///Calls Vec::new() internally.
    pub fn new() -> VecMap<K, V> {
        VecMap { vector: Vec::new() }
    }

    ///**Please only use this method to create maps at compile time if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new VecMap from the
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical key beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_vec_unchecked(vector: Vec<(K, V)>) -> VecMap<K, V> {
        VecMap { vector }
    }

    ///Creates a new, empty VecMap with capacity set to the provide value.
    ///Calls Vec::with_capacity() internally.
    pub fn with_capacity(capacity: usize) -> VecMap<K, V> {
        VecMap {
            vector: Vec::with_capacity(capacity),
        }
    }
    ///Tries to remove the value associated with the given key, returning None if it is not found.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.remove_entry(key).map(|(_, v)| v)
    }

    ///Tries to remove the entry associated with the given key, returning None if it is not found.
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .vector
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        Some(self.vector.remove(idx))
    }

    ///Inserts the provided value into the VecMap. If the provided key is
    ///found it will update the value. and return the old value. If not, this will allocate for a new key value pair.    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.vector.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => Some(std::mem::replace(v, value)),
            None => {
                self.vector.push((key, value));
                None
            }
        }
    }
}

impl<K: Eq, V: Sized + PartialEq> LinearMap<K, V> for VecMap<K, V> {
    type Backing = Vec<(K, V)>;
    fn as_slice(&self) -> &[(K, V)] {
        self.vector.as_slice()
    }

    fn into_inner(self) -> Self::Backing {
        self.vector
    }
}

impl<K: Eq, V: Sized + PartialEq> AsMutSlice<K, V> for VecMap<K, V> {
    fn as_mut_slice(&mut self) -> &mut [(K, V)] {
        self.vector.as_mut_slice()
    }
}
