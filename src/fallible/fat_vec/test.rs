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
