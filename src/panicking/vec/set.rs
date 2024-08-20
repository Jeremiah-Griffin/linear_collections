use crate::{panicking::InfallibleLinearSet, InfallibleLinearMap};

use super::map::VecMap;

///A set backed by a VecMap where the value for each key is ().
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VecSet<T: Eq> {
    map: VecMap<T, ()>,
}

impl<T: Eq> InfallibleLinearSet<T> for VecSet<T> {
    type BACKING = VecMap<T, ()>;

    fn map(&self) -> &Self::BACKING {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::BACKING {
        &mut self.map
    }

    fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_none()
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        self.map.remove_entry(value).map(|(v, _)| v)
    }
}

impl<T: Eq> VecSet<T> {
    ///Creates a new, empty VecSet
    pub fn new() -> Self {
        VecSet { map: VecMap::new() }
    }

    ///Creates a VecSet that can hold `capacity` elements without reallocating
    pub fn with_capacity(capacity: usize) -> Self {
        VecSet {
            map: VecMap::with_capacity(capacity),
        }
    }

    ///Returns a vector with all the elements in the set.
    pub fn into_vec(self) -> Vec<T> {
        //TODO:...since () is a ZST can I just transmute? This is silly.
        self.map.into_inner().into_iter().map(|(t, _)| t).collect()
    }

    pub fn len(&self) -> usize {
        self.map.len()
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
