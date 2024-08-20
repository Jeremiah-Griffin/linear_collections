use std::collections::VecDeque;

pub struct DequeMap<K: Eq, V: Sized + PartialEq> {
    deque: VecDeque<(K, V)>,
}

impl<K: Eq, V: Sized + PartialEq> DequeMap<K, V> {}
