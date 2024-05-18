#[cfg(test)]
///We only test VecMap here we're really testing only the linear map trait,
///not the specific implementation. Any further invariants are upheld and tested by the
///underlying type in std.
///
///These are all ripped from std::collections::HashMap. Credit to the rust developers.
use crate::LinearMap;
use crate::VecMap;
#[test]
fn test_create_capacity_zero() {
    let mut m = crate::VecMap::with_capacity(0);

    assert!(m.insert(1, 1).is_none());

    assert!(m.contains_key(&1));
    assert!(!m.contains_key(&0));
}

#[test]
fn test_insert() {
    let mut m = VecMap::new();
    assert_eq!(m.len(), 0);
    assert!(m.insert(1, 2).is_none());
    assert_eq!(m.len(), 1);
    assert!(m.insert(2, 4).is_none());
    assert_eq!(m.len(), 2);
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&2).unwrap(), 4);
}

#[test]
fn test_empty_remove() {
    let mut m: VecMap<i32, bool> = VecMap::new();
    assert_eq!(m.remove(&0), None);
}

#[test]
fn test_empty_entry() {
    let mut m: VecMap<i32, bool> = VecMap::new();
    if m.nth_value(0).is_some() {
        panic!()
    }
    m.insert(0, true);
    assert_eq!(m.len(), 1);
}

#[test]
fn test_lots_of_insertions() {
    let mut m = VecMap::new();

    // Try this a few times to make sure we never screw up the hashmap's
    // internal state.
    let loops = if cfg!(miri) { 2 } else { 10 };
    for _ in 0..loops {
        assert!(m.is_empty());

        let count = if cfg!(miri) { 101 } else { 1001 };

        for i in 1..count {
            assert!(m.insert(i, i).is_none());

            for j in 1..=i {
                let r = m.get(&j);
                assert_eq!(r, Some(&j));
            }

            for j in i + 1..count {
                let r = m.get(&j);
                assert_eq!(r, None);
            }
        }

        for i in count..(2 * count) {
            assert!(!m.contains_key(&i));
        }

        // remove forwards
        for i in 1..count {
            assert!(m.remove(&i).is_some());

            for j in 1..=i {
                assert!(!m.contains_key(&j));
            }

            for j in i + 1..count {
                assert!(m.contains_key(&j));
            }
        }

        for i in 1..count {
            assert!(!m.contains_key(&i));
        }

        for i in 1..count {
            assert!(m.insert(i, i).is_none());
        }

        // remove backwards
        for i in (1..count).rev() {
            assert!(m.remove(&i).is_some());

            for j in i..count {
                assert!(!m.contains_key(&j));
            }

            for j in 1..i {
                assert!(m.contains_key(&j));
            }
        }
    }
}

#[test]
fn test_find_mut() {
    let mut m = VecMap::new();
    assert!(m.insert(1, 12).is_none());
    assert!(m.insert(2, 8).is_none());
    assert!(m.insert(5, 14).is_none());
    let new = 100;
    match m.get_mut(&5) {
        None => panic!(),
        Some(x) => *x = new,
    }
    assert_eq!(m.get(&5), Some(&new));
}

#[test]
fn test_insert_overwrite() {
    let mut m = VecMap::new();
    assert!(m.insert(1, 2).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert!(!m.insert(1, 3).is_none());
    assert_eq!(*m.get(&1).unwrap(), 3);
}

#[test]
fn test_insert_conflicts() {
    let mut m = VecMap::with_capacity(4);
    assert!(m.insert(1, 2).is_none());
    assert!(m.insert(5, 3).is_none());
    assert!(m.insert(9, 4).is_none());
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&1).unwrap(), 2);
}

