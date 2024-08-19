#![allow(dead_code)]
#![cfg_attr(feature = "nightly_fallible", allow(internal_features))]
#![cfg_attr(feature = "nightly_fallible", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly_fallible", feature(try_reserve_kind))]
#![cfg_attr(feature = "nightly_fallible", feature(try_with_capacity))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_ext))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_trait))]
mod array;

//We make the modules public but *not* the contained types. Certain projects need only one type or the other.
//It would be unfortunate for a low level library which can only use fallible types to be forced to specify "FallibleFatVec".
//instead, if a library user *must* use both types, they should use the qualified path up to the module, fallible or panicking.

#[cfg(feature = "nightly_fallible")]
//added but not exposed pending miri testing
//We always compile fallible as the infallible versions are just fallible with panic called on the additional methods.
pub mod fallible;

#[cfg(features = "nightly_fallible")]
pub use fallible::{FallibleLinearMap, FallibleLinearSet};

pub use array::map::ArrayMap;

#[cfg(feature = "macros")]
pub use linear_collections_macros::{array_map, vec_map, vec_set};

#[cfg(feature = "panicking")]
pub mod panicking;
#[cfg(feature = "panicking")]
pub use panicking::{InfallibleLinearMap, InfallibleLinearSet};

#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test;
