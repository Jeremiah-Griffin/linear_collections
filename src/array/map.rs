use crate::{AsMutSlice, LinearMap};

#[derive(Copy, Clone)]
pub struct ArrayMap<K: Eq, V: Sized + PartialEq, const LENGTH: usize> {
    array: [(K, V); LENGTH],
}
impl<K: Eq, V: Sized + PartialEq, const LENGTH: usize> ArrayMap<K, V, LENGTH> {
    ///Creates a new ArrayMap from the provided array: use this if you know for a fact that
    ///each key provided is unique.
    ///
    ///Don't be afraid by the unsafe marker: improper use of this method will NOT create memory unsafety,
    ///but will result in every identical key beyond the first never getting accessed as LinearMaps short circuit
    ///on the first matching key.
    pub const unsafe fn new_unchecked(array: [(K, V); LENGTH]) -> ArrayMap<K, V, LENGTH> {
        ArrayMap { array }
    }

    ///Returns the number of items in this ArrayMap
    pub const fn len(&self) -> usize {
        self.array.len()
    }

    ///Returns true if the store is empty, false otherwise.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///Replaces a values all  key value pairs matching an element from iter with
    ///that element from iter.
    ///
    ///for example:
    ///[(A,1), (B, 2)].merge([(A,1), (B, 2'), (C, 2), (D, 3)].into_iter())
    ///will yield a map:
    ///[(A, 1), (B, 2')]
    pub fn merge_from_iter(&mut self, iter: impl Iterator<Item = (K, V)>) {
        iter.for_each(|(k, v)| self.replace(&k, v))
    }
}

impl<K: Eq, V: Sized + PartialEq, const LENGTH: usize> LinearMap<K, V> for ArrayMap<K, V, LENGTH> {
    type Backing = [(K, V); LENGTH];
    fn as_slice(&self) -> &[(K, V)] {
        &self.array
    }

    fn into_inner(self) -> Self::Backing {
        self.array
    }
}

impl<K: Eq, V: Sized + PartialEq, const LENGTH: usize> AsMutSlice<K, V> for ArrayMap<K, V, LENGTH> {
    fn as_mut_slice(&mut self) -> &mut [(K, V)] {
        &mut self.array
    }
}
