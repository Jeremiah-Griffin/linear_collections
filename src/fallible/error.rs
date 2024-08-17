use std::{collections::TryReserveError, fmt::Display};

#[derive(Debug)]
///Unified type for all memory allocation failures which may happen
pub enum AllocationError {
    ///There is no memory available left to allocate.
    OOM,
    ///The OS has refused to allocate additional memory.
    OSRefused,
    /// Error due to the computed capacity exceeding the collection's maximum
    /// (usually `isize::MAX` bytes).    
    CollectionRefused,
    ///Unspecified. Sort of a catch all, a lot of reasons for an allocation failure don't feel neatly into the above.
    Other,
}

impl Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<TryReserveError> for AllocationError {
    fn from(value: TryReserveError) -> Self {
        match value.kind() {
            std::collections::TryReserveErrorKind::CapacityOverflow => {
                AllocationError::CollectionRefused
            }
            std::collections::TryReserveErrorKind::AllocError { .. } => AllocationError::OSRefused,
        }
    }
}
