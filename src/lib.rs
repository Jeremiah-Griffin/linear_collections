#![allow(dead_code)]
//#[cfg_attr(not(feature = "panicking"), no_panic_whatsoever)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(try_reserve_kind)]
#![feature(try_with_capacity)]
#![feature(slice_concat_ext)]
#![feature(slice_concat_trait)]
#![feature(generic_const_exprs)]
#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

///Methods used internally throughout the crate.
mod private;
///Contains the map trait and its related types.
pub mod map;
///contains the set trait and its related types.
pub mod set;
pub mod array;
///This is in the crate root because it's used internally but we still need it throughout the
///fallible module internally.
pub mod stack_list;
mod fat_vec;
mod vec;
mod vecdeque;
pub use fat_vec::{map::*, set::*, FatVec, FatVecIterator};
pub use stack_list::{map::*, set::*};
pub use vec::{map::*, set::*, Vec};
pub use vecdeque::{map::*, set::*};
pub use map::Map;
pub use set::Set;


#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test;
///Functions and types useful to assit verification with Kani.
pub (crate) mod verification_utils;


///Sealed trait to provide mutable iteration without allowing consumers
///to violate the invariants of the map types
pub(crate) trait MapIterMut<K, V> {
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut (K, V)>
    where
        K: 'a,
        V: 'a;
}

