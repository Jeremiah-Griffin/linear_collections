pub mod map;

#[cfg(any(feature = "fallible_macros", feature = "panicking_macros"))]
pub use linear_collections_macros::array_map;

#[cfg(test)]
mod test;
