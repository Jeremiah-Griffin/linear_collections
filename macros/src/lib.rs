use crate::helpers::{validate_map_literal, MapLiteral, SetLiteral};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
mod helpers;

#[allow(unused_imports)]
#[proc_macro]
///NOTE: This macro does not currently consider prefixed or suffixed items (r"t" and "t" or 1 and 1usize) to be
///distinct. This is highly likely to change in the future and this fix may not be considered a breaking change.
///
///Creates an ArrayMap, checking at compile time that there are no duplicate keys.
///Example:
///`let map: ArrayMap<Char, i32, 3> = array_map![('A', 1), ('B', 2), ('C',3)];`
pub fn array_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::ArrayMap::from_array_unchecked([#(#iter),*])
        }
    }
    .into()
}

pub mod fallible;
pub mod panicking;
