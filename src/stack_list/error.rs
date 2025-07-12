use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum StackListError {
    WouldExceedCapacity,
}

impl Display for StackListError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackListError::WouldExceedCapacity => write!(f, "push would exceed capacity"),
        }
    }
}

impl Error for StackListError {}
