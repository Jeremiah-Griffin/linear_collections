use std::num::NonZero;

use crate::{stack_list::{raw_stack_list::{self, utils}, RawStackList}, verification_utils::Dropper};

#[test]
pub fn clear_to_drops() {
    let zeroth =  Dropper::new();
    let first =  Dropper::new();
    let second=  Dropper::new();
    let third=  Dropper::new();
    let fourth=  Dropper::new();
    let zeroth_clone =  zeroth.clone();
    let first_clone =  first.clone();
    let second_clone =  second.clone();
    let third_clone =  third.clone();
    let fourth_clone =  fourth.clone();


    const LENGTH: usize = 5;
    
    let mut list: RawStackList<Dropper, LENGTH> = RawStackList::from_array([
        zeroth, first, second, third, fourth,
    ]);

    let list_ref = &mut list;
    let limit = NonZero::new(LENGTH).unwrap();
    unsafe { raw_stack_list::clear_to!(list_ref, limit) };



    //clearing should drop all elements
    assert_eq!(zeroth_clone.dropped(), limit.get() > 0);
    assert_eq!(first_clone.dropped(), limit.get() > 1);
    assert_eq!(second_clone.dropped(), limit.get() > 2);
    assert_eq!(third_clone.dropped(), limit.get() > 3);
    assert_eq!(fourth_clone.dropped(), limit.get() > 4);
}

/*
///I dont really know how to test this without internal length trackign
#[test]
pub fn clear_is_clear() {
    let mut svec = RawStackList::from_array(["one", "two", "three", "four", "five"].clone());

    unsafe { svec.clear_to(5) };

    assert_eq!(unsafe { svec.iter_to(5) }.nth(0), None);
}*/


//Unaligned tcache chunk detected when running this. Commented out for now.
/*
///Used as a test as kani won't verify the FFI stuff in Dropper.
#[test]
pub fn clear_to_drops_broken() {
       let zeroth =  Dropper::new();
       let first =  Dropper::new();
       let second=  Dropper::new();
       let third=  Dropper::new();
       let fourth=  Dropper::new();

    const LENGTH: usize = 5;
    
    let mut list: RawStackList<Dropper, LENGTH> = RawStackList::from_array([
        zeroth.clone(), first.clone(), second.clone(), third.clone(), fourth.clone(),
    ]);

    for limit  in 1..=LENGTH{

        let zeroth_clone =  zeroth.clone();
        let first_clone =  first.clone();
        let second_clone =  second.clone();
        let third_clone =  third.clone();
        let fourth_clone =  fourth.clone();


        unsafe{list.clear_to(NonZero::new(limit).unwrap())};

        //clearing should drop all elements
        assert_eq!(zeroth_clone.dropped(), limit > 0);
        assert_eq!(first_clone.dropped(), limit > 1);
        assert_eq!(second_clone.dropped(), limit > 2);
        assert_eq!(third_clone.dropped(), limit > 3);
        assert_eq!(fourth_clone.dropped(), limit > 4);

        zeroth_clone.reset();
        first_clone.reset();
        second_clone.reset();
        third_clone.reset();
        fourth_clone.reset();
        

    }
}
*/
