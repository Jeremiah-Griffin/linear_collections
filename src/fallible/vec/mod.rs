use std::collections::TryReserveError;

pub mod map;
pub mod set;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
///TODO: replace this with a rawvec type. RawVec panic on out_of_capacity as well.
///Need to catch everything as an error.
#[repr(transparent)]
pub struct Vec<T> {
    inner: std::vec::Vec<T>,
}

impl<T> self::Vec<T> {
    pub const fn new() -> Self {
        Self {
            inner: std::vec::Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Result<Self, TryReserveError> {
        std::vec::Vec::try_with_capacity(capacity).map(|inner| Vec { inner })
    }

    pub fn push(&mut self, item: T) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(1)?;
        self.inner.push(item);

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.inner.iter()
    }

    pub fn reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.inner.try_reserve_exact(additional)
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.inner.into_iter()
    }
}
