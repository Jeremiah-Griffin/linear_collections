use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
};

use crate::panicking::{PanickingLinearMap, PanickingLinearSet};

pub(crate) fn serialize_panicking_map<
    'a,
    S: Serializer,
    K: Eq + Serialize,
    V: PartialEq + Serialize,
    M: PanickingLinearMap<K, V>,
>(
    panicking_map: &M,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut serializer = serializer.serialize_map(Some(panicking_map.len()))?;

    for (k, v) in panicking_map.iter() {
        serializer.serialize_entry(k, v)?;
    }

    serializer.end()
}

pub(crate) fn serialize_panicking_set<
    'a,
    S: Serializer,
    T: Eq + Serialize,
    M: PanickingLinearSet<T>,
>(
    panicking_set: &M,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut serializer = serializer.serialize_seq(Some(panicking_set.len()))?;

    for v in panicking_set.values() {
        serializer.serialize_element(v)?;
    }

    serializer.end()
}
