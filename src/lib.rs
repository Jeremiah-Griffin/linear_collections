/*
#![cfg_attr(feature = "nightly_vecdeque", feature(slice_concat_trait))]
#[cfg(feature = "nightly_vecdeque")]
mod vecdeque;
#[cfg(feature = "nightly_vecdeque")]
pub use vecdeque::map::VecDequeMap;
#[cfg(feature = "nightly_vecdeque")]
pub use vecdeque::set::VecDequeSet;
*/
mod array;
mod vec;
pub use array::map::ArrayMap;
#[cfg(feature = "macros")]
pub use linear_collections_macros::{array_map, vec_map, vec_set};
pub use vec::map::VecMap;
pub use vec::set::VecSet;
#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test;

///visible only within crate as callers could use this to violate internal
///invariants of implementors
pub(crate) trait AsMutSlice<K: Eq, V: Sized + PartialEq> {
    fn as_mut_slice(&mut self) -> &mut [(K, V)];
}

//This is allowed as making AsMutSlice public would permit
//clients to wantonly break invariants of the collection
#[allow(private_bounds)]
///Provides methods for maps backed by linear data structures like arrays and vectors.
///Because arrays may implement this type, we cannot assume that implementors will be dynamically sized.
///Only methods which do not require manipulating the length or capacity of the store are provided here:
///this is to permit the implementation of fixed sized types backed by arrays.
pub trait LinearMap<K: Eq, V: Sized + PartialEq>: AsMutSlice<K, V> {
    type Backing;
    ///The keys and values of the map.
    fn as_slice(&self) -> &[(K, V)];

    ///Consumes self, returning the underlying store.
    fn into_inner(self) -> Self::Backing;

    //notice to implementors: if calling as_slice is not zero cost, override
    //this default implementation with one that is.
    ///Returns true if this map contains the given key. False otherwise.
    fn contains_key(&self, key: &K) -> bool {
        for (k, _) in self.as_slice().iter() {
            if k == key {
                return true;
            }
        }
        false
    }

    ///Returns true if this map contains a given value. False otherwise.
    fn contains_value(&self, value: &V) -> bool {
        for (_, v) in self.as_slice().iter() {
            if v == value {
                return true;
            }
        }
        false
    }

    ///Gets a reference with the associated key. Will return None if that i
    ///key is not in the map.
    fn get<'a>(&'a self, key: &'a K) -> Option<&'a V> {
        self.as_slice()
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    ///Gets a mutable reference with the associated key. Will return None if that
    ///key is not in the map.
    fn get_mut<'a>(&'a mut self, key: &'a K) -> Option<&'a mut V> {
        self.as_mut_slice()
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    ///Gets a reference to the nth value in the map.
    ///Will return None if index is out of bounds.
    fn nth_value<'a>(&'a self, index: usize) -> Option<&'a V>
    where
        K: 'a,
    {
        self.as_slice().get(index).map(|(_, v)| v)
    }

    ///Gets a reference to the nth value in the map.
    ///Will return None if index is out of bounds.    
    fn nth_value_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut V>
    where
        K: 'a,
    {
        self.as_mut_slice().get_mut(index).map(|(_, v)| v)
    }

    ///Gets a reference to the nth value in the map.
    ///Will return None if index is out of bounds.
    fn nth_key<'a>(&'a self, index: usize) -> Option<&'a K>
    where
        V: 'a,
    {
        self.as_slice().get(index).map(|(k, _)| k)
    }

    ///Gets a reference to the nth key in the map.
    ///Will return None if index is out of bounds.    
    fn nth_key_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut K>
    where
        V: 'a,
    {
        self.as_mut_slice().get_mut(index).map(|(k, _)| k)
    }

    ///Searches for a key == key in the map. If it is present
    ///replaces its value with "value". If not, it does nothing.
    fn replace(&mut self, key: &K, value: V) {
        self.as_mut_slice()
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| *v = value);
    }

    ///For every key in iter which matches a key in self, this method replaces
    ///the value from iter in self, "merging" the iterator and the map.
    ///
    ///for example:
    ///[(A,1), (B, 2)].merge([(A,1), (B, 2'), (C, 2), (D, 3)].into_iter())
    ///will yield a map:
    ///[(A, 1), (B, 2')]
    fn merge_from_iter<'a>(&'a mut self, iter: impl Iterator<Item = &'a (K, V)>)
    where
        K: 'a,
        V: 'a + Clone,
    {
        iter.for_each(|(k, v)| self.replace(&k, v.clone().to_owned()))
    }
}
