use std::{array, mem::MaybeUninit, num::{NonZero}, ptr::{addr_of, addr_of_mut}};

#[cfg(test)]
mod test;
#[doc(hidden)]
pub mod utils;
#[cfg(kani)]
mod verification;


///These consts should really be statics, but using statics in where bounds
///seems unsupported. Hopefully they get const promoted by the compiler.

const CAPACITY_NONZERO: & 'static str = "CAPACITY must be nonzero";
const INDEX_LT_CAPACITY: & 'static str = "INDEX must be less than CAPACITY.";
#[derive(Debug)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
///A list resident on the stack which does not track the lenght of its contents, allowing it to
///be efficiently wrapped by other types which do.
///This struct has no methods which perform indexing.. This is done 
pub struct RawStackList<T, const CAPACITY: usize> {
    array: [MaybeUninit<T>; CAPACITY],
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

    ///While this method on its own is safe, if the parent of this `RawStackList` is 
    pub fn from_maybe_uninit(array: [MaybeUninit<T>; CAPACITY]) -> Self {
        RawStackList { array }
    }

    //**methods**//
    ///SAFETY: Undefined Behavior if any element 0..LIMIT is uninitialized.
    ///Drops all elements up to `limit`, *exclusive*.
    ///This may be switched to inclusive in a future release. If so, this method will be deprecated
    ///to avoid silently changing behavior.
    pub unsafe fn clear_to<const LIMIT: usize>(& mut self) where [(); CAPACITY
        .checked_sub(LIMIT)
        .expect("LIMIT must not be greater than CAPACITY")]:,

        [(); LIMIT.checked_sub(1).expect("LIMIT must be nonzero")]:

    {
        //SAFETY
        //LIMIT being nonzero is checked at compile time.
        let limit = unsafe { NonZero::new_unchecked(LIMIT)};

        //SAFETY:
        //LIMIT is guaranteed to be within capacity. Limit elements 0..LIMIT being initialized
        //is upheld by caller.
        unsafe {utils::clear_to(self, limit)}
    }

    ///SAFETY: Undefined Behavior if accessing an uninitialized element.
    ///Retrieves a reference the element at `INDEX`, checking at compile time whether it is within `CAPACITY`.
    ///Note it does not check within the length, which, if tracked, is done at runtime by the parent of this type.
    ///
    ///To index by a variable use the `get` macro from this module.
    pub unsafe fn get<const INDEX: usize>(&self) -> & T where [(); CAPACITY
        //the final index of a list with CAPACITY is CAPACITY - 1.
        .checked_sub(1)
        .expect(CAPACITY_NONZERO)
        .checked_sub(INDEX)
        .expect(INDEX_LT_CAPACITY)]: {

        //SAFETY: bounding to length is upheld by caller.
        unsafe { utils::get(self, INDEX)}


    }

    ///SAFETY: Undefined Behavior if accessing an uninitialized element.
    ///Retrieves a reference the element at `INDEX`, checking at compile time whether it is within `CAPACITY`.
    ///Note it does not check within the length, which, if tracked, is done at runtime by the parent of this type.
    ///
    ///To index by a variable use the `get_mut` macro from this module.
    pub unsafe fn get_mut<const INDEX: usize>(& mut self) -> & mut T where [(); CAPACITY
        //the final index of a list with CAPACITY is CAPACITY - 1.
        .checked_sub(1)
        .expect(CAPACITY_NONZERO)
        .checked_sub(INDEX)
        .expect(INDEX_LT_CAPACITY)]: {

        //SAFETY: bounding to length is upheld by caller.
        unsafe { utils::get_mut(self, INDEX)}
    }

