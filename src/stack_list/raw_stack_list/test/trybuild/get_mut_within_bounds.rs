#![feature(generic_arg_infer)]
use linear_collections::stack_list::raw_stack_list::get_mut;
use linear_collections::stack_list::RawStackList;

fn main(){
    let ref mut list  = RawStackList::<u8, 5>::from_array([0,1,2,3,4]);

    let _ = unsafe { get_mut!(list, 4)};
    let _ = unsafe { get_mut!(list, 3)};
    let _ = unsafe { get_mut!(list, 2)};
    let _ = unsafe { get_mut!(list, 1)};
    let _ = unsafe { get_mut!(list, 0)};
}
