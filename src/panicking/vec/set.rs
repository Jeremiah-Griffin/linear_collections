use crate::{LinearMap, VecMap};

///A set backed by a VecMap where the value for each key is ().
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct VecSet<T: Eq> {
    //TODO: Iterators, pub crate is a dumb cheat to get around lack of iterator support. Fix when adding iterators.
    pub(crate) map: VecMap<T, ()>,
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

    ///Returns the number of items in of the set
    pub fn len(&self) -> usize {
        self.map.len()
    }

    ///Returns true if the store is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    ///Adds a value to the set.
    ///If the set did not previously contain this value, true is returned.
    ///If the set already contained this value, false is returned, and the set is not modified: original value is not replaced, and the value passed as argument is dropped.   
    pub fn insert(&mut self, value: T) -> bool {
        self.map.insert(value, ()).is_some()
    }

    ///Attempts to remove the referenced value from the set, returning None if it is not present.
    pub fn remove(&mut self, value: &T) -> Option<T> {
        self.map.remove_entry(&value).map(|(t, _)| t)
    }

    ///Returns true if the referenced value is in the set, false otherwise.
    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    ///Returns a vector with all the elements in the set.
    pub fn into_vec(self) -> Vec<T> {
        //TODO:...since () is a ZST can I just transmute? This is silly.
        self.map.into_inner().into_iter().map(|(t, _)| t).collect()
    }
}
