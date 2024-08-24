use crate::fallible::{FallibleLinearMap, FallibleLinearSet};

use super::map::StackMap;

pub struct StackSet<T: Eq, const CAPACITY: usize> {
    map: StackMap<T, (), CAPACITY>,
}

impl<T: Eq, const STACK_CAPACITY: usize> StackSet<T, STACK_CAPACITY> {
    pub fn new() -> Self {
        Self {
            map: StackMap::new(),
        }
    }
}

impl<T: Eq, const STACK_CAPACITY: usize> FallibleLinearSet<T> for StackSet<T, STACK_CAPACITY> {
    type Backing = StackMap<T, (), STACK_CAPACITY>;

    fn map(&self) -> &Self::Backing {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::Backing {
        &mut self.map
    }

    fn contains(&self, value: &T) -> bool {
        self.map().contains_key(value)
    }

    fn insert(
        &mut self,
        value: T,
    ) -> Result<bool, <Self::Backing as crate::fallible::FallibleLinearMap<T, ()>>::InsertionError>
    {
        self.map_mut().insert(value, ()).map(|r| r.is_none())
    }

    fn is_empty(&self) -> bool {
        self.map().is_empty()
    }

    fn len(&self) -> usize {
        self.map().len()
    }

    fn values<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.map().keys()
    }

    fn remove(&mut self, value: &T) -> Option<T> {
        self.map_mut().remove_entry(value).map(|(k, _)| k)
    }
}
