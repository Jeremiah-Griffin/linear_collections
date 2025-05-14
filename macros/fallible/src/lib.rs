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
        let element_string = e.to_token_stream().to_string();

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


