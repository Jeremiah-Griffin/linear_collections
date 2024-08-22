use std::collections::TryReserveError;

mod fat_vec;
mod vec;
mod vecdeque;

pub use fat_vec::{map::*, set::*};
pub use vec::{map::*, set::*};
pub use vecdeque::{map::*, set::*};

#[cfg(feature = "fallible_macros")]
pub use fallible_linear_collections_macros::*;

//This is allowed as making AsMutSlice public would permit
//clients to wantonly break invariants of the collection
#[allow(private_bounds)]
///Provides methods for maps backed by linear data structures like arrays and vectors.
///Because arrays may implement this type, we cannot assume that implementors will be dynamically sized.
///Only methods which do not require manipulating the length or capacity of the store are provided here:
///this is to permit the implementation of fixed sized types backed by arrays.
pub trait FallibleLinearMap<K: Eq, V: Sized + PartialEq> {
    type Backing;

    ///Inserts a key-value pair into the map.
    ///If the map did not have this key present, None is returned.
    ///If the map did have this key present, the value is updated, and the old value is returned. The key is not updated, though; this matters for types that can be == without being identical. See the module-level documentation for more.
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, TryReserveError>;

    ///Consumes self, returning the underlying store.
    fn into_inner(self) -> Self::Backing;

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a;

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a;

    fn len(&self) -> usize;

    fn remove_entry(&mut self, key: &K) -> Option<(K, V)>;

    //notice to implementors: if calling as_slice is not zero cost, override
    //this default implementation with one that is.
    ///Returns true if this map contains the given key. False otherwise.
    fn contains_key(&self, key: &K) -> bool {
        for (k, _) in self.iter() {
            if k == key {
                return true;
            }
        }
        false
    }

    ///Returns true if this map contains a given value. False otherwise.
    fn contains_value(&self, value: &V) -> bool {
        for (_, v) in self.iter() {
            if v == value {
                return true;
            }
        }
        false
    }

    ///Gets a reference with the associated key. Will return None if that i
    ///key is not in the map.
    fn get<'a>(&'a self, key: &'a K) -> Option<&'a V> {
        self.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    ///Gets a mutable reference with the associated key. Will return None if that
    ///key is not in the map.
    fn get_mut<'a>(&'a mut self, key: &'a K) -> Option<&'a mut V> {
        self.iter_mut().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    ///Returns `true` if this map is empty and `false` otherwise.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K>
    where
        K: 'a,
        V: 'a,
    {
        self.iter().map(|(k, _)| k)
    }

    fn keys_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut K>
    where
        K: 'a,
        V: 'a,
    {
        self.iter_mut().map(|(k, _)| k)
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
    ///Gets a reference to the nth value in the map.
    ///Will return None if index is out of bounds.
    fn nth_value<'a>(&'a self, index: usize) -> Option<&'a V>
    where
        K: 'a,
    {
        self.iter().nth(index).map(|(_, v)| v)
    }

    ///Gets a reference to the nth value in the map.
    ///Will return None if index is out of bounds.    
    fn nth_value_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut V>
    where
        K: 'a,
    {
        self.iter_mut().nth(index).map(|(_, v)| v)
    }

    ///Gets a reference to the nth value in the map.
    ///Will return None if index is out of bounds.
    fn nth_key<'a>(&'a self, index: usize) -> Option<&'a K>
    where
        V: 'a,
    {
        self.iter().nth(index).map(|(k, _)| k)
    }

    ///Gets a reference to the nth key in the map.
    ///Will return None if index is out of bounds.    
    fn nth_key_mut<'a>(&'a mut self, index: usize) -> Option<&'a mut K>
    where
        V: 'a,
    {
        self.iter_mut().nth(index).map(|(k, _)| k)
    }

    ///Tries to remove the value associated with the given key, returning None if it is not found.
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove_entry(key).map(|(_, v)| v)
    }

    fn values<'a>(&'a self) -> impl Iterator<Item = &'a V>
    where
        K: 'a,
        V: 'a,
    {
        self.iter().map(|(_, v)| v)
    }

    fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut V>
    where
        K: 'a,
        V: 'a,
    {
        self.iter_mut().map(|(_, v)| v)
    }

    ///Searches for a key == key in the map. If it is present
    ///replaces its value with "value". If not, it does nothing.
    fn replace(&mut self, key: &K, value: V) {
        self.iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| *v = value);
    }
}

///Set types backed by a LinearMap<K, ()>
pub trait FallibleLinearSet<T: Eq>: Sized {
    ///The map type which backs this set.
    type BACKING: FallibleLinearMap<T, ()>;

    fn map(&self) -> &Self::BACKING;

    fn map_mut(&mut self) -> &mut Self::BACKING;

    ///Returns true if the referenced value is in the set, false otherwise.
    fn contains(&self, value: &T) -> bool {
        self.map().contains_key(value)
    }

    ///Adds a value to the set.
    ///If the set did not previously contain this value, true is returned.
    ///If the set already contained this value, false is returned, and the set is not modified: original value is not replaced, and the value passed as argument is dropped.
    fn insert(&mut self, value: T) -> Result<bool, TryReserveError> {
        self.map_mut().insert(value, ()).map(|r| r.is_none())
    }

    fn is_empty(&self) -> bool {
        self.map().is_empty()
    }

    fn len(&self) -> usize {
        self.map().len()
    }

    fn values<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.map().keys()
    }

    fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a,
    {
        self.map_mut().keys_mut()
    }

    ///Attempts to remove the referenced value from the set, returning None if it is not present.
    fn remove(&mut self, value: &T) -> Option<T> {
        self.map_mut().remove_entry(value).map(|(k, _)| k)
    }
}
