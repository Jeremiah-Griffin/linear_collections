#![allow(dead_code)]
#![cfg_attr(feature = "nightly_fallible", allow(internal_features))]
#![cfg_attr(feature = "nightly_fallible", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly_fallible", feature(try_reserve_kind))]
#![cfg_attr(feature = "nightly_fallible", feature(try_with_capacity))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_ext))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_trait))]
mod array;
#[cfg(feature = "nightly_fallible")]
//added but not exposed pending miri testing
//We always compile fallible as the infallible versions are just fallible with panic called on the additional methods.
mod fallible;
pub use array::map::ArrayMap;

#[cfg(feature = "macros")]
pub use linear_collections_macros::{array_map, vec_map, vec_set};

#[cfg(feature = "panicking")]
mod panicking;
#[cfg(feature = "panicking")]
pub use panicking::vec::{map::VecMap, set::VecSet};

#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test;
