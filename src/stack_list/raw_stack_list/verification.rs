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
