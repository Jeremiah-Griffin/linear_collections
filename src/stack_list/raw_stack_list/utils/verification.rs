use super::RawStackList;
use crate::stack_list::raw_stack_list;
use crate::stack_list::raw_stack_list::utils;
use std::num::NonZero;
use kani::proof;

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

    unsafe{utils::clear_to(&mut list, end)};
}
#[proof]
///Any insertion at any index < LENGTH must be retrievable.
pub fn get(){
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
    unsafe { utils::insert_at(list, inserted_idx, inserted)};



    let got = unsafe { utils::get(list, inserted_idx)};

    assert_eq!(*got, inserted);        
}

#[proof]
///Any insertion at any index < LENGTH must be retrievable.
pub fn get_mut(){
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
    unsafe{ utils::insert_at(list, inserted_idx, inserted)};



    let got = unsafe { utils::get_mut(list, inserted_idx)};

    assert_eq!(*got, inserted);        
}

#[proof]
///Insertation at any idx less than LENGTH is not UB
fn insert_at(){

    const LENGTH: usize = 10;
    let index = kani::any();

    kani::assume(LENGTH > index);

    let value = kani::any();

    let ref mut list  = RawStackList::<u8,LENGTH>::uninit(); 

    unsafe {utils::insert_at(list, index, value)};

}

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


    let got = unsafe { utils::remove(list, inserted_idx, CAPACITY)};

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

    assert_eq!(unsafe{utils::remove(&mut list, removal_index, LENGTH)}, vec.remove(removal_index));

    ///iter_to iterates 
    assert_eq!(vec, unsafe{list.iter_to::<{LENGTH - 1}>()}.map(|i| *i).collect::<Vec<u8>>());
}
