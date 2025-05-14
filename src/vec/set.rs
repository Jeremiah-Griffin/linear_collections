use std::collections::TryReserveError;

use crate::{FallibleLinearMap, FallibleLinearSet};

use super::map::VecMap;

///A set backed by a VecMap where the value for each key is ().
#[derive(Debug, PartialEq, Eq, Hash, Default)]
pub struct VecSet<T: Eq> {
    map: VecMap<T, ()>,
}

impl<T: Eq> FallibleLinearSet<T> for VecSet<T> {
    type Backing = VecMap<T, ()>;

    fn map(&self) -> &Self::Backing {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::Backing {
        &mut self.map
    }
}

impl<T: Eq> VecSet<T> {
    ///Creates a new, empty VecSet
    pub fn new() -> Self {
        VecSet { map: VecMap::new() }
    }

    ///Creates a VecSet that can hold `capacity` elements without reallocating
    pub fn with_capacity(capacity: usize) -> Result<VecSet<T>, TryReserveError> {
        VecMap::with_capacity(capacity).map(|map| VecSet { map })
    }

    ///Returns a vector with all the elements in the set.
    pub fn into_vec(self) -> Vec<T> {
        //TODO:...since () is a ZST can I just transmute? This is silly.
        self.map.into_inner().into_iter().map(|(t, _)| t).collect()
    }

    ///**Please only use this method to create set literals if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new VecSet from the supplied VecMap.
    ///
    ///SAFETY: improper use of this method - initializing with duplicate values - will NOT create memory unsafety, but will result in every
    ///identical value beyond the first never getting accessed as LinearMaps short circuit on the first match.
    pub const unsafe fn from_map_unchecked(map: VecMap<T, ()>) -> VecSet<T> {
        VecSet { map }
    }
}

#[cfg(feature = "serde")]
impl<'a, T: Eq + serde::Serialize> serde::Serialize for VecSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::serde::serialize_fallible_set(self, serializer)
    }
}
