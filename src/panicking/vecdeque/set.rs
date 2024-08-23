use crate::panicking::DequeMap;
use crate::panicking::PanickingLinearSet;

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
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: DequeMap::with_capacity(capacity),
        }
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

impl<T: Eq> PanickingLinearSet<T> for DequeSet<T> {
    type BACKING = DequeMap<T, ()>;

    fn map(&self) -> &Self::BACKING {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::BACKING {
        &mut self.map
    }
}
