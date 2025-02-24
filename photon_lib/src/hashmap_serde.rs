use std::{fmt, marker::PhantomData};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{SeqAccess, Visitor},
    ser::SerializeSeq as _,
};

use super::*;

#[derive(Serialize, Deserialize)]
struct KeyValuePair<K, V> {
    pub key: K,
    pub value: V,
}

pub fn serialize<K, V, S>(map: &IndexMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Serialize,
    V: Serialize,
    S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(map.len()))?;

    for (key, value) in map {
        let pair = KeyValuePair { key, value };
        seq.serialize_element(&pair)?;
    }

    seq.end()
}

pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<IndexMap<K, V>, D::Error>
where
    K: Deserialize<'de> + Hash + Eq,
    V: Deserialize<'de>,
    D: Deserializer<'de>,
{
    struct IndexMapVisitor<K, V>(PhantomData<(K, V)>);

    impl<'de, K, V> Visitor<'de> for IndexMapVisitor<K, V>
    where
        K: Deserialize<'de> + Hash + Eq,
        V: Deserialize<'de>,
    {
        type Value = IndexMap<K, V>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of key-value pairs")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut map = IndexMap::new();

            while let Some(pair) = seq.next_element::<KeyValuePair<K, V>>()? {
                map.insert(pair.key, pair.value);
            }

            Ok(map)
        }
    }

    deserializer.deserialize_seq(IndexMapVisitor(PhantomData))
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;

    use crate::{PhotonHashmap, photon::object::PhotonObject};

    #[test]
    fn roundtrip() {
        let obj = PhotonHashmap(indexmap! {
            PhotonObject::Integer(0x12) => PhotonObject::Integer(0x34),
            PhotonObject::String("key".into()) => PhotonObject::String("value".into()),
            PhotonObject::ByteArray(b"binkey".into()) => PhotonObject::StringArray(vec!["ab".into(), "cd".into()]),
            PhotonObject::Hashtable(PhotonHashmap(indexmap! {
                PhotonObject::Float(1.0.into()) => PhotonObject::Boolean(true),
            })) => PhotonObject::Hashtable(PhotonHashmap(indexmap! {
                PhotonObject::Null => PhotonObject::Byte(0xAB),
            })),
        });

        let serialized = serde_json::to_string(&obj).expect("serialize");
        let deserialized = serde_json::from_str(&serialized).expect("deserialize");

        assert_eq!(obj, deserialized);
    }
}
