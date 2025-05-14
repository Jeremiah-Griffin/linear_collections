use std::collections::BTreeSet;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Expr, ExprTuple, Token};

struct MapLiteral {
    pub inner: Punctuated<ExprTuple, Token![,]>,
}

impl Parse for MapLiteral {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            inner: Punctuated::parse_terminated(input)?,
        })
    }
}

struct SetLiteral {
    pub inner: Punctuated<Expr, Token![,]>,
}

impl Parse for SetLiteral {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            inner: Punctuated::parse_terminated(input)?,
        })
    }
}
///The code for checking the input elements is identical between all maps
///and factored into this function.
fn validate_map_literal(input: &MapLiteral) {
    let elements = &input.inner;
    let length = elements.len();

    if length == 0 {
        panic!("Input may not be empty")
    };

    //TODO: need to parse elements as tuples, forgor
    let mut duplicates: Vec<String> = Vec::new();
    let mut keys: BTreeSet<String> = BTreeSet::new();

    for e in elements.iter() {
        let element_string = e.into_token_stream().to_string();

        let tuple_pair_count = e.elems.len();

        if tuple_pair_count != 2 {
            panic!("All elements should have a length of 2 ( a key and a value) but {element_string} is {tuple_pair_count}")
        }

        let pairs = e.elems.clone().into_pairs().collect::<Vec<_>>();

        let key = pairs[0].clone().into_value().to_token_stream().to_string();

        if keys.insert(key.clone()) == false {
            duplicates.push(key)
        }
    }

    if duplicates.len() > 0 {
        panic!("Duplicate keys found: {duplicates:?}")
    }
}
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
            linear_collections::array::map::ArrayMap::from_array_unchecked([#(#iter),*])
        }
    }
    .into()
}



#[proc_macro]
///NOTE: This macro does not currently consider prefixed or suffixed items (r"t" and "t" or 1 and 1usize) to be
///distinct. This is highly likely to change in the future and this fix may not be considered a breaking change.
///
///Creates an `FatMap`, checking at compile time that there are no duplicate keys.
///Example:
///`let map: FatMap<Char, i32, 3> = fat_map![('A', 1), ('B', 2), ('C',3)];`
pub fn fat_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::fallible::FatMap::from_fatvec_unchecked(linear_collections::fallible::fat_vec![#(#iter),*])
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
pub fn fat_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);

    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::fallible::FatSet::from_map_unchecked(linear_collections::fallible::fat_map![#((#iter, ())),*])
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
            linear_collections::fallible::VecMap::from_vec_unchecked(vec![#(#iter),*])
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
            linear_collections::fallible::VecSet::from_map_unchecked(linear_collections::fallible::vec_map![#((#iter, ())),*])
        }
    }
    .into()
}

#[proc_macro]
pub fn deque_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::fallible::DequeMap::from_vecdeque_unchecked(std::collections::VecDeque::from::([#(#iter),*]))
        }
    }
    .into()
}

#[proc_macro]
pub fn deque_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);
    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::fallible::DequeSet::from_map_unchecked(linear_collections::fallible::deque_map![#((#iter, ())),*])
        }
    }
    .into()
}
