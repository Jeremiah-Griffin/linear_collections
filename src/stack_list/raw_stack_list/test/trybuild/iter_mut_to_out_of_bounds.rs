use linear_collections::stack_list::raw_stack_list::iter_mut_to;
use linear_collections::stack_list::RawStackList;

fn main(){
    let ref mut list  = RawStackList::<u8, 5>::uninit();

    
    let _ = unsafe { iter_mut_to!(list, 5)};
}
