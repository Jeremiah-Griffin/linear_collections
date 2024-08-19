use crate::LinearMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
///A map type backed by an Array, stack allocated and fixed in size.
pub struct ArrayMap<K: Eq, V: Sized + PartialEq, const LENGTH: usize> {
    array: [(K, V); LENGTH],
}
impl<K: Eq, V: Sized + PartialEq, const LENGTH: usize> ArrayMap<K, V, LENGTH> {
    ///**Please only use this method to create map literals if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new ArrayMap from the supplied array.
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical key beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_array_unchecked(array: [(K, V); LENGTH]) -> ArrayMap<K, V, LENGTH> {
        ArrayMap { array }
    }

    ///Returns the number of elements in the ArrayMap
    pub const fn len(&self) -> usize {
        LENGTH
    }

    ///Returns true if the store is empty, false otherwise.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K: Eq, V: Sized + PartialEq, const LENGTH: usize> LinearMap<K, V> for ArrayMap<K, V, LENGTH> {
    type Backing = [(K, V); LENGTH];

    fn into_inner(self) -> Self::Backing {
        self.array
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &(K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.array.iter()
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.array.iter_mut()
    }
}
