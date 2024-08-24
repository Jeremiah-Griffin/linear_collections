#![allow(dead_code)]
#![cfg_attr(feature = "nightly_fallible", allow(internal_features))]
#![cfg_attr(feature = "nightly_fallible", feature(core_intrinsics))]
#![cfg_attr(feature = "nightly_fallible", feature(try_reserve_kind))]
#![cfg_attr(feature = "nightly_fallible", feature(try_with_capacity))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_ext))]
#![cfg_attr(feature = "nightly_fallible", feature(slice_concat_trait))]
pub mod array;
///This is in the crate root because it's used internally but we still need it throughout both
///fallible and panicking crates internally.
mod array_vec;

//We make the modules public but *not* the contained types. Certain projects need only one type or the other.
//It would be unfortunate for a low level library which can only use fallible types to be forced to specify "FallibleFatVec".
//instead, if a library user *must* use both types, they should use the qualified path up to the module, fallible or panicking.

#[cfg(feature = "nightly_fallible")]
///added but not exposed pending miri testing
///We always compile fallible as the infallible versions are just fallible with panic called on the additional methods.
pub mod fallible;

#[cfg(feature = "panicking")]
pub mod panicking;

/*
#[cfg(feature = "serde")]
mod serde;
*/
#[cfg(test)]
mod test;

///Sealed trait to provide mutable iteration without allowing consumers
///to violate the invariants of the map types
pub(crate) trait MapIterMut<K, V> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a;
}
