use std::mem::MaybeUninit;
use crate::verification_utils::Dropper;
use crate::stack_list::raw::RawStackList;
use crate::stack_list::StackList;

use kani::{proof, loop_invariant};
#[proof]
///Soundness
fn new(){
    StackList::<u8, 5>::new();
}


#[proof]
fn clear(){
    let length: usize = kani::any();
    let mut list = StackList{
        //Array is any valid [T;CAPACITY]. This ensures the internal
        // MaybeUninit are init and valid memory.
        raw: RawStackList::<u8,5>::from_array(kani::any()),
        length,
    };

    assert_eq!(length, 0);    
}
#[proof]
fn len(){
    let list: StackList<u8, 5> = kani::any();

    assert_eq!(list.len(), list.len());
}

#[proof]
fn from_array(){
    const CAPACITY: usize = 5;
    let list: StackList<u8, CAPACITY> = StackList::from_array(kani::any());

    assert_eq!(list.len(), CAPACITY);
}

#[proof]
///All calls to pop while a `StackList` is non-empty should return `Some`.
///All calls  made after the stack list have been emptied should return `None`.
fn pop(){
    const CAPACITY: usize = 5;

    let arr: [u8;CAPACITY] = kani::any();

    let mut list: StackList<u8, CAPACITY> = StackList::from_array(arr.clone());


    let mut i: usize = CAPACITY;

    //All calls to pop should return the same as indexing from the back of an array.
    while i > 0 {
        assert_eq!(list.pop().unwrap(), *arr.get(i).unwrap());
        i -= 1;
    }


    //All succesive calls to pop should return none.
    //Bound by CAPACITY to satisfy kani.
    let mut i = 0;


    while i <= CAPACITY{
        assert!(list.pop().is_none());
        i += 1;
    }    
}

#[proof]
fn remove(){
    const CAPACITY: usize = 5;

    let num_removals: usize = kani::any();


    kani::assume(num_removals <= CAPACITY);

    let arr: [u8;CAPACITY] = kani::any();


    let mut vec = std::vec::Vec::from(arr);
    let mut list = StackList::from_array(arr);

    let mut i = 0;
    #[loop_invariant(i < CAPACITY)]
    while i < num_removals{
        let index: usize = kani::any();
        kani::assume(index < list.len());

        let from_list = list.remove(index);
        assert!(from_list.is_some());
        let from_list = from_list.unwrap();
        let from_vec = vec.remove(index);

        assert_eq!(from_list, from_vec);

        i += 1;
    }
}