#[test]
fn test_conflict_remove() {
    let mut m = VecMap::with_capacity(4);
    assert!(m.insert(1, 2).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert!(m.insert(5, 3).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert!(m.insert(9, 4).is_none());
    assert_eq!(*m.get(&1).unwrap(), 2);
    assert_eq!(*m.get(&5).unwrap(), 3);
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert!(m.remove(&1).is_some());
    assert_eq!(*m.get(&9).unwrap(), 4);
    assert_eq!(*m.get(&5).unwrap(), 3);
}

#[test]
fn test_is_empty() {
    let mut m = VecMap::with_capacity(4);
    assert!(m.insert(1, 2).is_none());
    assert!(!m.is_empty());
    assert!(m.remove(&1).is_some());
    assert!(m.is_empty());
}

#[test]
fn test_remove() {
    let mut m = VecMap::new();
    m.insert(1, 2);
    assert_eq!(m.remove(&1), Some(2));
    assert_eq!(m.remove(&1), None);
}

#[test]
fn test_iterate() {
    let mut m = VecMap::with_capacity(4);
    for i in 0..32 {
        assert!(m.insert(i, i * 2).is_none());
    }
    assert_eq!(m.len(), 32);

    let mut observed: u32 = 0;

    for (k, v) in m.as_slice() {
        assert_eq!(*v, *k * 2);
        observed |= 1 << *k;
    }
    assert_eq!(observed, 0xFFFF_FFFF);
}

#[test]
fn test_find() {
    let mut m = VecMap::new();
    assert!(m.get(&1).is_none());
    m.insert(1, 2);
    match m.get(&1) {
        None => panic!(),
        Some(v) => assert_eq!(*v, 2),
    }
}
#[test]
fn test_remove_entry() {
    let mut m = std::collections::HashMap::new();
    m.insert(1, 2);
    assert_eq!(m.remove_entry(&1), Some((1, 2)));
    assert_eq!(m.remove(&1), None);
}

/*
#[test]
fn test_clone() {
    let mut m = VecMap::new();
    assert_eq!(m.len(), 0);
    assert!(m.insert(1, 2).is_none());
    assert_eq!(m.len(), 1);
    assert!(m.insert(2, 4).is_none());
    assert_eq!(m.len(), 2);
    let m2 = m.clone();
    assert_eq!(*m2.get(&1).unwrap(), 2);
    assert_eq!(*m2.get(&2).unwrap(), 4);
    assert_eq!(m2.len(), 2);
}

#[test]
fn test_empty_iter() {
    let mut m: VecMap<i32, bool> = VecMap::new();
    assert_eq!(m.drain().next(), None);
    assert_eq!(m.keys().next(), None);
    assert_eq!(m.values().next(), None);
    assert_eq!(m.values_mut().next(), None);
    assert_eq!(m.iter().next(), None);
    assert_eq!(m.iter_mut().next(), None);
    assert_eq!(m.len(), 0);
    assert!(m.is_empty());
    assert_eq!(m.into_iter().next(), None);
}

    #[test]
    fn test_keys() {
        let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
        let map: VecMap<_, _> = pairs.into_iter().collect();
        let keys: Vec<_> = map.keys().cloned().collect();
        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_values() {
        let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
        let map: VecMap<_, _> = pairs.into_iter().collect();
        let values: Vec<_> = map.values().cloned().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

    #[test]
    fn test_values_mut() {
        let pairs = [(1, 1), (2, 2), (3, 3)];
        let mut map: VecMap<_, _> = pairs.into_iter().collect();
        for value in map.values_mut() {
            *value = (*value) * 2
        }
        let values: Vec<_> = map.values().cloned().collect();
        assert_eq!(values.len(), 3);
        assert!(values.contains(&2));
        assert!(values.contains(&4));
        assert!(values.contains(&6));
    }

    #[test]
    fn test_into_keys() {
        let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
        let map: VecMap<_, _> = pairs.into_iter().collect();
        let keys: Vec<_> = map.into_keys().collect();

        assert_eq!(keys.len(), 3);
        assert!(keys.contains(&1));
        assert!(keys.contains(&2));
        assert!(keys.contains(&3));
    }

    #[test]
    fn test_into_values() {
        let pairs = [(1, 'a'), (2, 'b'), (3, 'c')];
        let map: VecMap<_, _> = pairs.into_iter().collect();
        let values: Vec<_> = map.into_values().collect();

        assert_eq!(values.len(), 3);
        assert!(values.contains(&'a'));
        assert!(values.contains(&'b'));
        assert!(values.contains(&'c'));
    }

#[test]
fn test_eq() {
    let mut m1 = VecMap::new();
    m1.insert(1, 2);
    m1.insert(2, 3);
    m1.insert(3, 4);

    let mut m2 = VecMap::new();
    m2.insert(1, 2);
    m2.insert(2, 3);

    assert!(m1 != m2);

    m2.insert(3, 4);

    assert_eq!(m1, m2);
}*/

#[cfg(feature = "macros")]
pub mod macro_tests {
    //the as binding is to simulate the name of the crate
    //be called in a consuming crate. linear_collections otherwise will not resolve
    use crate as linear_collections;
    use crate::array::map::ArrayMap;
    #[cfg(test)]
    use macros::array_map;
    #[test]
    fn array_map_empty() {
        array_map!([("k", "v"),]);
    }

    #[test]
    fn array_map_one() {}

    #[test]
    fn array_map_many() {}

    #[test]
    #[should_panic]
    fn array_map_one_duplicate() {}

    #[test]
    #[should_panic]
    fn array_map_many_duplicates() {}

    #[test]
    #[should_panic]
    ///numeric types can have suffixes, and these should not affect parsing
    fn array_map_numeric_with_type() {}

    #[test]
    #[should_panic]
    fn array_map_numeric_with_seperator() {}

    #[test]
    fn array_map_string_literal_with_prefix() {}
}
