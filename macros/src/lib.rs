use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::{BTreeSet, HashSet};
use syn::{parse_macro_input, Expr, ExprArray, ExprTuple};

///should change this to just panic on invalid imports and switch the
///qyote! to emit the different type of struct.
#[proc_macro]
pub fn array_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ExprArray);
    let elements = input.elems;
    let iter = elements.iter();
    let length = elements.len();

    if length == 0 {
        panic!("Input may not be empty")
    };

    //TODO: need to parse elements as tuples, forgor

    let mut duplicates: Vec<String> = Vec::new();
    let mut keys: BTreeSet<String> = BTreeSet::new();

    for e in iter.clone() {
        let element_string = e.to_token_stream().to_string();
        let Expr::Tuple(e) = e else {
            panic!(
                "All elements input to this macro should be tuples, but {element_string} is not."
            );
        };

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

    quote! {
        unsafe{
            linear_collections::ArrayMap::new_unchecked([#(#iter),*])

        }
    }
    .into()
}
/*
#[proc_macro]
pub fn vec_map(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}

#[proc_macro]
pub fn vec_set(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}

#[proc_macro]
pub fn vecdeque_map(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}

#[proc_macro]
pub fn vecdeque_set(tokens: TokenStream) -> TokenStream {
    unimplemented!()
}
*/
