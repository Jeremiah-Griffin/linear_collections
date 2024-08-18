use super::map::FatMap;

#[derive(Debug, PartialEq, Eq)]
///A set type backed by an FatVec, a vector with stack space to hold up to
///`STACK_CAPACITY` items on the stack. The remaining overflow onto the heap.
pub struct FatSet<K: Eq, const STACK_CAPACITY: usize> {
    fat_vec: FatMap<K, (), STACK_CAPACITY>,
}
