use super::StackList;
#[kani::proof]
///Soundness
fn new(){
    StackList::<u8, 5>::new();
}

#[kani::proof]
fn clear() {
    

}