    ///Inserts an element at the supplied `INDEX`.
    ///Note that this method does not prevent the caller from inserting elements sparsely
    ///nor does it prevent inserting elements where they already exist; doing so will cause the prior item to
    ///cease to exist without calling `drop`.
    /// 
    ///Performs a bounds check at compile time to ensure `INDEX` < `CAPACITY`.
    ///If this bounds checking is undesirable or a variable value is necessary, use the `insert_at` macro from this module instead.
    pub fn insert_at<const INDEX: usize>(&mut self, value: T) where [(); CAPACITY
        //the final index of a list with CAPACITY is CAPACITY - 1.
        .checked_sub(1)
        .expect(CAPACITY_NONZERO)
        .checked_sub(INDEX)
        .expect(INDEX_LT_CAPACITY)]: {
        //SAFETY:
        //The bounds check in the where bound precludes inserting beyond CAPACITY.
        unsafe { utils::insert_at(self, INDEX, value)};
    }

  
    ///SAFETY:
    ///It must be guaranteed that all items <= index are initialized.
    ///Creates an iterator to `limit`, *inclusive*.
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



///`clear_to!(list: ident, limit: ident OR literal)`
///
///SAFETY: Undefined Behavior if any element from 0..`limit` is not initialized.
///if `limit` is an ident, it is Undefined Behavior to supply if `limit` is greater than CAPACITY.
///If providing an identifier, it must be of the type `NonZero<usize>`. However, literals supplied must be of type `usize`.
/// 
///Unfortunately, this macro is unable to bounds check constants, statics, etc; passing the
///identifier of a constant or static *is* safe, but will not provide compile time bound checking.
///If this is required use the `RawStackList::clear_to` instead.
pub macro clear_to {
    ($list_binding: ident, $limit_literal: literal) => {
        //We use the fully qualified path be used here to catch type errors with other types which also have a clear_to_bounded method
        linear_collections::stack_list::raw_stack_list::RawStackList::clear_to::<$limit_literal>($list_binding)
    },
    //Note that this will not match constant bindings; those are idents. 
    ($list_binding: ident, $limit_binding: ident) => {
        linear_collections::stack_list::raw_stack_list::utils::clear_to($list_binding, $limit_binding)
    },
}    

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
///If this is required use the `RawStackList::get` instead.
pub macro get {
    ($list_binding: ident, $index_literal: literal) => {
        //We use the fully qualified path be used here to catch type errors with other types which also have a get_bounded method
        linear_collections::stack_list::raw_stack_list::RawStackList::get::<$index_literal>($list_binding)
    },
    //Note that this will not match constant bindings; those are idents. 
    ($list_binding: ident, $index_binding: ident) => {
        linear_collections::stack_list::raw_stack_list::utils::get($list_binding, $index_binding)
    },
}

///`get_mut!(list: ident, index: ident OR literal)`
///
///SAFETY: Undefined Behavior if accessing an uninitialized element.
///if `index` is an ident, undefined behavor if accessing beyond the `CAPACITY` of the RawStackList.
/// 
///Retrieves a unique reference to element from `list` at `index`.
///if `index` is a literal this will run a bounds check to ensure it is less than the `CAPACITY` of `list`.
/// 
///Unfortunately, this macro is unable to bounds check constants, statics, etc; passing the
///identifier of a constant or static *is* safe, but will not provide compile time bound checking.
///If this is required use the `RawStackList::get_mut` instead.
pub macro get_mut {
    ($list_binding: ident, $index_literal: literal) => {
        //We use the fully qualified path be used here to catch type errors with other types which also have a get_mut_bounded method
        linear_collections::stack_list::raw_stack_list::RawStackList::get_mut::<$index_literal>($list_binding)
    },
    //Note that this will not match constant bindings; those are idents. 
    ($list_binding: ident, $index_binding: ident) => {
        linear_collections::stack_list::raw_stack_list::utils::get_mut($list_binding, $index_binding)
    },
}

///`insert_at!(list: ident, index: ident OR literal, item: ident)`
///
///SAFETY:
///If `item` is provided as a variable, Undefined Behavior will occur if it is greater than or equal to CAPACITY. This macro is unsafe to call.
///This macro is safe to call when `item` is a literal.
/// 
///Retrieves a unique reference to element from `list` at `index`.
///if `index` is a literal this will run a bounds check to ensure it is less than the `CAPACITY` of `list`.
/// 
///Unfortunately, this macro is unable to bounds check constants, statics, etc; passing the
///identifier of a constant or static *is* safe, but will not provide compile time bound checking.
///If this is required use the `RawStackList::insert_at` instead.
pub macro insert_at{
    ($list_binding: ident, $index_literal: literal, $item: ident) => {
        //We use the fully qualified path be used here to catch type errors with other types which also have a insert_at_bounded method
        linear_collections::stack_list::raw_stack_list::RawStackList::insert_at::<$index_literal>($list_binding, $item)
    },
    //Note that this will not match constant bindings; those are idents. 
    ($list_binding: ident, $index_binding: ident, $item: ident) => {
        linear_collections::stack_list::raw_stack_list::utils::insert_at($list_binding, $index_binding, $item)
    },
}

