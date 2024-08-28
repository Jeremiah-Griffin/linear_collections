use serde::{ser::SerializeSeq, Serialize};

use super::StackList;
impl<T: Serialize, const STACK_CAPACITY: usize> Serialize for StackList<T, STACK_CAPACITY> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut serializer = serializer.serialize_seq(Some(self.len()))?;

        for v in self.iter() {
            serializer.serialize_element(v)?;
        }

        serializer.end()
    }
}
