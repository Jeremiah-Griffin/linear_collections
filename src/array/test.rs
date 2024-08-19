use crate::{ArrayMap, LinearMap};
#[allow(unused_imports)]
#[cfg(test)]
#[test]
///Since I keep forgetting this, if len is moved into the trait
///and thus made non-const this test will fail to compile.
fn array_map_len_is_const() {
    const fn foo() -> usize {
        let f = unsafe { ArrayMap::from_array_unchecked([(1, 1)]) };

        f.len()
    }

    assert_eq!(foo(), 1);
}

#[test]
///Since I keep forgetting this, if len is moved into the trait
///and thus made non-const this test will fail to compile.
fn array_map_is_empty_is_const() {
    const fn foo() -> bool {
        let f = unsafe { ArrayMap::from_array_unchecked([(1, 1)]) };

        f.is_empty()
    }

    assert_eq!(foo(), false);
}

#[test]
///Will fail to compile if from_array_unchecked is not const.
fn from_array_unchecked_is_const() {
    const fn _foo() {
        let _ = unsafe { ArrayMap::from_array_unchecked([(1, 1)]) };
    }
}
