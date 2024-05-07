use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::collections::{BTreeSet, HashMap, HashSet};
use syn::{parse_macro_input, Expr, ExprArray};

#[proc_macro]
pub fn array_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ExprArray);

    //TODO: need to parse elements as tuples, forgor
    let input_length = input.elems.len();

    let mut elems = BTreeSet::new();

    let mut duplicates = BTreeSet::new();

    input
        .elems
        .into_pairs()
        .map(|p| p.into_value().into_token_stream())
        .map(|t| proc_macro::TokenStream::from(t).to_string())
        //TODO: this breaks in cases like numerical type suffixes and string prefixes
        //as those will convert to "to_string" uniquely from each other, despite the types and
        //values being identical. As such, we can do like a regex thing to extract the leading prefix
        //from string literals and the type suffix from numerics
        .for_each(|s| {
            if elems.insert(s.clone()) == false {
                duplicates.insert(s);
            }
        });

    if duplicates.len() > 0 {
        panic!("Duplicate elements found: {duplicates:?}")
    }

    quote! {
        #[allow(unsafe_code)]
        ///SAFETY: the macro guarantees that we supply no duplicate elements to this function.
        unsafe {
            linear_collections::ArrayMap::new_unchecked([#(#elems),* ; #input_length]){
        }}
    }
    .into()
}
