use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use super::{RawStackList, StackList};

///Just a helper to test drop behavior
struct Dropper {
    reference: Arc<Mutex<bool>>,
}

impl Dropper {
    pub fn new(reference: Arc<Mutex<bool>>) -> Self {
        Self { reference }
    }
}

impl Drop for Dropper {
    fn drop(&mut self) {
        *self.reference.lock().unwrap().deref_mut() = true;
    }
}

#[test]
fn dropper_checks_drop() {
    let was_dropped = Arc::new(Mutex::new(false));

    assert_eq!(*was_dropped.lock().unwrap().deref(), false);

    let dropper = Dropper::new(was_dropped.clone());

    drop(dropper);

    assert_eq!(*was_dropped.lock().unwrap().deref(), true);
}

#[test]
fn raw_stack_list_remove_front_is_same_as_vec() {
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
fn raw_stack_list_remove_mid_is_same_as_vec() {
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
fn raw_stack_list_remove_end_is_same_as_vec() {
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
pub fn raw_stack_list_clear_is_clear() {
    let mut svec = RawStackList::from_array(["one", "two", "three", "four", "five"].clone());

    unsafe { svec.clear_to(5) };

    assert_eq!(unsafe { svec.iter_to(5) }.nth(0), None);
}*/

#[test]
pub fn raw_stack_list_clear_to_drops() {
    //If drop tracking works we can run the actual test.
    let first_was_dropped = Arc::new(Mutex::new(false));
    let second_was_dropped = Arc::new(Mutex::new(false));
    let third_was_dropped = Arc::new(Mutex::new(false));
    let fourth_was_dropped = Arc::new(Mutex::new(false));
    let fifth_was_dropped = Arc::new(Mutex::new(false));

    let mut svec = RawStackList::from_array([
        Dropper::new(first_was_dropped.clone()),
        Dropper::new(second_was_dropped.clone()),
        Dropper::new(third_was_dropped.clone()),
        Dropper::new(fourth_was_dropped.clone()),
        Dropper::new(fifth_was_dropped.clone()),
    ]);

    //Adding elements should never dorop them.
    assert_eq!(*first_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*second_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*third_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*fourth_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*fifth_was_dropped.lock().unwrap().deref(), false);

    unsafe { svec.clear_to(5) };

    //clearing should drop all elements
    assert_eq!(*first_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*second_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*third_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*fourth_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*fifth_was_dropped.lock().unwrap().deref(), true);
}

#[test]
fn stack_list_remove_start_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = StackList::<&str, 5>::new();

    arrvec.push(one).unwrap();
    arrvec.push(two).unwrap();
    arrvec.push(three).unwrap();
    arrvec.push(four).unwrap();
    arrvec.push(five).unwrap();

    let (from_vec, from_arr) = (vec.remove(0), arrvec.remove(0).unwrap());

    assert_eq!(from_vec, from_arr);

    let remaining = arrvec.iter().map(|t| *t).collect::<Vec<&str>>();

    assert_eq!(vec, remaining);
}

#[test]
fn stack_list_remove_mid_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = StackList::<&str, 5>::new();

    arrvec.push(one).unwrap();
    arrvec.push(two).unwrap();
    arrvec.push(three).unwrap();
    arrvec.push(four).unwrap();
    arrvec.push(five).unwrap();

    let (from_vec, from_arr) = (vec.remove(2), arrvec.remove(2).unwrap());

    assert_eq!(from_vec, from_arr);

    let remaining = arrvec.iter().map(|t| *t).collect::<Vec<&str>>();

    assert_eq!(vec, remaining);
}

#[test]
fn stack_list_remove_end_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = StackList::<&str, 5>::new();

    arrvec.push(one).unwrap();
    arrvec.push(two).unwrap();
    arrvec.push(three).unwrap();
    arrvec.push(four).unwrap();
    arrvec.push(five).unwrap();

    let (from_vec, from_arr) = (vec.remove(4), arrvec.remove(4).unwrap());

    assert_eq!(from_vec, from_arr);

    let remaining = arrvec.iter().map(|t| *t).collect::<Vec<&str>>();

    assert_eq!(vec, remaining);
}

#[test]
pub fn stack_list_get_0th_empty() {
    let no_stack = StackList::<&str, 0>::new();
    let some_stack = StackList::<&str, 5>::new();

    assert_eq!(no_stack.get(0), None);
    assert_eq!(some_stack.get(0), None);
}

#[test]
pub fn stack_list_get_1th_empty() {
    let no_stack = StackList::<&str, 0>::new();
    let some_stack = StackList::<&str, 5>::new();

    assert_eq!(no_stack.get(1), None);
    assert_eq!(some_stack.get(1), None);
}

#[test]
pub fn stack_list_eq_is_eq() {
    const ARRAY: [usize; 5] = [5; 5];
    assert_eq!(
        StackList::<usize, 5>::from_array(ARRAY),
        StackList::<usize, 5>::from_array(ARRAY)
    )
}

#[test]
fn stack_list_remove_front_is_same_as_vec() {
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
pub fn stack_list_clear_is_clear() {
    let mut svec = StackList::from_array(["one", "two", "three", "four", "five"].clone());

    svec.clear();

    assert_eq!(svec.iter().nth(0), None);
}

#[test]
pub fn stack_list_clear_to_drops() {
    //If drop tracking works we can run the actual test.
    let first_was_dropped = Arc::new(Mutex::new(false));
    let second_was_dropped = Arc::new(Mutex::new(false));
    let third_was_dropped = Arc::new(Mutex::new(false));
    let fourth_was_dropped = Arc::new(Mutex::new(false));
    let fifth_was_dropped = Arc::new(Mutex::new(false));

    let mut svec = StackList::from_array([
        Dropper::new(first_was_dropped.clone()),
        Dropper::new(second_was_dropped.clone()),
        Dropper::new(third_was_dropped.clone()),
        Dropper::new(fourth_was_dropped.clone()),
        Dropper::new(fifth_was_dropped.clone()),
    ]);

    //Adding elements should never dorop them.
    assert_eq!(*first_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*second_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*third_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*fourth_was_dropped.lock().unwrap().deref(), false);
    assert_eq!(*fifth_was_dropped.lock().unwrap().deref(), false);

    svec.clear();

    //clearing should drop all elements
    assert_eq!(*first_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*second_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*third_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*fourth_was_dropped.lock().unwrap().deref(), true);
    assert_eq!(*fifth_was_dropped.lock().unwrap().deref(), true);
}