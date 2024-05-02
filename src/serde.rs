use crate::LinearMap;
use crate::VecMap;
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};
use std::marker::PhantomData;

impl<K: Eq + Serialize, V: Sized + PartialEq + Serialize> Serialize for VecMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.as_slice().len()))?;

        for (k, v) in self.as_slice().iter() {
            map.serialize_entry(k, v)?;
        }

        map.end()
    }
}

struct VecMapVisitor<'de, K: Eq + Deserialize<'de>, V: Sized + PartialEq + Deserialize<'de>> {
    marker: PhantomData<fn() -> VecMap<K, V>>,
    blarghnungle: PhantomData<&'de str>,
}

impl<'de, K: Eq + Deserialize<'de>, V: Sized + PartialEq + Deserialize<'de>> Visitor<'de>
    for VecMapVisitor<'de, K, V>
{
    type Value = VecMap<K, V>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("VecMap")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut vecmap = VecMap::with_capacity(map.size_hint().unwrap_or(0));

        while let Some((key, value)) = map.next_entry()? {
            vecmap.insert(key, value);
        }

        Ok(vecmap)
    }
}

impl<'de, K: Eq + Deserialize<'de>, V: Sized + PartialEq + Deserialize<'de>> Deserialize<'de>
    for VecMap<K, V>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(VecMapVisitor {
            marker: PhantomData::default(),
            blarghnungle: PhantomData::default(),
        })
    }
}
