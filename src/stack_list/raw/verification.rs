use super::RawStackList;
use std::mem::MaybeUninit;
#[kani::proof]
fn uninit(){
    let list: RawStackList<u8,10> = RawStackList::uninit();
}


#[kani::proof]
///Any array passed should be valid.
/// We can't place Kany::any() in type position, so we have to be a bit less general.
fn from_array(){
    RawStackList::<u8,2>::from_array([kani::any(), kani::any()]);
}


#[kani::proof]
fn from_maybe_uninit(){
    RawStackList::<u8, 2>::from_maybe_uninit([MaybeUninit::new(kani::any()), MaybeUninit::uninit()]);
}

#[kani::proof]
//we don't do any branching depending on `limit` so this should be fine.
#[kani::unwind(10)]
///Every value less than or equal to the lenght of the RawStackList must be safe
/// to clear to.
fn clear_to(){
    let array: [u8; 5] = kani::any();

    let length = array.len();

    let mut list = RawStackList::from_array(array);

    let end = kani::any();

    kani::assume(end <= length);

    unsafe{list.clear_to(end)};

}

#[kani::proof]
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

#[kani::proof]
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


#[kani::proof]
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

#[kani::proof]
fn iter_to(){todo!()}

#[kani::proof]
fn iter_mut_to(){todo!()}
*/

#[kani::proof]
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


#[kani::proof]
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
