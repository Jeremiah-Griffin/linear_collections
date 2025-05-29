use linear_collections::stack_list::raw_stack_list::iter_to;
use linear_collections::stack_list::RawStackList;


fn main(){
    let ref list  = RawStackList::<u8, 5>::uninit();

    let _ = iter_to!(list, 0);
}
