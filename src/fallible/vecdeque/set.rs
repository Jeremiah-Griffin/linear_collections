use std::collections::TryReserveError;

use crate::fallible::FallibleLinearSet;

use super::map::DequeMap;

pub struct DequeSet<T: Eq> {
    map: DequeMap<T, ()>,
}

impl<T: Eq> DequeSet<T> {
    ///Creates a new, empty DequeSet
    pub fn new() -> Self {
        Self {
            map: DequeMap::new(),
        }
    }

    ///Creates a DequeSet that can hold `capacity` elements without reallocating
    pub fn with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        DequeMap::with_capacity(capacity).map(|map| Self { map })
    }

    ///**Please only use this method to create set literals if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new DequeSet from the supplied VecMap.
    ///
    ///SAFETY: improper use of this method - initializing with duplicate values - will NOT create memory unsafety, but will result in every
    ///identical value beyond the first never getting accessed as LinearMaps short circuit on the first match.
    pub const unsafe fn from_map_unchecked(map: DequeMap<T, ()>) -> Self {
        Self { map }
    }
}

impl<T: Eq> FallibleLinearSet<T> for DequeSet<T> {
    type Backing = DequeMap<T, ()>;

    fn map(&self) -> &Self::Backing {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::Backing {
        &mut self.map
    }
}

#[cfg(feature = "serde")]
impl<'a, T: Eq + serde::Serialize> serde::Serialize for DequeSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::serde::fallible::serialize_fallible_set(self, serializer)
    }
}
