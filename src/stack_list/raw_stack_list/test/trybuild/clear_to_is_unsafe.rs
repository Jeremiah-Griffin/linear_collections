use linear_collections::stack_list::raw_stack_list::clear_to;
use linear_collections::stack_list::RawStackList;


fn main(){

    const CAPACITY: usize = 5;
    let ref mut list  = RawStackList::<u8, CAPACITY>::uninit();

    let _ = clear_to!(list, 5);
}
