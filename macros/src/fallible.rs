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
            linear_collections::fallible::FatMap::from_fatvec_unchecked([#(#iter),*])
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
            linear_collections::fallible::FatSet::from_map_unchecked(vec_map![#((#iter, ())),*])
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
            linear_collections::fallible::VecSet::from_map_unchecked(vec_map![#((#iter, ())),*])
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
            linear_collections::fallible::DequeMap::from_vecdeque_unchecked(vec![#(#iter),*])
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
            linear_collections::fallible::DequeMap::from_map_unchecked(vec_map![#((#iter, ())),*])
        }
    }
    .into()
}
