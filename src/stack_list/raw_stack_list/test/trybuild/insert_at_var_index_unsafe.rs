use linear_collections::stack_list::raw_stack_list::insert_at;
use linear_collections::stack_list::RawStackList;


fn main(){
    let ref mut list  = RawStackList::<u8, 5>::uninit();

    let idx = 0;
    let elem = 0;

    let _ =  insert_at!(list, idx, elem);
}
