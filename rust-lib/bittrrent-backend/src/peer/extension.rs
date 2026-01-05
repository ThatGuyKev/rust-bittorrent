use crate::torrent::TorrentInfo;
use crate::utils::{get_payload, get_u8};
use std::io::Cursor;

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize)]
pub enum ExtensionMessage {
    Handshake(ExtensionHandshakePayload),
    Metadata(MetadataMessage),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MetadataMessage {
    Request(MetadataRequest),
    Data(MetadataData),
    Reject { msg_type: usize, piece: usize },
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MetadataData {
    pub msg_type: usize,
    pub piece: usize,
    pub total_size: usize,

    pub torrent_info: Option<TorrentInfo>,
}
#[derive(Deserialize, Debug, Serialize)]
pub struct MetadataRequest {
    pub msg_type: usize,
    pub piece: usize,
    pub peer_extension_id: Option<usize>,
}

impl MetadataMessage {
    pub fn new_request(piece: usize, peer_ext_id: usize) -> Self {
        MetadataMessage::Request(MetadataRequest {
            msg_type: 0,
            piece,
            peer_extension_id: Some(peer_ext_id),
        })
    }

    pub fn new_data(piece: usize, total_size: usize, info: Option<TorrentInfo>) -> Self {
        MetadataMessage::Data(MetadataData {
            msg_type: 1,
            piece,
            total_size,
            torrent_info: info,
        })
    }

    pub fn new_reject(piece: usize) -> Self {
        MetadataMessage::Reject { msg_type: 2, piece }
    }
    pub fn to_be_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
        let bytes = match self {
            MetadataMessage::Request(MetadataRequest {
                msg_type,
                piece,
                peer_extension_id,
            }) => {
                let dict = serde_bencode::to_bytes(&serde_bencode::value::Value::Dict(
                    vec![
                        (
                            b"msg_type".to_vec(),
                            serde_bencode::value::Value::Int(*msg_type as i64),
                        ),
                        (
                            b"piece".to_vec(),
                            serde_bencode::value::Value::Int(*piece as i64),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ))?;
                println!("bencoded request dict: {:?}", dict.to_ascii_lowercase());
                if let Some(ext_id) = peer_extension_id {
                    let mut result = vec![*ext_id as u8];
                    result.extend_from_slice(&dict);
                    result
                } else {
                    let mut reuslt = vec![1u8]; // Default to 18 if no extension ID provided
                    reuslt.extend_from_slice(&dict);
                    reuslt
                }
            }
            MetadataMessage::Data(MetadataData {
                msg_type,
                piece,
                total_size,
                torrent_info: _,
            }) => {
                let dict = serde_bencode::to_bytes(&serde_bencode::value::Value::Dict(
                    vec![
                        (
                            b"msg_type".to_vec(),
                            serde_bencode::value::Value::Int(*msg_type as i64),
                        ),
                        (
                            b"piece".to_vec(),
                            serde_bencode::value::Value::Int(*piece as i64),
                        ),
                        (
                            b"total_size".to_vec(),
                            serde_bencode::value::Value::Int(*total_size as i64),
                        ),
                        // (
                        //     b"data".to_vec(),
                        //     serde_bencode::value::Value::Bytes(data.clone()),
                        // ),
                    ]
                    .into_iter()
                    .collect(),
                ))?;
                dict
            }
            MetadataMessage::Reject { msg_type, piece } => {
                let dict = serde_bencode::to_bytes(&serde_bencode::value::Value::Dict(
                    vec![
                        (
                            b"msg_type".to_vec(),
                            serde_bencode::value::Value::Int(*msg_type as i64),
                        ),
                        (
                            b"piece".to_vec(),
                            serde_bencode::value::Value::Int(*piece as i64),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ))?;
                dict
            }
        };
        Ok(bytes)
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionHandshakePayload {
    pub m: ExtensionMethods,
}

impl ExtensionHandshakePayload {
    pub fn to_be_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
        let dict = serde_bencode::to_bytes(&serde_bencode::value::Value::Dict(
            vec![(
                b"m".to_vec(),
                serde_bencode::value::Value::Dict(
                    vec![
                        (
                            b"ut_metadata".to_vec(),
                            serde_bencode::value::Value::Int(self.m.ut_metadata as i64),
                        ),
                        (
                            b"ut_pex".to_vec(),
                            serde_bencode::value::Value::Int(self.m.ut_pex as i64),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
            )]
            .into_iter()
            .collect(),
        ))?;
        Ok(dict)
    }
}

#[derive(Debug, Copy, Serialize, Deserialize, Clone)]
pub struct ExtensionMethods {
    pub ut_metadata: u8,
    pub ut_pex: u8,
}

impl ExtensionMessage {
    pub fn encode(&self) -> Result<Vec<u8>, anyhow::Error> {
        let encoded = match self {
            ExtensionMessage::Metadata(metadata_message) => metadata_message.to_be_bytes()?,
            ExtensionMessage::Handshake(_) => {
                return Err(anyhow::anyhow!(
                    "Extension Handshake should be sent via ExtensionHandshake variant"
                ));
            }
        };
        Ok(encoded)
    }
    pub fn parse(src: &mut Cursor<&[u8]>, len: u32) -> Result<ExtensionMessage, anyhow::Error> {
        println!("Parsing Extension Message");
        let message = match get_u8(src)? {
            0 => {
                // Extension Handshake
                let payload_bytes = get_payload(src, (len - 1) as usize)?;

                let extension_handshake: ExtensionHandshakePayload =
                    serde_bencode::from_bytes(&payload_bytes)?;
                println!("Parsed Extension Handshake: {:?}", extension_handshake);
                ExtensionMessage::Handshake(extension_handshake)
            }
            1 => {
                let payload_bytes = get_payload(src, (len - 2) as usize)?;
                println!("Metadata Payload Bytes: {:x?}", payload_bytes);
                let mut metadata_message: MetadataData = serde_bencode::from_bytes(&payload_bytes)?;
                let start_of_data = len as usize - 2 - metadata_message.total_size;
                let torrent_info: TorrentInfo =
                    serde_bencode::from_bytes(&payload_bytes[start_of_data..])?;
                metadata_message.torrent_info = Some(torrent_info);

                ExtensionMessage::Metadata(MetadataMessage::Data(metadata_message))
            }
            ext_msg_id => {
                println!("Unknown Extension Message ID: {}", ext_msg_id);
                return Err(anyhow::anyhow!(
                    "Unknown extension message ID '{}'",
                    ext_msg_id
                ));
            }
        };
        println!("Parsed Extension Message: {:?}", message);
        Ok(message)
    }
}

// use serde::de::Visitor;
// impl<'de> Deserialize<'de> for MetadataData {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         struct MetadataDataVisitor;
//         impl<'de> Visitor<'de> for MetadataDataVisitor {
//             type Value = MetadataData;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str("byte string representing metadata")
//             }

//             fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
//             where
//                 E: serde::de::Error,
//             {
//                 let info_bytes = &v[3..];
//                 let mut torrent_info: Option<TorrentInfo> = None;
//                 if let Ok(info) = serde_bencode::from_bytes(info_bytes) {
//                     torrent_info = Some(info);
//                 } else {
//                     println!("Cannot parse info")
//                 }
//                 Ok(MetadataData {
//                     msg_type: v[0] as usize,
//                     piece: v[1] as usize,
//                     total_size: v[2] as usize,
//                     torrent_info: torrent_info,
//                 })
//             }
//         }
//         deserializer.deserialize_bytes(MetadataDataVisitor)
//     }
// }
