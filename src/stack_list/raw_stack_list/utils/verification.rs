use super::RawStackList;
use kani::proof;
#[proof]
///Any insertion at any index < LENGTH must be retrievable.
pub fn get(){
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

    let ref list = list;


    let got = unsafe { crate::stack_list::raw_stack_list::get!(list, inserted_idx)};

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


    let mut list = RawStackList::<u8,CAPACITY>::uninit();

    //Note that it *is not* UB to write to a MaybeUninit that is initialized or to write to
    //arbirtrary locations within a raw stack list, just ill advised.
    // Inserting a a random point in the list is a-ok.
    unsafe { list.insert_at(inserted_idx, inserted)};

    let ref mut list = list;


    let got = unsafe { crate::stack_list::raw_stack_list::get_mut!(list, inserted_idx)};

    assert_eq!(*got, inserted);        
}
