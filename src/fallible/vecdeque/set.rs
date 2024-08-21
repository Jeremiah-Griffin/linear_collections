use crate::fallible::FallibleLinearSet;

use super::map::DequeMap;

pub struct DequeSet<T: Eq> {
    map: DequeMap<T, ()>,
}

impl<T: Eq> FallibleLinearSet<T> for DequeSet<T> {
    type BACKING = DequeMap<T, ()>;

    fn map(&self) -> &Self::BACKING {
        &self.map
    }

    fn map_mut(&mut self) -> &mut Self::BACKING {
        &mut self.map
    }
}
