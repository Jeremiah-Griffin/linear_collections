#![feature(generic_arg_infer)]
use linear_collections::stack_list::raw_stack_list::get_mut;
use linear_collections::stack_list::RawStackList;


fn main(){
    let ref mut list  = RawStackList::<u8, 5>::uninit();

    let _ = get_mut!(list, 0);
}
