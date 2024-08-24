use super::{ArrayVec, RawArrayVec};

#[test]
fn raw_arr_vec_remove_front_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawArrayVec::<&str, 5>::uninit();

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
fn raw_arr_vec_remove_mid_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawArrayVec::<&str, 5>::uninit();

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
fn raw_arr_vec_remove_end_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawArrayVec::<&str, 5>::uninit();

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

#[test]
fn arr_vec_remove_start_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = ArrayVec::<&str, 5>::new();

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
fn arr_vec_remove_mid_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = ArrayVec::<&str, 5>::new();

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
fn arr_vec_remove_end_is_same_as_vec() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = ArrayVec::<&str, 5>::new();

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
