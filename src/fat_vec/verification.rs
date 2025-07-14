use crate::fat_vec::FatVec;
use crate::list::List;
use kani::proof;
#[proof]
fn new() {
    FatVec::<u8, 5>::new();
}

#[proof]
fn capacity() {
    const STACK_CAPACITY: usize = 5;
    let list: FatVec<u8, STACK_CAPACITY> = kani::any();

    assert_eq!(list.capacity(), list.vec.capacity() + STACK_CAPACITY);
}

#[proof]
fn clear() {
    let mut fat_vec: FatVec<u8, 5> = kani::any();

    fat_vec.clear();

    assert_eq!(fat_vec.len(), 0);
}

#[proof]
fn is_empty() {
    let mut list: FatVec<u8, 5> = kani::any();

    assert_eq!(list.is_empty(), list.len() == 0);
}

/*
///TODO: excluded due to poor performance
#[proof]
fn iter() {
    let arr: [u8; 2] = kani::any();
    let mut list: FatVec<u8, 1> = FatVec::new();

    let mut i = 0;
    //#[kani::loop_invariant(i <= 2)]
    while i < arr.len() {
        list.push(i as u8).unwrap();
        i += 1;
    }

    assert_eq!(
        list.iter().map(|i| *i).collect::<Vec<u8>>(),
        Vec::from(Clone::clone(&arr))
    );
}
///TODO: excluded due to poor performance
#[proof]
fn iter_mut() {
    let arr: [u8; 10] = kani::any();
    let mut list: FatVec<u8, 5> = FatVec::new();

    for i in arr {
        list.push(i).unwrap();
    }

    assert_eq!(
        list.iter_mut().map(|i| *i).collect::<Vec<u8>>(),
        Vec::from(Clone::clone(&arr))
    );
}

///TODO: excluded due to poor performance
#[proof]
fn len() {
    let len: usize = kani::any();
    let mut list: FatVec<usize, 2> = kani::any();

    kani::assume(len < 5);


    for i in 0..len{
        list.push(i);
    }

    assert_eq!(list.len(), len);
}
///TODO: excluded due to poor performance
#[proof]
fn pop() {
    let len: usize = kani::any();
    let mut list = FatVec::<u8, 2>::new();
    kani::assume(len < 5);

    for i in 0..len {
        list.push(i as u8).unwrap();
    }

    let mut vec = Vec::with_capacity(list.len());

    for i in list.iter() {
        vec.push(*i);
    }

    for i in 0..len {
        assert_eq!(list.pop(), vec.pop())
    }

    ///all successive calls from pop should return None.
    assert_eq!(list.pop(), None);
}
*/

#[proof]
fn remaining_capacity() {
    const STACK_CAPACITY: usize = 5;
    let mut list: FatVec<u8, STACK_CAPACITY> = FatVec::new();

    assert_eq!(list.remaining_capacity(), STACK_CAPACITY);

    let list: FatVec<u8, STACK_CAPACITY> = kani::any();

    assert_eq!(
        list.remaining_capacity(),
        (list.vec.capacity() + STACK_CAPACITY) - list.len()
    );
}

#[proof]
fn push() {
    let mut list: FatVec<usize, 2> = FatVec::new();

    let length: usize = 5;
    for i in 0..length {
        list.push(i).unwrap();
    }

    let idx = kani::any();

    kani::assume(idx < length);

    assert_eq!(list.get(idx), Some(&idx));

    ///all calls list.get(len + 1) must return None
    let after_idx: usize = kani::any();

    kani::assume(after_idx >= length);

    assert_eq!(list.get(after_idx), None)
}

/*
///TODO: Performance
#[proof]
fn eq() {
    let a: FatVec<u8, 1> = kani::any();
    let b: FatVec<u8, 1> = kani::any();

    if a == b {
        let a_len = a.len();
        let b_len = b.len();
        //assert_eq!(a_len, b_len);

        kani::assume(a_len < 3);
        kani::assume(b_len < 3);
        for i in 0..a_len {
            assert_eq!(a.get(i), b.get(i))
        }
    }
}
*/
