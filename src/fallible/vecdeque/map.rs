use std::collections::{TryReserveError, VecDeque};

use crate::fallible::FallibleLinearMap;

pub struct DequeMap<K: Eq, V: Sized + PartialEq> {
    deque: VecDeque<(K, V)>,
}

impl<K: Eq, V: Sized + PartialEq> DequeMap<K, V> {
    pub fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        Ok(Self {
            deque: VecDeque::with_capacity(capacity),
        })
    }

    ///**Please only use this method to create maps at compile time if the "macros" feature is unavailable to you**
    ///"macros" provides safe, checked alternatives to initialize linear maps with compile time checking
    ///of the invariants of each type.
    ///
    ///Creates a new VecDequeMap from the
    ///
    ///SAFETY: improper use of this method - initializing with duplicate keys -will NOT create memory unsafety, but will result in every
    ///identical key beyond the first never getting accessed as LinearMaps short circuit on the first matching key.
    pub const unsafe fn from_vecdeque_unchecked(vecdeque: VecDeque<(K, V)>) -> DequeMap<K, V> {
        DequeMap { deque: vecdeque }
    }
}

impl<K: Eq, V: Sized + PartialEq> FallibleLinearMap<K, V> for DequeMap<K, V> {
    type Backing = VecDeque<(K, V)>;

    fn insert(&mut self, key: K, value: V) -> Result<Option<V>, std::collections::TryReserveError> {
        let mut iter = self.deque.iter_mut();
        match iter.find(|(k, _)| *k == key) {
            Some((_, v)) => Ok(Some(std::mem::replace(v, value))),
            None => {
                //need to manually drop because the Result gets created as a temporary (?)
                drop(iter);

                if self.deque.capacity() <= self.deque.len() {
                    self.deque.try_reserve(1)?;
                }

                self.deque.push_back((key, value));
                Ok(None)
            }
        }
    }

    fn into_inner(self) -> Self::Backing {
        self.deque
    }

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.deque.iter()
    }

    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a,
    {
        self.deque.iter_mut()
    }

    fn remove_entry(&mut self, key: &K) -> Option<(K, V)> {
        let idx = self
            .deque
            .iter()
            .enumerate()
            .find(|(_, (k, _))| k == key)
            .map(|(i, _)| i)?;

        self.deque.remove(idx)
    }

    fn len(&self) -> usize {
        self.deque.len()
    }
}
