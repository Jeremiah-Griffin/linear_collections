use std::{num::NonZero, mem::MaybeUninit};
use std::ptr::{addr_of, addr_of_mut};
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
pub unsafe fn get<T, const CAPACITY: usize>(list: & RawStackList<T, CAPACITY>, index: usize) -> &T{
    //SAFETY: upheld by caller
    unsafe { list.array.get_unchecked(index).assume_init_ref()}

}

///*This is an unstable implementation detail. Please do not use this method.*
///Retrieves a unique reference to the element at `index`.
///SAFETY: Undefined Behavior if accessed beyond CAPACITY *OR* into uninitialized element.
pub unsafe fn get_mut<T, const CAPACITY: usize>(list: & mut RawStackList<T, CAPACITY>, index: usize) -> & mut T{
    //SAFETY: upheld by caller
    unsafe { list.array.get_unchecked_mut(index).assume_init_mut()}
}


///SAFETY: UB if index >= CAPACITY.
pub unsafe fn insert_at<T, const CAPACITY: usize>(list: &mut RawStackList<T, CAPACITY>, index: usize, item: T) {
    //SAFETY: upheld by caller
    unsafe { list.array.get_unchecked_mut(index).write(item) };
}

///SAFETY:
///It must be guaranteed that all items <= index are initialized.
///Creates an iterator over all elements 0 to `limit`, *inclusive*.
pub unsafe fn iter_to<'a, T, const CAPACITY: usize>(list: & 'a RawStackList<T, CAPACITY>, limit: usize) -> impl Iterator<Item = &'a T> {
    list.array[0..limit]
        .iter()
        //SAFETY:
        //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
        .map(|t| unsafe { t.assume_init_ref() })
}



///SAFETY:
///It must be guaranteed that all items <= index are initialized.
///Creates an iterator over all elements 0 to `limit`, inclusive.
 pub unsafe fn iter_mut_to<'a, T, const CAPACITY: usize>(list: &'a mut RawStackList<T, CAPACITY>, limit: usize) -> impl Iterator<Item = &'a mut T> {
    //TODO: This can panic
    list.array[0..limit]
        .iter_mut()
        //SAFETY:
        //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
        .map(|t| unsafe { t.assume_init_mut() })
}


///SAFETY: UB if accessed beyond `CAPACITY` *OR* into an uninitialized element.
pub unsafe fn remove<T, const CAPACITY: usize>(list: &mut RawStackList<T,CAPACITY>, index: usize, length: usize) -> T {
    //SAFETY: upheld by caller
    //take value
    let t = unsafe { list.array.get_unchecked(index).assume_init_read() };
    //shift values right of `r` left.
    let elements_after_index = (length.saturating_sub(index)).saturating_sub(1);
    //SAFETY: upheld by caller
    unsafe { std::ptr::copy(
        (addr_of!(list.array) as *const MaybeUninit<T>).add(index + 1),
        (addr_of_mut!(list.array) as *mut MaybeUninit<T>).add(index),
        elements_after_index,
    ) };
    t
}
 
