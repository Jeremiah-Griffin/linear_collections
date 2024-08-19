use super::FatVec;

pub struct Iter<'a, const STACK_CAPACITY: usize, T> {
    idx: usize,
    fatvec: &'a FatVec<T, STACK_CAPACITY>,
}

impl<'a, const STACK_CAPACITY: usize, T> Iter<'a, STACK_CAPACITY, T> {
    pub fn new(fatvec: &'a FatVec<T, STACK_CAPACITY>) -> Self {
        Iter { idx: 0, fatvec }
    }
}

impl<'a, const STACK_CAPACITY: usize, T> Iterator for Iter<'a, STACK_CAPACITY, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.fatvec.get(self.idx);
        self.idx += 1;
        t
    }
}

/*
impl<'a, const STACK_CAPACITY: usize, T> Iterator for IterMut<'a, STACK_CAPACITY, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let t = self.svec.get_mut(self.idx);
        self.idx += 1;
        t
    }
}*/
