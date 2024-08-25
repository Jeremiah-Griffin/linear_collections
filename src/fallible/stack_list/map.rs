use crate::{
    fallible::FallibleLinearMap,
    stack_list::{error::PushError, StackList},
    MapIterMut,
};

///A map backed by a `StackList`
pub struct StackMap<K: Eq, V: Sized + PartialEq, const CAPACITY: usize> {
    stack_list: StackList<(K, V), CAPACITY>,
}

impl<K: Eq, V: Sized + PartialEq, const CAPACITY: usize> StackMap<K, V, CAPACITY> {
    pub fn new() -> Self {
        StackMap {
            stack_list: StackList::new(),
        }
    }
}

impl<K: Eq, V: Sized + PartialEq, const CAPACITY: usize> FallibleLinearMap<K, V>
    for StackMap<K, V, CAPACITY>
{
    type Backing = StackList<(K, V), CAPACITY>;
    type InsertionError = PushError;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, Self::InsertionError> {
        let mut iter = self.stack_list.iter_mut();
        match iter.find(|(k, _)| *k == key) {
            Some((_, v)) => Ok(Some(std::mem::replace(v, value))),
            None => {
                //need to manually drop because the Result gets created as a temporary (?)
                drop(iter);
                self.stack_list.push((key, value))?;
                Ok(None)
            }
        }
    }
    fn into_inner(self) -> Self::Backing {
        self.stack_list
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.stack_list.iter()
    }

    fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .stack_list
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        self.stack_list.remove(idx)
    }

    fn len(&self) -> usize {
        self.stack_list.len()
    }
}

impl<K: Eq, V: Sized + PartialEq, const STACK_CAPACITY: usize> MapIterMut<K, V>
    for StackMap<K, V, STACK_CAPACITY>
{
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.stack_list.iter_mut()
    }
}
