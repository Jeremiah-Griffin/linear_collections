use super::RawStackList;
use crate::stack_list::raw_stack_list;
use kani::proof;

use std::{mem::MaybeUninit, num::NonZero};
#[proof]
fn uninit() {
    let list: RawStackList<u8, 10> = RawStackList::uninit();
}

#[proof]
fn default() {
    let list: RawStackList<u8, 10> = RawStackList::default();
}

#[proof]
///Any array passed should be valid.
/// We can't place Kany::any() in type position, so we have to be a bit less general.
fn from_array() {
    RawStackList::<u8, 10>::from_array([kani::any(); 10]);
}

#[proof]
fn from_maybe_uninit() {
    RawStackList::<u8, 10>::from_maybe_uninit([MaybeUninit::new(kani::any()); 10]);
}

#[proof]
fn as_slice() {
    let list: RawStackList<u8, 5> = kani::any();

    let slice = list.as_slice();
}

#[proof]
fn as_mut_slice() {
    let mut list: RawStackList<u8, 5> = kani::any();

    let slice = list.as_mut_slice();
}

#[proof]
///Limit is bounded to be > 0, this is the lowest we can test.
fn clear_to_one() {
    let mut list: RawStackList<u8, 5> = kani::any();

    unsafe { list.clear_to::<1>() };
}

#[proof]
fn clear_to_middle() {
    let mut list: RawStackList<u8, 5> = kani::any();

    unsafe { list.clear_to::<3>() };
}

#[proof]
fn clear_to_capacity() {
    let mut list: RawStackList<u8, 5> = kani::any();
    unsafe { list.clear_to::<5>() };
}

#[proof]
fn get_zero() {
    let array: [u8; 5] = kani::any();
    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));
    let mut vec = Vec::from(array);

    const INDEX: usize = 0;
    assert_eq!(unsafe { list.get::<INDEX>() }, vec.get(INDEX).unwrap());
}

#[proof]
fn get_middle() {
    let array: [u8; 5] = kani::any();
    let list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));
    let mut vec = Vec::from(array);

    const INDEX: usize = 2;
    assert_eq!(unsafe { list.get::<INDEX>() }, vec.get(INDEX).unwrap());
}

#[proof]
///Bounded b
fn get_end() {
    let array: [u8; 5] = kani::any();
    let list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));
    let mut vec = Vec::from(array);

    const INDEX: usize = 4;
    assert_eq!(unsafe { list.get::<INDEX>() }, vec.get(INDEX).unwrap());
}

#[proof]
fn get_mut_zero() {
    let array: [u8; 5] = kani::any();
    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));
    let mut vec = Vec::from(array);

    const INDEX: usize = 0;
    assert_eq!(
        unsafe { list.get_mut::<INDEX>() },
        vec.get_mut(INDEX).unwrap()
    );
}

#[proof]
fn get_mut_middle() {
    let array: [u8; 5] = kani::any();
    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));
    let mut vec = Vec::from(array);

    const INDEX: usize = 2;
    assert_eq!(
        unsafe { list.get_mut::<INDEX>() },
        vec.get_mut(INDEX).unwrap()
    );
}

#[proof]
///Bounded b
fn get_mut_end() {
    let array: [u8; 5] = kani::any();
    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));
    let mut vec = Vec::from(array);

    const INDEX: usize = 4;
    assert_eq!(
        unsafe { list.get_mut::<INDEX>() },
        vec.get_mut(INDEX).unwrap()
    );
}

#[proof]
fn insert_at_zero() {
    let mut list: RawStackList<u8, 5> = RawStackList::uninit();
    list.insert_at::<0>(kani::any());
}

#[proof]
fn insert_at_middle() {
    let mut list: RawStackList<u8, 5> = RawStackList::uninit();
    list.insert_at::<2>(kani::any());
}

#[proof]
fn insert_at_end() {
    let mut list: RawStackList<u8, 5> = RawStackList::uninit();
    list.insert_at::<4>(kani::any());
}

#[proof]
fn remove_start() {
    const LENGTH: usize = 5;
    let array: [u8; LENGTH] = kani::any();

    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));

    const INDEX: usize = 0;

    assert_eq!(unsafe { list.remove::<INDEX>(LENGTH) }, array[INDEX]);
}

#[proof]
fn remove_middle() {
    const LENGTH: usize = 5;

    let array: [u8; LENGTH] = kani::any();

    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));

    const INDEX: usize = 2;

    assert_eq!(unsafe { list.remove::<INDEX>(LENGTH) }, array[INDEX]);
}

#[proof]
fn remove_end() {
    const LENGTH: usize = 5;

    let array: [u8; LENGTH] = kani::any();

    let mut list: RawStackList<u8, 5> = RawStackList::from_array(Clone::clone(&array));

    const INDEX: usize = 4;

    assert_eq!(unsafe { list.remove::<INDEX>(LENGTH) }, array[INDEX]);
}

/*
///TODO: verification
Temporarily exempted from proofs
#[proof]
fn iter(){

const LENGTH:usize = 5;
let arr: [u8;LENGTH] = [0,1,2,3,4];

let vec = std::vec::Vec::from(arr);

let list = RawStackList::from_array(arr);

let to: usize = kani::any();

kani::assume(to < LENGTH);

assert_eq!(vec[0..to].into_iter().map(|i| *i).collect::<Vec<u8>>(), unsafe{list.iter_to(to).map(|i| *i).collect::<Vec<u8>>()});
}
///TODO: verification
Temporarily exempted from proofs
#[proof]
fn iter_mut_to(){todo!()}
*/
