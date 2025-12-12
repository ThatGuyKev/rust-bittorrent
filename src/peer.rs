use bytes::{Bytes, BytesMut};
use serde::Serialize;
use serde::de::{Deserialize, Deserializer, Visitor};
use std::fmt;
use tokio::net::TcpStream;

#[derive(Debug, Serialize)]
pub struct Peers(pub Vec<Peer>);
#[derive(Debug, Serialize, serde::Deserialize)]
pub struct Peer {
    pub ip_address: String,
    pub port: String,
    #[serde(skip)]
    pub connection: Option<PeerConnection>,
    pub connected: bool,
}
#[derive(Debug)]
pub struct PeerConnection {
    pub stream: TcpStream,
    pub buffer: BytesMut,
}

impl PeerConnection {
    pub fn new(stream: TcpStream) -> PeerConnection {
        PeerConnection {
            stream,
            // TODO: find a way to calculate a suitable buffer size
            buffer: BytesMut::with_capacity(4096),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<PeerMessagesFrame>, anyhow::Error> {
        // TODO: impl loop frame reading
        Ok(Some(PeerMessagesFrame::Bitfield))
    }
    pub async fn write_frame(&mut self, frame: &PeerMessagesFrame) -> Result<(), anyhow::Error> {
        // TODO: impl sending a frame
        Ok(())
    }
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
                        .map(|chunk| Peer {
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

#[derive(Debug)]
pub struct Handshake {
    pub length: u8,
    pub bittorrent_protocol: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

pub enum PeerMessagesFrame {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
}
