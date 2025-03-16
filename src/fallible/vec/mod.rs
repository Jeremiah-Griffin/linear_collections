use std::collections::TryReserveError;

pub mod map;
pub mod set;

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

    p
}
