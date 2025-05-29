use super::RawStackList;
use crate::stack_list::raw_stack_list;
use kani::proof;

use std::{num::NonZero,mem::MaybeUninit};
#[proof]
fn uninit(){
    let list: RawStackList<u8,10> = RawStackList::uninit();
}


#[proof]
///Any array passed should be valid.
/// We can't place Kany::any() in type position, so we have to be a bit less general.
fn from_array(){
    RawStackList::<u8,2>::from_array([kani::any(), kani::any()]);
}


#[proof]
fn from_maybe_uninit(){
    RawStackList::<u8, 2>::from_maybe_uninit([MaybeUninit::new(kani::any()), MaybeUninit::uninit()]);
}

/*
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

Temporarily exempted from proofs
#[proof]
fn iter_mut_to(){todo!()}
*/

#[proof]
fn remove_correct_value(){
 let inserted: u8 = kani::any();

    let inserted_idx: usize = kani::any();

    const CAPACITY: usize = 10;

    //The length of  collection of nonzero size is always 1 greater than the
    //index of the final element.
    kani::assume(inserted_idx < CAPACITY);


    let ref mut list = RawStackList::<u8,CAPACITY>::uninit();

    //Note that it *is not* UB to write to a MaybeUninit that is initialized or to write to
    //arbirtrary locations within a raw stack list, just ill advised.
    // Inserting a a random point in the list is a-ok.
    unsafe { raw_stack_list::insert_at!(list, inserted_idx, inserted)};


    let got = unsafe { list.remove(inserted_idx, CAPACITY)};

    assert_eq!(got, inserted);        
}


#[proof]
///All values right of the removed item must be shifted left to remain contiguous.
fn remove_shifts_left(){

    const LENGTH: usize = 5;

    let removal_index: usize = kani::any();

    let arr: [u8; LENGTH] = kani::any();

    kani::assume(removal_index < LENGTH);
    

    ///std vec shifts left, so we'll use that.
    let mut vec = std::vec::Vec::from(arr);

    let mut list = RawStackList::from_array(arr);

    assert_eq!(unsafe{list.remove(removal_index, LENGTH)}, vec.remove(removal_index));

    ///iter_to iterates 
    assert_eq!(vec, unsafe{list.iter_to::<{LENGTH - 1}>().map(|i| *i).collect::<Vec<u8>>());    
}
