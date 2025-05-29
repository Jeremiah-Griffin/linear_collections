use super::RawStackList;

#[test]
fn remove_front_shifts_left() {
    let one = "one";
    let two = "two";
    let three = "three";
    let four = "four";
    let five = "five";

    let mut vec = vec![one, two, three, four, five];

    let mut arrvec = RawStackList::<&str,5>::uninit();

    unsafe {
        arrvec.insert_at::<0>(one);
        arrvec.insert_at::<1>(two);
        arrvec.insert_at::<2>(three);
        arrvec.insert_at::<3>(four);
        arrvec.insert_at::<4>(five);

        let (from_vec, from_arr) = (vec.remove(0), arrvec.remove::<0>(5));

        assert_eq!(from_vec, from_arr);

        let remaining = arrvec.iter_to::<4>().map(|t| *t).collect::<Vec<&str>>();

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
        arrvec.insert_at::<0>(one);
        arrvec.insert_at::<1>(two);
        arrvec.insert_at::<2>(three);
        arrvec.insert_at::<3>(four);
        arrvec.insert_at::<4>(five);

        let (from_vec, from_arr) = (vec.remove(2), arrvec.remove::<2>(5));

        assert_eq!(from_vec, from_arr);

        let remaining = arrvec.iter_to::<4>().map(|t| *t).collect::<Vec<&str>>();

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
        arrvec.insert_at::<0>(one);
        arrvec.insert_at::<1>(two);
        arrvec.insert_at::<2>(three);
        arrvec.insert_at::<3>(four);
        arrvec.insert_at::<4>(five);

        let (from_vec, from_arr) = (vec.remove(4), arrvec.remove::<4>(5));

        assert_eq!(from_vec, from_arr);

        let remaining = arrvec.iter_to::<4>().map(|t| *t).collect::<Vec<&str>>();

        assert_eq!(vec, remaining);
    }
}

#[test]
pub fn trybuild(){
    let t = trybuild::TestCases::new();
    //get
    //Expansion of the macro should require an unsafe block.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_is_unsafe.rs");
    //Indexing beyond capacity should failt to compile because of the where bound
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_out_of_bounds.rs");
    //Temporarily disabled due to ICE.
    //Indexing 0..Capacity  should pass
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/get_within_bounds.rs");
    
    //get_mut
    //Expansion of the macro should require an unsafe block.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_mut_out_of_bounds.rs");
    //Indexing beyond capacity should failt to compile because of the where bound
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_mut_is_unsafe.rs");
    //Temporarily disabled due to ICE.
    //Indexing 0..Capacity  should pass    
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/get_mut_within_bounds.rs");

    //clear_to
    //LIMIT must not be greater than CAPACITY
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/clear_to_out_of_bounds.rs");
    //Expansion of the macro should require an unsafe block.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/clear_to_is_unsafe.rs");
    //A zero literal should be rejected
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/clear_to_zero.rs");
    //Temporarily disabled due to ICE.
    //Indexing 0..Capacity  should pass    
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/clear_to_within_bounds.rs");


    //insert_at
    //When passing a variable for index, insert_at! is unsafe.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/insert_at_var_index_unsafe.rs");
    //Temporarily disabled due to ICE.
    //when passing a literal for index, insert_at! is safe
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/insert_at_literal_index_safe.rs");
    //insertions at a literal index >= CAPACITY should fail
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/insert_at_out_of_bounds.rs");

    //iter_to
    //Expansion of the macro should require an unsafe block.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/iter_to_out_of_bounds.rs");
    //Indexing beyond capacity should failt to compile because of the where bound
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/iter_to_is_unsafe.rs");
    //Temporarily disabled due to ICE.
    //Indexing 0..Capacity  should pass    
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/iter_to_within_bounds.rs");

    //iter_mut_to
    //Expansion of the macro should require an unsafe block.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/iter_mut_to_out_of_bounds.rs");
    //Indexing beyond capacity should failt to compile because of the where bound
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/iter_mut_to_is_unsafe.rs");
    //Temporarily disabled due to ICE.
    //Indexing 0..Capacity  should pass    
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/iter_mut_to_within_bounds.rs");


    //remove
    //Expansion of the macro should require an unsafe block.
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/remove_is_unsafe.rs");
    //Indexing beyond capacity should failt to compile because of the where bound
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/remove_out_of_bounds.rs");
    //Temporarily disabled due to ICE.
    //Indexing 0..Capacity  should pass
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/remove_within_bounds.rs");    
 }

