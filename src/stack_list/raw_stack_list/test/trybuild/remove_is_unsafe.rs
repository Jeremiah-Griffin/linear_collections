use linear_collections::stack_list::raw_stack_list::remove;
use linear_collections::stack_list::RawStackList;

fn main(){
    let ref mut list  = RawStackList::<u8, 5>::uninit();

    let _ = remove!(list, 0, 5);
}
