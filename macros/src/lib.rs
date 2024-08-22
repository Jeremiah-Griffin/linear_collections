use std::collections::BTreeSet;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, ExprTuple, Token};

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
