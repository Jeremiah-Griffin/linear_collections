use std::intrinsics::transmute_unchecked;

use crate::stack_list::RawStackList;

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
//When `capacity` is less than `STACK_CAPACITY` fatvec's capacity should be equal to `STACK_CAPCITY`
pub fn with_capacity_less_than_stack() {
    let vec = FatVec::<bool, 10>::with_capacity(5).unwrap();
    assert_eq!(vec.capacity(), 10);
    assert_eq!(vec.len(), 0);
}

#[test]
//When `capacity` is equal to `STACK_CAPACITY` the fatvec's capacity should be equal to `STACK_CAPCITY`
pub fn with_capacity_equal_stack() {
    let vec = FatVec::<bool, 10>::with_capacity(10).unwrap();
    assert_eq!(vec.capacity(), 10);
    assert_eq!(vec.len(), 0);
}

#[test]
//When `capacity` is greater than `STACK_CAPACITY` the fatvec's capacity should be equal to the provided `capacity`
pub fn with_capacity_greater_than_stack() {
    let vec = FatVec::<bool, 10>::with_capacity(100).unwrap();
    assert_eq!(vec.capacity(), 100);
    assert_eq!(vec.len(), 0);
}

#[test]
///expanding the heap capacity of a  `FatVec` should not change its length
pub fn with_heap_capacity_does_not_change_length() {
    assert_eq!(FatVec::<bool, 0>::with_heap_capacity(0).unwrap().len(), 0);
    assert_eq!(FatVec::<bool, 50>::with_heap_capacity(50).unwrap().len(), 0);
}

//***methods***

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
pub fn get_unchecked_last_stack_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    assert_eq!(*unsafe { svec.get_unchecked(0) }, "one");

    svec.push("two").unwrap();
    assert_eq!(*unsafe { svec.get_unchecked(0) }, "one");
}
#[test]
pub fn get_unchecked_last_heap_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    svec.push("two").unwrap();
    assert_eq!(*unsafe { svec.get_unchecked(1) }, "two");
}

#[test]
pub fn get_mut_last_stack_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    assert_eq!(svec.get_mut(0), Some(&mut "one"));

    svec.push("two").unwrap();
    assert_eq!(svec.get_mut(0), Some(&mut "one"));
}
#[test]
pub fn get_mut_last_heap_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    assert_eq!(svec.get_mut(1), None);

    svec.push("two").unwrap();
    assert_eq!(svec.get_mut(1), Some(&mut "two"));
}

#[test]
pub fn get_unchecked_mut_last_stack_resident() {
    let mut svec = FatVec::<&str, 1>::new();

    svec.push("one").unwrap();
    assert_eq!(*unsafe { svec.get_unchecked_mut(0) }, "one");

    svec.push("two").unwrap();
    assert_eq!(*unsafe { svec.get_unchecked_mut(0) }, "one");
}
#[test]
pub fn get_unchecked_mut_last_heap_resident() {
    let mut svec = FatVec::<&str, 2>::new();

    svec.push("one").unwrap();
    svec.push("two").unwrap();
    assert_eq!(*unsafe { svec.get_unchecked_mut(1) }, "two");
}

#[test]
///Removing at any index from an empty `FatVec` should return `None`.
pub fn remove_empty() {
    let mut list = FatVec::<&str, 2>::new();
    assert_eq!(list.remove(0), None);
}

#[test]
///Removing at an index equal to the length of the `FatVec` should return `None`
///and the `FatVec` should be unchanged.
pub fn remove_at_length() {
    let zero = "zero";
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";

    let mut list = FatVec::with_array([zero, one]);

    list.push(two).unwrap();
    list.push(three).unwrap();
    list.push(four).unwrap();

    let mut cloned = FatVec::with_array([zero, one]);

    cloned.push(two).unwrap();
    cloned.push(three).unwrap();
    cloned.push(four).unwrap();
    assert_eq!(list.len(), 5);

    assert_eq!(list.remove(list.len()), None);
    assert_eq!(list, cloned);
}

#[test]
///Removing at an index greater than the length of the `FatVec` should return `None`
///and the `FatVec` should be unchanged.
pub fn remove_beyond_length() {
    let zero = "zero";
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";

    let mut list = FatVec::with_array([zero, one]);

    list.push(two).unwrap();
    list.push(three).unwrap();
    list.push(four).unwrap();

    let mut cloned = FatVec::with_array([zero, one]);

    cloned.push(two).unwrap();
    cloned.push(three).unwrap();
    cloned.push(four).unwrap();

    assert_eq!(list.len(), 5);
    assert_eq!(list.remove(list.len()), None);
    assert_eq!(list.remove(list.len() + 1), None);

    assert_eq!(list, cloned);
}

#[test]
pub fn remove_within_length() {
    let zero = "zero";
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";

    let mut list = FatVec::with_array([zero, one]);

    list.push(two).unwrap();
    list.push(three).unwrap();
    list.push(four).unwrap();

    assert_eq!(list.len(), 5);
    assert_eq!(list.remove(0), Some(zero));
    assert_eq!(list.len(), 4);
    assert_eq!(list.remove(0), Some(one));
    assert_eq!(list.len(), 3);
    assert_eq!(list.remove(0), Some(two));
    assert_eq!(list.len(), 2);
    assert_eq!(list.remove(0), Some(three));
    assert_eq!(list.len(), 1);
    assert_eq!(list.remove(0), Some(four));
    assert_eq!(list.len(), 0);
}

