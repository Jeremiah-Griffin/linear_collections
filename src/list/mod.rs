///Unidemnsional collections which may grow, optionally within a bound.
pub trait List<T>: Sized {
    ///Certain operations on `List`s are fallible. This is the error type used to
    ///reflect that.
    type Error: std::error::Error;

    //***constructors***//

    ///Creates a new `List`.
    fn new() -> Self;

    //***methods***//

    ///Returns the number of elements this `List` may contain either without reallocating or in toto.
    fn capacity(&self) -> usize;

    ///Calls `drop` on all elements of this `List`, leaving it empty with a length of 0.
    fn clear(&mut self) {
        for _ in 0..self.len() {
            let item = self.pop();
            drop(item);
        }
    }

    ///Returns an iterator that, when exhausted, leaves this `List` empty.
    fn drain<'a>(self) -> impl Iterator<Item = T>;

    ///Returns an iterator of references of the elements of this `List`.
    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    ///Returns an iterator of unique references of the elements of this `List`.
    fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        T: 'a;

    ///Returns the number of elements in this `List`.
    fn len(&self) -> usize;

    ///Removes the last element of this `List` if it is not empty.
    fn pop(&mut self) -> Option<T>;

    ///Returns the number of elements that may be inserted without reallocating, if possible.
    fn remaining_capacity(&self) -> usize {
        self.capacity().saturating_sub(self.len())
    }

    ///Appends `item` to the end of this `List`.
    fn push(item: T) -> Result<(), Self::Error>;
}
