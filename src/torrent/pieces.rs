use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::fmt;
#[derive(Debug)]
pub struct Pieces(pub Vec<[u8; 20]>);

impl<'de> Deserialize<'de> for Pieces {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PiecesVisitor;

        impl<'de> Visitor<'de> for PiecesVisitor {
            type Value = Pieces;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte string representing concatenated SHA1 hashes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() % 20 != 0 {
                    return Err(E::custom(
                        "length of pieces byte string is not a multiple of 20",
                    ));
                }

                Ok(Pieces(
                    v.chunks_exact(20)
                        .map(|chunk| chunk.try_into().unwrap())
                        .collect(),
                ))
            }
        }

        deserializer.deserialize_bytes(PiecesVisitor)
    }
}

impl Serialize for Pieces {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let single_slice = self.0.concat();
        serializer.serialize_bytes(&single_slice)
    }
}
