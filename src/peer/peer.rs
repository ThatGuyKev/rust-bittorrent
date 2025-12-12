
use serde::Serialize;
use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt;

#[derive(Debug, Serialize)]
pub struct Peers(pub Vec<String>);
#[derive(Debug, Serialize, serde::Deserialize)]
pub struct Peer {
    pub ip_address: String,
    pub port: String,
}

impl<'de> Deserialize<'de> for Peers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PeersVisitor;

        impl<'de> Visitor<'de> for PeersVisitor {
            type Value = Peers;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("byte string representing concatenated peers")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() % 6 != 0 {
                    return Err(E::custom(
                        "length of peers byte string is not a multiple of 6",
                    ));
                }

                Ok(Peers(
                    v.chunks_exact(6)
                        .map(|chunk| {
                            format!(
                                "{}.{}.{}.{}:{}",
                                chunk[0],
                                chunk[1],
                                chunk[2],
                                chunk[3],
                                u16::from_be_bytes([chunk[4], chunk[5]])
                            )
                        })
                        .collect(),
                ))
            }
        }
        deserializer.deserialize_bytes(PeersVisitor)
    }
}
