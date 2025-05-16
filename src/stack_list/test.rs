use super::{RawStackList, StackList};

#[test]
fn remove_start_is_same_as_vec() {
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
fn remove_mid_is_same_as_vec() {
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
fn remove_end_is_same_as_vec() {
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
pub fn get_0th_empty() {
    let no_stack = StackList::<&str, 0>::new();
    let some_stack = StackList::<&str, 5>::new();

    assert_eq!(no_stack.get(0), None);
    assert_eq!(some_stack.get(0), None);
}

#[test]
pub fn get_1th_empty() {
    let no_stack = StackList::<&str, 0>::new();
    let some_stack = StackList::<&str, 5>::new();

    assert_eq!(no_stack.get(1), None);
    assert_eq!(some_stack.get(1), None);
}

#[test]
pub fn eq_is_eq() {
    const ARRAY: [usize; 5] = [5; 5];
    assert_eq!(
        StackList::<usize, 5>::from_array(ARRAY),
        StackList::<usize, 5>::from_array(ARRAY)
    )
}

#[test]
fn remove_front_is_same_as_vec() {
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
pub fn clear_is_clear() {
    let mut svec = StackList::from_array(["one", "two", "three", "four", "five"].clone());

    svec.clear();

    assert_eq!(svec.iter().nth(0), None);
}
