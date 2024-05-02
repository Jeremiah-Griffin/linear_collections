use crate::{LinearMap, VecMap};

pub struct VecSet<T: Eq> {
    map: VecMap<T, ()>,
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

    ///Returns the number of items in of the set
    pub fn len(&self) -> usize {
        self.map.len()
    }

    ///Returns true if the store is empty, false otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
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

    ///Returns the backing vector of this type
    pub fn into_inner(self) -> Vec<T> {
        //TODO:...since unit is a ZST can I just transmute? This is silly.
        self.map.into_inner().into_iter().map(|(t, _)| t).collect()
    }
}
