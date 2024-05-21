use crate::helpers::{validate_map_literal, MapLiteral, SetLiteral};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, ExprTuple, Token};
mod helpers;

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

#[proc_macro]
///NOTE: This macro does not currently consider prefixed or suffixed items (r"t" and "t" or 1 and 1usize) to be
///distinct. This is highly likely to change in the future and this fix may not be considered a breaking change.
///
///Creates a VecMap, checking at compile time that there are no duplicate keys.j
///Example:
///`let map: VecMap<Char, i32> = vec_map![('A', 1), ('B', 2), ('C',3)];`
pub fn vec_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::VecMap::from_vec_unchecked(vec![#(#iter),*])
        }
    }
    .into()
}

#[proc_macro]
///NOTE: This macro does not currently consider prefixed or suffixed items (r"t" and "t" or 1 and 1usize) to be
///distinct. This is highly likely to change in the future and this fix may not be considered a breaking change.
///
///Creates a VecSet, checking at compile time that there are no duplicate values.
///Input syntax is identical to the `vec![]` macro with the caveat that duplicate values will error at compile time.
pub fn vec_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);

    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::VecSet::from_map_unchecked(vec_map![#((#iter, ())),*])
        }
    }
    .into()
}

/*
#[proc_macro]
pub fn vecdeque_map(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}

#[proc_macro]
pub fn vecdeque_set(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}
*/
