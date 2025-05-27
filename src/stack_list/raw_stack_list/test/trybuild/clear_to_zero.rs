use linear_collections::stack_list::raw_stack_list::clear_to;
use linear_collections::stack_list::RawStackList;

fn main(){
    let ref mut list  = RawStackList::<u8, 5>::uninit();

    
    let _ = unsafe { clear_to!(list, 0)};
}
