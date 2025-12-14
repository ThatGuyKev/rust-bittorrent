use crate::peer::Peer;
use serde::{Deserialize, Serialize};

use serde::de::{Deserializer, Visitor};
use std::fmt;
pub const DEFAULT_TRACKER_PORT: usize = 6881;

// Not used now, but for reference
#[derive(Serialize, Deserialize)]
pub struct TrackerRequest {
    pub info_hash: [u8; 20],
    pub peer_id: String,
    pub port: String,
    pub uploaded: String,
    pub downloaded: String,
    pub left: String,
    pub compact: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrackerResponse {
    pub interval: usize,
    pub peers: Peers,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tracker {
    pub port: String,
    pub uploaded: usize,
    pub downloaded: usize,
    pub left: usize,
    pub peers: Peers,
}

#[derive(Debug, Serialize)]
pub struct Peers(pub Vec<Peer>);

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
                        .map(|chunk| Peer {
                            peer_id: [0; 20],
                            ip_address: format!(
                                "{}.{}.{}.{}",
                                chunk[0], chunk[1], chunk[2], chunk[3]
                            ),
                            port: format!("{}", u16::from_be_bytes([chunk[4], chunk[5]])),
                            connection: None,
                            connected: false,
                        })
                        .collect(),
                ))
            }
        }
        deserializer.deserialize_bytes(PeersVisitor)
    }
}
