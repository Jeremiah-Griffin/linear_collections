use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug)]
pub enum PushError {
    WouldExceedCapacity,
}

impl Display for PushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PushError::WouldExceedCapacity => write!(f, "push would exceed capacity"),
        }
    }
}

impl Error for PushError {}
