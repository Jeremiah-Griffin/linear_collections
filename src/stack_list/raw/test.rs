use std::{num::NonZero, ops::{Deref, DerefMut}, sync::{Arc, Mutex}};
use crate::verification_utils::{Dropper};
use super::RawStackList;

#[test]
fn remove_front_shifts_left() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawStackList::<&str, 5>::uninit();

    unsafe {
        arrvec.insert_at(0, one);
        arrvec.insert_at(1, two);
        arrvec.insert_at(2, three);
        arrvec.insert_at(3, four);
        arrvec.insert_at(4, five);

        let (from_vec, from_arr) = (vec.remove(0), arrvec.remove(0, 5));

        assert_eq!(from_vec, from_arr);

        let remaining = arrvec.iter_to(4).map(|t| *t).collect::<Vec<&str>>();

        assert_eq!(vec, remaining);
    }
}

#[test]
fn remove_mid_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawStackList::<&str, 5>::uninit();

    unsafe {
        arrvec.insert_at(0, one);
        arrvec.insert_at(1, two);
        arrvec.insert_at(2, three);
        arrvec.insert_at(3, four);
        arrvec.insert_at(4, five);

        let (from_vec, from_arr) = (vec.remove(2), arrvec.remove(2, 5));

        assert_eq!(from_vec, from_arr);

        let remaining = arrvec.iter_to(4).map(|t| *t).collect::<Vec<&str>>();

        assert_eq!(vec, remaining);
    }
}

#[test]
fn remove_end_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawStackList::<&str, 5>::uninit();

    unsafe {
        arrvec.insert_at(0, one);
        arrvec.insert_at(1, two);
        arrvec.insert_at(2, three);
        arrvec.insert_at(3, four);
        arrvec.insert_at(4, five);

        let (from_vec, from_arr) = (vec.remove(4), arrvec.remove(4, 5));

        assert_eq!(from_vec, from_arr);

        let remaining = arrvec.iter_to(4).map(|t| *t).collect::<Vec<&str>>();

        assert_eq!(vec, remaining);
    }
}

/*
///I dont really know how to test this without internal length trackign
#[test]
pub fn clear_is_clear() {
    let mut svec = RawStackList::from_array(["one", "two", "three", "four", "five"].clone());

    unsafe { svec.clear_to(5) };

    assert_eq!(unsafe { svec.iter_to(5) }.nth(0), None);
}*/

///Used as a test as kani won't verify the FFI stuff in Dropper.
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
    
    let mut svec: RawStackList<Dropper, LENGTH> = RawStackList::from_array([
        zeroth, first, second, third, fourth,
    ]);
    let limit = NonZero::new(LENGTH).unwrap();
    unsafe { svec.clear_to(limit) };



    //clearing should drop all elements
    assert_eq!(zeroth_clone.dropped(), limit.get() > 0);
    assert_eq!(first_clone.dropped(), limit.get() > 1);
    assert_eq!(second_clone.dropped(), limit.get() > 2);
    assert_eq!(third_clone.dropped(), limit.get() > 3);
    assert_eq!(fourth_clone.dropped(), limit.get() > 4);
}


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
