use std::error::Error;

//Never implement clone: panics on alloc failure.
///Provides methods for maps backed by linear data structures like arrays and vectors.
///Because arrays may implement this type, we cannot assume that implementors will be dynamically sized.
///Only methods which do not require manipulating the length or capacity of the store are provided here:
///this is to permit the implementation of fixed sized types backed by arrays.
pub trait Map<K: Eq, V> {
    type Backing;
    //Aliasing the InsertionError allows us to implement this for both heap allocated types which return TryReserveError
    //and the stack allocated ArrayVec which return ArrayVecError.
    type InsertionError: Error;

    ///Inserts a key-value pair into the map.
    ///If the map did not have this key present, None is returned.
    ///If the map did have this key present, the value is updated, and the old value is returned. The key is not updated, though; this matters for types that can be == without being identical. See the module-level documentation for more.
    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::InsertionError>;
    ///Consumes self, returning the underlying store.
    fn into_inner(self) -> Self::Backing;

    ///Returns an iterator over the keys and values of of this `Map`.
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a;

    ///Returns an iterator over the keys of the `Map`, with mutable references to its values.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (&'a K, &'a mut V)>
    where
        K: 'a,
        V: 'a;

    ///Returns the number of items in this `Map`
    fn len(&self) -> usize;

    ///Returns the entry at
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
    fn contains_value(&self, value: &V) -> bool
    where
        V: PartialEq,
    {
        for (_, v) in self.iter() {
            if v == value {
                return true;
            }
        }
        false
    }

    ///Gets a reference with the associated key. Will return None if that i
    ///key is not in the map.
    fn get<'a, 'k>(&'a self, key: &'k K) -> Option<&'a V>
    //this bound confuses the hell out of me.
    where
        K: 'a,
    {
        self.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    ///Gets a mutable reference with the associated key. Will return None if that
    ///key is not in the map.
    fn get_mut<'a, 'k>(&'a mut self, key: &'k K) -> Option<&'a mut V>
    where
        K: 'a,
    {
        self.iter_mut().find(|(k, _)| *k == key).map(|(_, v)| v)
    }

    ///Returns `true` if this map is empty and `false` otherwise.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    ///Iterator over the keys of this map.
    fn keys<'a>(&'a self) -> impl Iterator<Item = &'a K>
    where
        K: 'a,
        V: 'a,
    {
        self.iter().map(|(k, _)| k)
    }

    /*
    //TODO: TryClone.
    //removed from fallible as we don't have a good way of copying elements with a guarantee of no panics.
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
        V: 'a ,
    {
        iter.for_each(|(k, v)| self.replace(&k, v.clone().to_owned()))
    }*/
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

    ///Tries to remove the value associated with the given key, returning None if it is not found.
    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove_entry(key).map(|(_, v)| v)
    }

    ///Iterator over the values of this map, returning a shared reference to each.
    fn values<'a>(&'a self) -> impl Iterator<Item = &'a V>
    where
        K: 'a,
        V: 'a,
    {
        self.iter().map(|(_, v)| v)
    }

    ///Iterator over the values of this map, returning an exclusive reference to each.
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
            .find(|(k, _)| *k == key)
            .map(|(_, v)| *v = value);
    }
}
