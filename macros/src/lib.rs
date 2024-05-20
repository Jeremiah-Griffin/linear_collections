use crate::helpers::{validate_map_literal, MapLiteral, SetLiteral};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, ExprTuple, Token};
mod helpers;

#[proc_macro]
///Creates an ArrayMap, checking at compile time that there are no duplicate keys.
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
///Creates a VecMap, checking at compile time that there are no duplicate keys.
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
///Creates a VecMap, checking at compile time that there are no duplicate values.
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
