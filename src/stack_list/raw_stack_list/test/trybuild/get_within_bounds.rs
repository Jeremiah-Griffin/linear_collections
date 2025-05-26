#![feature(generic_arg_infer)]
use linear_collections::stack_list::raw_stack_list::get;
use linear_collections::stack_list::RawStackList;

fn main(){
    let ref list  = RawStackList::<u8, 5>::from_array([0,1,2,3,4]);

    let _ = unsafe { get!(list, 4)};
    let _ = unsafe { get!(list, 3)};
    let _ = unsafe { get!(list, 2)};
    let _ = unsafe { get!(list, 1)};
    let _ = unsafe { get!(list, 0)};
}
