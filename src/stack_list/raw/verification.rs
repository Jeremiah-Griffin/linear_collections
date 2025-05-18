use super::RawStackList;
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

#[proof]
//we don't do any branching depending on `limit` so this should be fine.
///Every value less than or equal to the lenght of the RawStackList must be safe
/// to clear to.
fn clear_to(){
    let array: [u8; 5] = kani::any();

    let length = array.len();

    let mut list = RawStackList::from_array(array);

    let end: NonZero<usize> = kani::any();

    kani::assume(end.get() <= length);

    unsafe{list.clear_to(end)};
}

#[proof]
///Any insertion at any index < LENGTH must be retrievable.
fn get(){
    let inserted: u8 = kani::any();

    let inserted_idx: usize = kani::any();

    const CAPACITY: usize = 10;

    //The length of  collection of nonzero size is always 1 greater than the
    //index of the final element.
    kani::assume(inserted_idx < CAPACITY);


    let mut list = RawStackList::<u8,CAPACITY>::uninit();

    //Note that it *is not* UB to write to a MaybeUninit that is initialized or to write to
    //arbirtrary locations within a raw stack list, just ill advised.
    // Inserting a a random point in the list is a-ok.
    unsafe { list.insert_at(inserted_idx, inserted)};


    let got = unsafe { list.get(inserted_idx)};

    assert_eq!(*got, inserted);        
}

#[proof]
///Any insertion at any index < LENGTH must be retrievable.
fn get_mut(){
    let inserted: u8 = kani::any();

    let inserted_idx: usize = kani::any();

    const LENGTH: usize = 10;

    //The length of  collection of nonzero size is always 1 greater than the
    //index of the final element.
    kani::assume(LENGTH > inserted_idx);


    let mut list = RawStackList::<u8,LENGTH>::uninit();

    //Note that it *is not* UB to write to a MaybeUninit that is initialized or to write to
    //arbirtrary locations within a raw stack list, just ill advised.
    // Inserting a a random point in the list is a-ok.
    unsafe { list.insert_at(inserted_idx, inserted)};


    let got = unsafe { list.get_mut(inserted_idx)};

    assert_eq!(*got, inserted);        
}


#[proof]
///Insertation at any idx less than LENGTH is not UB
fn insert_at(){

    const LENGTH: usize = 10;
    let index = kani::any();

    kani::assume(LENGTH > index);

    let value = kani::any();

    let mut list  = RawStackList::<u8,LENGTH>::uninit(); 

    unsafe {list.insert_at(index, value)};

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


    let mut list = RawStackList::<u8,CAPACITY>::uninit();

    //Note that it *is not* UB to write to a MaybeUninit that is initialized or to write to
    //arbirtrary locations within a raw stack list, just ill advised.
    // Inserting a a random point in the list is a-ok.
    unsafe { list.insert_at(inserted_idx, inserted)};


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
    assert_eq!(vec, unsafe{list.iter_to(LENGTH - 1)}.map(|i| *i).collect::<Vec<u8>>());    
}
