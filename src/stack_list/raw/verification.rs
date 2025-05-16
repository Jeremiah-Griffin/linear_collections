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
fn clear_to_within_range(){
    let array = [0,1,2,3,4];

    let length = array.len();

    let mut list = RawStackList::from_array(array);

    let end = kani::any();

    kani::assume(end <= length);

    unsafe{list.clear_to(end)};

}

#[kani::proof]
fn get(){
    let inserted: u8 = kani::any();

    let inserted_idx: usize = kani::any();


    let mut list = RawStackList::<u8,10>::uninit();

    //Note that it *is not* UB to write to a MaybeUninit that is init or to write to
    //arbirtrary locations within a raw stack list.

    unsafe { list.insert_at(inserted_idx, inserted)};


    let got = unsafe { list.get(inserted_idx)};

    assert_eq!(*got, inserted);        
}