#[test]
///Remove should not only shift left within its allocation, but also shift elements from the heap to the left, *onto the stack*.
pub fn remove_shifts_onto_stack() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut list = FatVec::with_array([one, two]);

    list.push(three).unwrap();
    list.push(four).unwrap();
    list.push(five).unwrap();

    //remove the end of the stack

    assert_eq!(list.remove(1), Some("two"));

    //shift onto stack
    assert_eq!(
        unsafe { transmute_unchecked::<RawStackList<&str, 2>, [&str; 2]>(list.stack_list) },
        [one, three]
    );
}

#[test]
///Remove should not only shift left, but also shift elements from the heap to the left, *onto the stack*.
pub fn remove_shifts_from_heap() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut list = FatVec::with_array([one, two]);

    list.push(three).unwrap();
    list.push(four).unwrap();
    list.push(five).unwrap();

    //remove the end of the stack

    assert_eq!(list.remove(1), Some("two"));

    assert_eq!(list.vec, vec![four, five]);
}

#[test]
///remove_unchecked should not only shift left within its allocation, but also shift elements from the heap to the left, *onto the stack*.
pub fn remove_unchecked_shifts_onto_stack() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut list = FatVec::with_array([one, two]);

    list.push(three).unwrap();
    list.push(four).unwrap();
    list.push(five).unwrap();

    //remove_unchecked the end of the stack

    assert_eq!(unsafe { list.remove_unchecked(1) }, "two");

    //shift onto stack
    assert_eq!(
        unsafe { transmute_unchecked::<RawStackList<&str, 2>, [&str; 2]>(list.stack_list) },
        [one, three]
    );
}

#[test]
///remove_unchecked should not only shift left, but also shift elements from the heap to the left, *onto the stack*.
pub fn remove_unchecked_shifts_from_heap() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut list = FatVec::with_array([one, two]);

    list.push(three).unwrap();
    list.push(four).unwrap();
    list.push(five).unwrap();

    //remove_unchecked the end of the stack

    assert_eq!(unsafe { list.remove_unchecked(1) }, "two");

    assert_eq!(list.vec, vec![four, five]);
}

#[test]
pub fn into_iter_next() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut list = FatVec::with_array([one, two]);

    list.push(three).unwrap();
    list.push(four).unwrap();
    list.push(five).unwrap();

    assert_eq!(
        list.into_iter().collect::<Vec<&str>>(),
        vec![one, two, three, four, five]
    );
}

#[test]
pub fn into_iter_next_empty() {
    let list = FatVec::<&str, 2>::new();

    assert_eq!(list.into_iter().next(), None)
}

#[test]
pub fn into_iter_next_back() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut list = FatVec::with_array([one, two]);

    list.push(three).unwrap();
    list.push(four).unwrap();
    list.push(five).unwrap();
    let mut iter = list.into_iter();
    assert_eq!(iter.next_back(), Some(five));
    assert_eq!(iter.next_back(), Some(four));
    assert_eq!(iter.next_back(), Some(three));
    assert_eq!(iter.next_back(), Some(two));
    assert_eq!(iter.next_back(), Some(one));
    assert_eq!(iter.next_back(), None);
}

#[test]
pub fn into_iter_next_back_empty() {
    let list = FatVec::<&str, 2>::new();

    assert_eq!(list.into_iter().next_back(), None)
}

#[test]
pub fn pop() {
    let zero = "zero";
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";

    let mut list = FatVec::with_array([zero, one]);
    list.push(two).unwrap();
    list.push(three).unwrap();
    list.push(four).unwrap();

    assert_eq!(list.len(), 5);
    assert_eq!(list.pop(), Some(four));
    assert_eq!(list.len(), 4);
    assert_eq!(list.pop(), Some(three));
    assert_eq!(list.len(), 3);
    assert_eq!(list.pop(), Some(two));
    assert_eq!(list.len(), 2);
    assert_eq!(list.pop(), Some(one));
    assert_eq!(list.len(), 1);
    assert_eq!(list.pop(), Some(zero));
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop(), None);
    assert_eq!(list.len(), 0);
}

#[test]
pub fn with_array() {
    let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let fv = FatVec::<u8, 10>::with_array(nums);

    assert_eq!(fv.len(), 10);
    assert_eq!(Vec::from(nums), fv.into_iter().collect::<Vec<u8>>());
}

#[test]
pub fn with_partial_array() {
    let nums = [0, 1, 2];
    let fv = FatVec::<u8, 10>::with_partial_array(nums);

    assert_eq!(fv.len(), 3);
    assert_eq!(Vec::from(nums), fv.into_iter().collect::<Vec<u8>>());
}

#[test]
pub fn with_partial_array_full() {
    let nums = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let fv = FatVec::<u8, 10>::with_partial_array(nums);

    assert_eq!(fv.len(), 10);
    assert_eq!(Vec::from(nums), fv.into_iter().collect::<Vec<u8>>());
}
//miri does not support dev deps
#[cfg(not(miri))]
#[test]
pub fn with_partial_array_trybuild() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/fat_vec/trybuild/items_gt_stack_capacity.rs");
    t.compile_fail("src//fat_vec/trybuild/empty_items.rs");
}
