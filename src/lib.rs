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

pub mod array;
mod fat_vec;
///Contains the map trait and its related types.
pub mod map;
///Methods used internally throughout the crate.
mod private;
mod public;
///contains the set trait and its related types.
pub mod set;
///This is in the crate root because it's used internally but we still need it throughout the
///fallible module internally.
pub mod stack_list;
mod vec;
mod vecdeque;
pub use fat_vec::{FatVec, FatVecIterator, map::*, set::*};
pub use map::Map;
pub use set::Set;
pub use stack_list::{map::*, set::*};
pub use vec::{Vec, map::*, set::*};
pub use vecdeque::{map::*, set::*};

#[cfg(feature = "serde")]
mod serde;
#[cfg(test)]
mod test;
///Functions and types useful to assit verification with Kani.
pub(crate) mod verification_utils;
