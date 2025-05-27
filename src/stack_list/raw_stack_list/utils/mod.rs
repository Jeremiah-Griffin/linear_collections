use std::num::NonZero;

use super::RawStackList;

#[cfg(test)]
mod test;
#[cfg(kani)]
mod verification;


///*This is an unstable implementation detail. Please do not use this method.*
///SAFETY: Undefined Behavior if `limit` is greater than the length cum CAPACITY of the list.
///Drops all elements up to `limit`, *exclusive*.
pub unsafe fn clear_to<T, const CAPACITY: usize>(list: & mut RawStackList<T, CAPACITY>, limit: NonZero<usize>) {
    let mut i = 0;
    //While it is possible to supply a `limit` > CAPACITY, this is already UB so there is no  utility in checking it.
    #[cfg_attr(kani, kani::loop_invariant(i < CAPACITY))]
    while i < limit.get(){
        //SAFETY
        // Bounds checking of `limit` is upheld by caller.
       let item = unsafe {list.array.get_unchecked_mut(i)};
       //SAFETY
       //indexing into only valid items is upheld by caller.
       unsafe{item.assume_init_drop();}
       i = i.saturating_add(1);             
    }
}

///*This is an unstable implementation detail. Please do not use this method.*
///Retrieves a shared reference to the element at `index`.
///SAFETY: Undefined Behavior if accessed beyond CAPACITY *OR* into uninitialized element.
pub unsafe fn get<'a, T, const CAPACITY: usize>(list: & 'a RawStackList<T, CAPACITY>, index: usize) -> & 'a T{
    //SAFETY: upheld by caller
    unsafe { list.array.get_unchecked(index).assume_init_ref()}

}

///*This is an unstable implementation detail. Please do not use this method.*
///Retrieves a unique reference to the element at `index`.
///SAFETY: Undefined Behavior if accessed beyond CAPACITY *OR* into uninitialized element.
pub unsafe fn get_mut<'a, T, const CAPACITY: usize>(list: & 'a mut RawStackList<T, CAPACITY>, index: usize) -> & 'a mut T{
    //SAFETY: upheld by caller
    unsafe { list.array.get_unchecked_mut(index).assume_init_mut()}

}

