use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

use super::FatVec;

#[test]
///After a call to new, an empty vec should have no items.
pub fn initial_len_is_zero() {
    assert_eq!(FatVec::<bool, 5>::new().len(), 0);
}

#[test]
///A call to capacity on an empty `FatVec` should return the STACK_CAPACITY only,
pub fn new_initial_capacity_is_stack_capacity() {
    assert_eq!(FatVec::<bool, 0>::new().capacity(), 0);
    assert_eq!(FatVec::<bool, 5>::new().capacity(), 5);
}

#[test]
///expanding the capacity of a  `FatVec` should not change its length
pub fn new_stack_capacity_does_not_mutate_length() {
    assert_eq!(FatVec::<bool, 0>::new().len(), 0);
    assert_eq!(FatVec::<bool, 50>::new().len(), 0);
}

#[test]
///expanding the stack capacity of a  `FatVec` should not change its length
pub fn new_stack_capacity_does_not_change_length() {
    assert_eq!(FatVec::<bool, 0>::new().len(), 0);
    assert_eq!(FatVec::<bool, 50>::new().len(), 0);
}

#[test]
///expanding the heap capacity of a  `FatVec` should not change its length
pub fn with_heap_capacity_does_not_change_length() {
    assert_eq!(FatVec::<bool, 0>::with_heap_capacity(0).unwrap().len(), 0);
    assert_eq!(FatVec::<bool, 50>::with_heap_capacity(50).unwrap().len(), 0);
}

//***methods***
#[test]
pub fn clear_is_clear() {
    let mut svec = FatVec::with_array(["one", "two", "three", "four", "five"].clone());

    svec.push("six").unwrap();
    svec.push("seven").unwrap();
    svec.push("eight").unwrap();
    svec.push("nine").unwrap();
    svec.push("ten").unwrap();
    svec.clear();

    assert_eq!(svec.len(), 0);
    assert_eq!(svec.vec.len(), 0);
    assert_eq!(svec.iter().nth(0), None);
}

#[test]
pub fn clear_drops() {
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

    {
        //**testing Drop tracking***

        let was_dropped = Arc::new(Mutex::new(false));

        assert_eq!(*was_dropped.lock().unwrap().deref(), false);

        let dropper = Dropper::new(was_dropped.clone());

        drop(dropper);

        assert_eq!(*was_dropped.lock().unwrap().deref(), true);
    }

    //If drop tracking works we can run the actual test.
    let first_was_dropped = Arc::new(Mutex::new(false));
    let second_was_dropped = Arc::new(Mutex::new(false));
    let third_was_dropped = Arc::new(Mutex::new(false));
    let fourth_was_dropped = Arc::new(Mutex::new(false));
    let fifth_was_dropped = Arc::new(Mutex::new(false));

    let mut svec = FatVec::with_array([
        Dropper::new(first_was_dropped.clone()),
        Dropper::new(second_was_dropped.clone()),
    ]);

    svec.push(Dropper::new(third_was_dropped.clone())).unwrap();
    svec.push(Dropper::new(fourth_was_dropped.clone())).unwrap();
    svec.push(Dropper::new(fifth_was_dropped.clone())).unwrap();

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

#[test]
pub fn get_0th_empty() {
    let no_stack = FatVec::<&str, 0>::new();
    let some_stack = FatVec::<&str, 5>::new();

    assert_eq!(no_stack.get(0), None);
    assert_eq!(some_stack.get(0), None);
}

#[test]
pub fn get_1th_empty() {
    let no_stack = FatVec::<&str, 0>::new();
    let some_stack = FatVec::<&str, 5>::new();

    assert_eq!(no_stack.get(1), None);
    assert_eq!(some_stack.get(1), None);
}

#[test]
pub fn get_last_stack_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    assert_eq!(svec.get(0), Some(&"one"));

    svec.push("two").unwrap();
    assert_eq!(svec.get(0), Some(&"one"));
}

#[test]
pub fn get_last_heap_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    assert_eq!(svec.get(1), None);

    svec.push("two").unwrap();
    assert_eq!(svec.get(1), Some(&"two"));
}

#[test]
///Vec's iter is known good. There shouldn't be any difference between these two
///under any circumstances.
pub fn iter_is_same_as_vec() {
    let array = ["one", "two", "three", "four", "five"];

    let mut svec = FatVec::with_array(array.clone());
    let mut vec = Vec::from(array);

    //With all items resident on the stack.
    assert_eq!(
        svec.iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>(),
        svec.iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>()
    );

    svec.push("six").unwrap();
    svec.push("seven").unwrap();
    svec.push("eight").unwrap();
    svec.push("nine").unwrap();
    svec.push("ten").unwrap();

    vec.push("six");
    vec.push("seven");
    vec.push("eight");
    vec.push("nine");
    vec.push("ten");

    //Half of items resident on heap should not chnange anything
    assert_eq!(
        svec.iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>(),
        svec.iter()
            .map(|s| String::from(*s))
            .collect::<Vec<String>>()
    );
}

#[test]
pub fn eq_is_eq() {
    const ARRAY: [usize; 5] = [5; 5];
    assert_eq!(
        FatVec::<usize, 5>::with_array(ARRAY),
        FatVec::<usize, 5>::with_array(ARRAY)
    )
}

