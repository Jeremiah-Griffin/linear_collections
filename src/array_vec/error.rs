use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum ArrayVecError {
    WouldExceedCapacity,
}

impl Display for ArrayVecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArrayVecError::WouldExceedCapacity => write!(f, "push would exceed capacity"),
        }
    }
}

impl Error for ArrayVecError {}
