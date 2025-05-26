#[test]
pub fn trybuild(){
    let t = trybuild::TestCases::new();
    //Indexing beyond capacity should failt to compile because of the where bound
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_out_of_bounds.rs");
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_mut_out_of_bounds.rs");


    //Expansion of the macro should require an unsafe block. Caller must ensure
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_is_unsafe.rs");
    t.compile_fail("src/stack_list/raw_stack_list/test/trybuild/get_mut_is_unsafe.rs");

    //Indexing anywhere within capacity  should pass
    // Temporarily disabled due to ICE.
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/get_within_bounds.rs");
    //t.pass("src/stack_list/raw_stack_list/test/trybuild/get_within_bounds.rs");
}
