use super::RawStackList;

#[cfg(test)]
mod test;
#[cfg(kani)]
mod verification;


///*This is an implementation detail. Please do not use this method.*
///Retrieves element at `index`.
///SAFETY: UB if accessed beyond CAPACITY *OR* into uninitialized element.
pub unsafe fn get<'a, T, const CAPACITY: usize>(list: & 'a RawStackList<T, CAPACITY>, index: usize) -> & 'a T{
    //SAFETY: upheld by caller
    unsafe { list.array.get_unchecked(index).assume_init_ref()}

}



