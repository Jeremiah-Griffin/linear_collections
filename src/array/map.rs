#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
///A map type backed by an Array, stack allocated and fixed in size.
///
///ArrayMap is the only map type in linear_collections which does *not* implement either LinearMap nor InfallibleLinearMap, which relies on
///dynamic memory allocation to function.
pub struct ArrayMap<K: Eq, V, const LENGTH: usize> {
    array: [(K, V); LENGTH],
}

impl<K: Eq, V, const LENGTH: usize> ArrayMap<K, V, LENGTH> {
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

    pub const fn as_slice(&self) -> &[(K, V)] {
        &self.array
    }

    ///Returns the number of elements in the ArrayMap
    pub const fn len(&self) -> usize {
        LENGTH
    }

    ///Returns true if the store is empty, false otherwise.
    pub const fn is_empty(&self) -> bool {
        LENGTH == 0
    }

    pub fn get<'a>(&'a self, key: &K) -> Option<&V> {
        self.array.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn get_mut<'a>(&'a mut self, key: &K) -> Option<&mut V> {
        self.array
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn into_inner(self) -> [(K, V); LENGTH] {
        self.array
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &(K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.array.iter()
    }

    pub fn keys<'a>(&'a self) -> impl Iterator<Item = &K> {
        self.iter().map(|(k, _)| k)
    }

    pub fn values<'a>(&'a self) -> impl Iterator<Item = &V> {
        self.iter().map(|(_, v)| v)
    }

    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut V> {
        self.array.iter_mut().map(|(_, v)| v)
    }
}
