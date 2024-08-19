use crate::panicking::InfallibleLinearMap;
use crate::panicking::InfallibleLinearSet;
use crate::VecMap;
use crate::VecSet;
use serde::{de::Visitor, ser::SerializeMap, ser::SerializeSeq, Deserialize, Serialize};
use std::marker::PhantomData;
#[cfg(test)]
mod test;
//custom implementation to ensure this gets (de)serialized as a map instead of a list of tuples
impl<K: Eq + Serialize, V: Sized + PartialEq + Serialize> Serialize for VecMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;

        for (k, v) in self.iter() {
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

//since the interior of a set is Map<T, ()>, we need custome serialization to ensure that
//sets get (de)serialized as lists.
impl<T: Eq + Serialize> Serialize for VecSet<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut list = serializer.serialize_seq(Some(self.len()))?;

        //TODO: Iterators
        for (t, _) in self.map().iter() {
            list.serialize_element(t)?;
        }

        list.end()
    }
}

struct VecSetVisitor<'de, T: Eq + Deserialize<'de>> {
    marker: PhantomData<fn() -> VecSet<T>>,
    blarghnungle: PhantomData<&'de str>,
}

impl<'de, T: Eq + Deserialize<'de>> Visitor<'de> for VecSetVisitor<'de, T> {
    type Value = VecSet<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("VecSet")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut set = VecSet::with_capacity(seq.size_hint().unwrap_or(0));

        while let Some(value) = seq.next_element()? {
            set.insert(value);
        }

        Ok(set)
    }
}

impl<'de, T: Eq + Deserialize<'de>> Deserialize<'de> for VecSet<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(VecSetVisitor {
            marker: PhantomData::default(),
            blarghnungle: PhantomData::default(),
        })
    }
}
