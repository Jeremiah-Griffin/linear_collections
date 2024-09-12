use std::marker::PhantomData;

use serde::{
    ser::{SerializeMap, SerializeSeq},
    Deserialize, Serialize, Serializer,
};

use crate::fallible::{FallibleLinearMap, FallibleLinearSet};

pub(crate) fn serialize_fallible_map<
    'a,
    S: Serializer,
    K: Eq + Serialize,
    V: PartialEq + Serialize,
    M: FallibleLinearMap<K, V>,
>(
    fallible_map: &M,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut serializer = serializer.serialize_map(Some(fallible_map.len()))?;

    for (k, v) in fallible_map.iter() {
        serializer.serialize_entry(k, v)?;
    }

    serializer.end()
}

pub(crate) fn serialize_fallible_set<
    'a,
    S: Serializer,
    T: Eq + Serialize,
    M: FallibleLinearSet<T>,
>(
    fallible_set: &M,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut serializer = serializer.serialize_seq(Some(fallible_set.len()))?;

    for v in fallible_set.values() {
        serializer.serialize_element(v)?;
    }

    serializer.end()
}

struct MapVisitor<
    'de,
    K: Eq + Deserialize<'de>,
    V: Sized + PartialEq + Deserialize<'de>,
    M: FallibleLinearMap<K, V>,
> {
    marker: PhantomData<fn() -> M>,
    use_generics: PhantomData<(&'de str, K, V)>,
}

/*
impl<'de, K: Eq + Deserialize<'de>, V: Eq + Deserialize<'de>, M: FallibleLinearMap<K, V>>
    Visitor<'de> for MapVisitor<'de, K, V, M>
{
    type Value = M;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("map")
    }
}

pub(crate) fn deserialize_fallible_map<
    'a,
    D: Deserializer<'a>,
    K: Eq + Deserialize<'a>,
    V: PartialEq + Deserialize<'a>,
    M: FallibleLinearMap<K, V>,
>(
    fallible_map: &M,
    deserializer: D,
) -> Result<M, D::Error> {
    let mut deserializer = deserializer.deserialize_map(MapVisitor {
        marker: PhantomData::default(),
        use_generics: PhantomData::default(),
    });

    for (k, v) in fallible_map.iter() {
        serializer.serialize_entry(k, v)?;
    }

    serializer.end()
}*/
