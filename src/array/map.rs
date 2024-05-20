use crate::{AsMutSlice, LinearMap};

#[cfg(feature = "macros")]
pub use macros::array_map;

#[derive(Copy, Clone)]
pub struct ArrayMap<K: Eq, V: Sized + PartialEq, const LENGTH: usize> {
    array: [(K, V); LENGTH],
}
impl<K: Eq, V: Sized + PartialEq, const LENGTH: usize> ArrayMap<K, V, LENGTH> {
    ///**Please only use this method to create maps at compile time if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new ArrayMap from the
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical key beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_array_unchecked(array: [(K, V); LENGTH]) -> ArrayMap<K, V, LENGTH> {
        ArrayMap { array }
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
