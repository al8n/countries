use super::StaticMap;
use ::serde::{ser::SerializeMap, Serialize};

impl<K: Serialize, V: Serialize> Serialize for StaticMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.map.len()))?;
        for (k, v) in self.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
