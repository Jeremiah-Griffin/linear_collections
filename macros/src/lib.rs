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

#[proc_macro]
///NOTE: This macro does not currently consider prefixed or suffixed items (r"t" and "t" or 1 and 1usize) to be
///distinct. This is highly likely to change in the future and this fix may not be considered a breaking change.
///
///Creates an `FatMap`, checking at compile time that there are no duplicate keys.
///Example:
///`let map: FatMap<Char, i32, 3> = fat_map![('A', 1), ('B', 2), ('C',3)];`
pub fn fallible_fat_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::fallible::FatMap::from_fatvec_unchecked(linear_collections::fallible::fallible_fat_vec![#(#iter),*])
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
pub fn fallible_fat_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);

    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::fallible::FatSet::from_map_unchecked(linear_collections::fallible::fallible_fat_map![#((#iter, ())),*])
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
pub fn fallible_vec_map(tokens: TokenStream) -> TokenStream {
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
pub fn fallible_vec_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);

    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::fallible::VecSet::from_map_unchecked(linear_collections::fallible::fallible_vec_map![#((#iter, ())),*])
        }
    }
    .into()
}

#[proc_macro]
pub fn fallible_deque_map(tokens: TokenStream) -> TokenStream {
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
pub fn fallible_deque_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);
    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::fallible::DequeSet::from_map_unchecked(linear_collections::fallible::fallible_deque_map![#((#iter, ())),*])
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
pub fn panicking_fat_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::panicking::FatMap::from_fatvec_unchecked(linear_collections::panicking::panicking_fat_vec![#(#iter),*])
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
pub fn panicking_fat_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);

    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::panicking::FatSet::from_map_unchecked(linear_collections::panicking::panicking_fat_map![#((#iter, ())),*])
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
pub fn panicking_vec_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::panicking::VecMap::from_vec_unchecked(vec![#(#iter),*])
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
            linear_collections::panicking::VecSet::from_map_unchecked(linear_collections::panicking::panicking_vec_map![#((#iter, ())),*])
        }
    }
    .into()
}

#[proc_macro]
pub fn panicking_deque_map(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as MapLiteral);

    validate_map_literal(&input);

    let iter = input.inner.iter();
    quote! {
        unsafe{
            linear_collections::panicking::DequeMap::from_vecdeque_unchecked(std::collections::VecDeque::from::([#(#iter),*]))
        }
    }
    .into()
}

#[proc_macro]
pub fn panicking_deque_set(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as SetLiteral);
    let iter = input.inner.iter();

    quote! {
        unsafe{
            linear_collections::panicking::DequeSet::from_map_unchecked(linear_collections::panicking::panicking_deque_map![#((#iter, ())),*])
        }
    }
    .into()
}
