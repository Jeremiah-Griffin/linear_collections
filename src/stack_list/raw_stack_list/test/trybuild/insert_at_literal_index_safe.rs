use linear_collections::stack_list::raw_stack_list::insert_at;
use linear_collections::stack_list::RawStackList;


fn main(){
    let ref mut list  = RawStackList::<u8, 5>::uninit();

    let elem = 0;

    let _ =  unsafe { insert_at!(list, 4, elem)};
    let _ =  unsafe { insert_at!(list, 3, elem)};
    let _ =  unsafe { insert_at!(list, 2, elem)};
    let _ =  unsafe { insert_at!(list, 1, elem)};
    let _ =  unsafe { insert_at!(list, 0, elem)};
    
}
