use std::{array, mem::MaybeUninit, ptr::{addr_of, addr_of_mut}};

#[cfg(test)]
mod test;
#[cfg(kani)]
mod verification;

#[derive(Debug)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
///A list resident on the stack which does not track the lenght of its contents, allowing it to
///be efficiently wrapped by other types which do.
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

    pub fn from_maybe_uninit(array: [MaybeUninit<T>; CAPACITY]) -> Self {
        RawStackList { array }
    }

    //**methods**//

    ///SAFETY: UB if `limit` is beyond CAPACITY OR > the lenght of the list.
    ///Drops all elements up to `limit`, exclusive.
    pub unsafe fn clear_to(&mut self, limit: usize) {
        self.array[0..limit]
            .iter_mut()
            //SAFETY: upheld by caller
            .for_each(|i| unsafe { i.assume_init_drop() });
       
    }


    ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized element.
    pub unsafe fn get(&self, index: usize) -> &T {
        //SAFETY: upheld by caller
        unsafe { self.array.get_unchecked(index).assume_init_ref() }
    }

    ///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized element.
    pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
        //SAFETY: upheld by caller
        unsafe { self.array.get_unchecked_mut(index).assume_init_mut() }
    }

    ///SAFETY: UB if index >= CAPACITY.
    pub unsafe fn insert_at(&mut self, index: usize, value: T) {
        //SAFETY: upheld by caller
        unsafe { self.array.get_unchecked_mut(index).write(value) };
    }    

    

    ///SAFETY:
    ///It must be guaranteed that all items <= index are initialized.
    pub unsafe fn iter_to<'a>(&'a self, index: usize) -> impl Iterator<Item = &'a T> {
        //TODO: This can panic
        self.array[0..index]
            .iter()
            //SAFETY:
            //Initializing is tied to the idx. all items <= to idx are guaranteed to be init.
            .map(|t| unsafe { t.assume_init_ref() })
    }

    pub unsafe fn iter_mut_to<'a>(&'a mut self, index: usize) -> impl Iterator<Item = &'a mut T> {
        //TODO: This can panic
        self.array[0..index]
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
 
