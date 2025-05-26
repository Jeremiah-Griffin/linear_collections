use std::{array, mem::MaybeUninit, num::NonZero, ptr::{addr_of, addr_of_mut}};

#[cfg(test)]
mod test;
#[doc(hidden)]
pub mod utils;
#[cfg(kani)]
mod verification;

#[derive(Debug)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
///A list resident on the stack which does not track the lenght of its contents, allowing it to
///be efficiently wrapped by other types which do.
///This struct has no methods which perform indexing.. This is done 
pub struct RawStackList<T, const CAPACITY: usize> {
    ///temp compile hack
    pub array: [MaybeUninit<T>; CAPACITY],
}

impl<T, const CAPACITY: usize> RawStackList<T, CAPACITY> {
    //**constructors**//
    ///initializes all elements of this array to MaybeUninit::uninit.
    pub fn uninit() -> Self {
        Self {
            array: array::from_fn(|_| MaybeUninit::uninit()),
        }
    }

    ////Creates a RawStaticList from an array.
    pub fn from_array(array: [T; CAPACITY]) -> Self {
        RawStackList {
            //SAFETY:
            //The representation fo a MaybeUninity T and T are identical.
            //The lengths are the same in this case as well.
            array: unsafe {
                    core::intrinsics::transmute_unchecked(array)
                },
        }
    }

    pub fn from_maybe_uninit(array: [MaybeUninit<T>; CAPACITY]) -> Self {
        RawStackList { array }
    }

    //**methods**//

    ///SAFETY: UB if `limit` is greater than the length (cum CAPACITY) of the list.
    ///Drops all elements up to `limit`, exclusive.
    pub unsafe fn clear_to(&mut self, limit: NonZero<usize>) {
            let mut i = 0;

            //While it is possible to supply a `limit` > CAPACITY, this is already UB so there is no  utility in checking it.
            #[cfg_attr(kani, kani::loop_invariant(i < CAPACITY))]
             while i < limit.get(){
                 //SAFETY
                 // Bounds checking of `limit` is upheld by caller.
                let item = unsafe {self.array.get_unchecked_mut(i)};

                //SAFETY
                //indexing into only valid items is upheld by caller.
                unsafe{item.assume_init_drop();}

                i = i.saturating_add(1);             
             }
    }

    ///SAFETY: Undefined Behavior if accessing an uninitialized element.
    ///Retrieves a reference the element at `INDEX`, checking at compile time whether it is within `CAPACITY`.
    ///Note it does not check within the length, which, if tracked, is done at runtime by the parent of this type.
    ///
    ///To index by a variable use the `get` macro from this module.
    pub unsafe fn get<const INDEX: usize>(&self) -> & T where [(); CAPACITY
        //the final index of a list with CAPACITY is CAPACITY - 1.
        .checked_sub(1)
        .expect("CAPACITY must be nonzero")
        .checked_sub(INDEX)
        .expect("INDEX must be less than CAPACITY.")]: {

        //SAFETY: upheld by caller
        unsafe { self.array.get_unchecked(INDEX).assume_init_ref()}
    }

    ///SAFETY: UB if index >= CAPACITY.
    pub unsafe fn insert_at(&mut self, index: usize, value: T) {
        //SAFETY: upheld by caller
        unsafe { self.array.get_unchecked_mut(index).write(value) };
    }    
  
    ///SAFETY:
    ///It must be guaranteed that all items <= index are initialized.
    ///Creates an iterator to `limit`, inclusive.
    pub unsafe fn iter_to<'a>(&'a self, limit: usize) -> impl Iterator<Item = &'a T> {
        self.array[0..limit]
            .iter()
            //SAFETY:
            //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
            .map(|t| unsafe { t.assume_init_ref() })    }

    ///SAFETY:
    ///It must be guaranteed that all items <= index are initialized.
    ///Creates an iterator to `limit`, inclusive.
     pub unsafe fn iter_mut_to<'a>(&'a mut self, limit: usize) -> impl Iterator<Item = &'a mut T> {
        //TODO: This can panic
        self.array[0..limit]
            .iter_mut()
            //SAFETY:
            //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
            .map(|t| unsafe { t.assume_init_mut() })
    }

    ///SAFETY: UB if accessed beyond `CAPACITY` *OR* into an uninitialized element.
    pub unsafe fn remove(&mut self, index: usize, length: usize) -> T {
        //SAFETY: upheld by caller
        //take value
        let t = unsafe { self.array.get_unchecked(index).assume_init_read() };

        //shift values right of `r` left.
        let elements_after_index = (length.saturating_sub(index)).saturating_sub(1);

        //SAFETY: upheld by caller
        unsafe { std::ptr::copy(
            (addr_of!(self.array) as *const MaybeUninit<T>).add(index + 1),
            (addr_of_mut!(self.array) as *mut MaybeUninit<T>).add(index),
            elements_after_index,
        ) };

        t
    }
}


use crate as linear_collections;

///`get!(list: ident, index: ident OR literal)`
///
///SAFETY: Undefined Behavior if accessing an uninitialized element.
///if `index` is an ident, undefined behavor if accessing beyond the `CAPACITY` of the RawStackList.
/// 
///Retrieves a reference to element from `list` at `index`.
///if `index` is a literal this will run a bounds check to ensure it is less than the `CAPACITY` of `list`.
/// 
///Unfortunately, this macro is unable to bounds check constants, statics, etc; passing the
///identifier of a constant or static *is* safe, but will not provide compile time bound checking.
///If this is required use the `RawStackList::get_bounded` instead.
pub macro get {
    ($list_binding: ident, $index_literal: literal) => {
        //We use the fully qualified path be used here to catch type errors with other types which also have a get_bounded method
        // Wrapped in `{}` to support complex expressions.
        linear_collections::stack_list::raw_stack_list::RawStackList::get::<$index_literal>($list_binding)
    },
    //Note that this will not match constant bindings; those are idents. 
    ($list_binding: ident, $index_binding: ident) => {
        linear_collections::stack_list::raw_stack_list::utils::get($list_binding, $index_binding)
    },
}
