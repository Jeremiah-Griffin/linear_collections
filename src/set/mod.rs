use crate::Map;

///Set types backed by a Map<T, ()>.
pub trait Set<T: Eq> {
    ///The map type which backs this set.
    type Backing: Map<T, ()>;


    fn map(&self) -> &Self::Backing;

    fn map_mut(&mut self) -> &mut Self::Backing;

    ///Returns true if the referenced value is in the set, false otherwise.
    fn contains(&self, value: &T) -> bool {
        self.map().contains_key(value)
    }

    //Generic over E allows us to implement this for both heap allocated types which return TryReserveError
    //and the stack allocated ArrayVec which return ArrayVecError.
    ///Adds a value to the set.
    ///If the set did not previously contain this value, true is returned.
    ///If the set already contained this value, false is returned, and the set is not modified: original value is not replaced, and the value passed as argument is dropped.
    fn insert(
        &mut self,
        value: T,
    ) -> Result<bool, <Self::Backing as Map<T, ()>>::InsertionError> {
        self.map_mut().insert(value, ()).map(|r| r.is_none())
    }

    ///Returns `true` if this set is empty. `false` otherwise.
    fn is_empty(&self) -> bool {
        self.map().is_empty()
    }

    ///The number of items contained in this set.
    fn len(&self) -> usize {
        self.map().len()
    }

    ///Iterates over the values in this set.
    fn values<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.map().keys()
    }

    ///Attempts to remove the referenced value from the set, returning None if it is not present.
    fn remove(&mut self, value: &T) -> Option<T> {
        self.map_mut().remove_entry(value).map(|(k, _)| k)
    }
}