#[test]
pub fn push_inside_stack_capacity_correct_invariants() {
    let mut svec = FatVec::<&'static str, 10>::new();

    svec.push("one").unwrap();
    svec.push("two").unwrap();
    svec.push("three").unwrap();
    svec.push("four").unwrap();
    svec.push("five").unwrap();

    let array = unsafe {
        svec.array[0..5]
            .into_iter()
            .map(|t| t.assume_init_read())
            .collect::<Vec<&str>>()
    };

    print!("{array:?}");

    assert_eq!(svec.vec.len(), 0);
    assert_eq!(svec.len(), 5);
    assert_eq!(array, vec!["one", "two", "three", "four", "five"])
}

#[test]
pub fn push_exact_stack_capacity_correct_invariants() {
    let mut svec = FatVec::<&'static str, 10>::new();

    svec.push("one").unwrap();
    svec.push("two").unwrap();
    svec.push("three").unwrap();
    svec.push("four").unwrap();
    svec.push("five").unwrap();
    svec.push("six").unwrap();
    svec.push("seven").unwrap();
    svec.push("eight").unwrap();
    svec.push("nine").unwrap();
    svec.push("ten").unwrap();

    let array = unsafe {
        svec.array
            .into_iter()
            .map(|t| t.clone().assume_init_read())
            .collect::<Vec<&str>>()
    };

    assert_eq!(svec.len(), 10);
    assert_eq!(svec.vec.len(), 0);
    assert_eq!(svec.len(), svec.array.len());
    assert_eq!(
        &array,
        &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"]
    );
}

#[test]
pub fn push_beyond_stack_capacity_correct_invariants() {
    let mut svec = FatVec::<&str, 5>::new();

    svec.push("one").unwrap();
    svec.push("two").unwrap();
    svec.push("three").unwrap();
    svec.push("four").unwrap();
    svec.push("five").unwrap();
    svec.push("six").unwrap();
    svec.push("seven").unwrap();
    svec.push("eight").unwrap();
    svec.push("nine").unwrap();
    svec.push("ten").unwrap();

    let array = unsafe {
        svec.array
            .into_iter()
            .map(|t| t.clone().assume_init_read())
            .collect::<Vec<&str>>()
    };

    assert_eq!(svec.len(), 10);
    assert_eq!(svec.vec.len(), 5);
    assert_eq!(svec.array.len(), 5);
    assert_eq!(svec.vec.len(), svec.array.len());
    assert_eq!(&array, &["one", "two", "three", "four", "five"]);
    assert_eq!(&svec.vec[0..], &["six", "seven", "eight", "nine", "ten"]);
}

#[test]
pub fn with_array_identical_to_push() {
    let mut svec: FatVec<&str, 5> = FatVec::new();

    svec.push("one").unwrap();
    svec.push("two").unwrap();
    svec.push("three").unwrap();
    svec.push("four").unwrap();
    svec.push("five").unwrap();

    assert_eq!(
        svec,
        FatVec::<&str, 5>::with_array(["one", "two", "three", "four", "five"])
    );
}

#[test]
pub fn remove_removes() {
    let mut v = FatVec::<&str, 5>::with_array(["one", "two", "three", "four", "five"]);

    v.push("six").unwrap();
    v.push("seven").unwrap();
    v.push("eight").unwrap();
    v.push("nine").unwrap();
    v.push("ten").unwrap();

    assert_eq!(v.remove(4), Some("five"));
    assert_eq!(v.remove(9), Some("ten"));
}

#[test]
///Remove should behave identically regardless of stack capacity.
pub fn remove_empty() {
    let mut empty_stack = FatVec::<&str, 0>::new();
    let mut has_stack = FatVec::<&str, 10>::with_heap_capacity(100).unwrap();

    assert_eq!(empty_stack.remove(0), None);
    assert_eq!(has_stack.remove(100), None);
}

#[test]
///When the requested index is at the end of the *array* there are no elements to shift left.
pub fn remove_shifts_at_end() {
    let mut v = FatVec::<&str, 5>::with_array(["one", "two", "three", "four", "five"]);

    let _ = v.remove(4);

    //if remove did not shift left we would expect uninitialized memory at `index` and the incorrect number of items.
    assert_eq!(
        v.iter().map(|t| t.clone()).collect::<Vec<&str>>(),
        vec!["one", "two", "three", "four"]
    )
}

#[test]
///When the requested index is > 0 but < array.len() all successive elements must shift left,
///but all preceeding elements must remain in place.
pub fn remove_shifts_at_middle() {
    let mut v = FatVec::<&str, 5>::with_array(["one", "two", "three", "four", "five"]);

    let _ = v.remove(1);

    //if remove did not shift left we would expect uninitialized memory at `index` and the incorrect number of items.
    assert_eq!(
        v.iter().map(|t| t.clone()).collect::<Vec<&str>>(),
        vec!["one", "three", "four", "five"]
    )
}

#[test]
///When the requested index is 0 of the *array* all elements should get shifted left.
pub fn remove_shifts_at_start() {
    let mut v = FatVec::<&str, 5>::with_array(["one", "two", "three", "four", "five"]);

    let _ = v.remove(0);

    //if remove did not shift left we would expect uninitialized memory at `index` and the incorrect number of items.
    assert_eq!(
        v.iter().map(|t| t.clone()).collect::<Vec<&str>>(),
        vec!["two", "three", "four", "five"]
    )
}
