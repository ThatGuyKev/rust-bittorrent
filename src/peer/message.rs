use crate::utils::{get_len, get_payload, get_u8};
use bytes::{Buf, BufMut};
use serde::{Deserialize, Serialize};
use std::{fmt, io::Cursor};

use crate::peer::extension::{ExtensionHandshakePayload, ExtensionMessage};

#[derive(Debug)]
pub enum PeerMessage {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    // The 'have' message's payload is a single number, the index which that downloader just completed and checked the hash of.
    Have(u32),
    // 'bitfield' is only ever sent as the first message.
    // Its payload is a bitfield with each index sent set to one and the rest set to zero.
    // Downloaders which don't have anything yet may skip the 'bitfield' message.
    // The first byte of the bitfield corresponds to indices 0 - 7 from high bit to low bit, respectively.
    // The next one 8-15, etc. Spare bits at the end are set to zero.
    Bitfield(BitfieldPayload),
    // 'request' messages contain an index, begin, and length.
    // The last two are byte offsets. Length is generally a power of two unless it gets truncated by the end of the file.
    // All current implementations use 2^14 (16 kiB), and close connections which request an amount greater than that.
    Request(PeerRequest),
    // 'piece' messages contain an index, begin, and piece. Note that they are correlated with request messages implicitly.
    // It's possible for an unexpected piece to arrive if choke and unchoke messages are sent in quick succession and/or transfer is going very slowly.
    Piece(PiecePayload),
    // 'cancel' messages have the same payload as request messages.
    // They are generally only sent towards the end of a download, during what's called 'endgame mode'.
    // When a download is almost complete, there's a tendency for the last few pieces to all be downloaded off a single hosed modem line, taking a very long time.
    // To make sure the last few pieces come in quickly, once requests for all pieces a given downloader doesn't have yet are currently pending,
    // it sends requests for everything to everyone it's downloading from.
    // To keep this from becoming horribly inefficient, it sends cancels to everyone else every time a piece arrives.
    Cancel(CancelPayload),
    Handshake(Handshake),
    ExtensionHandshake(ExtensionHandshakePayload),
    ExtensionMessage(ExtensionMessage),
}

type BitfieldPayload = Vec<u8>;
#[derive(Debug, Serialize, Deserialize)]
pub struct CancelPayload {
    pub index: usize,
    pub begin: usize,
    pub length: usize,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PiecePayload {
    pub index: usize,
    pub begin: usize,
    pub piece: Vec<u8>,
}

#[derive(Debug)]
pub struct Handshake {
    pub length: u8,
    pub bittorrent_protocol: [u8; 19],
    pub reserved: [u8; 8],
    pub info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerRequest {
    index: usize,
    begin: usize,
    length: u32,
}
const RESERVED_LENGTH_BYTES: usize = 4;
const RESERVED_TAG_BYTES: usize = 1;

impl PeerRequest {
    pub fn new(index: usize, begin: usize, length: u32) -> Self {
        PeerRequest {
            index,
            begin,
            length,
        }
    }

    pub fn to_be_bytes(&self) -> Result<Vec<u8>, anyhow::Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&(self.index as u32).to_be_bytes());
        bytes.extend_from_slice(&(self.begin as u32).to_be_bytes());
        bytes.extend_from_slice(&self.length.to_be_bytes());
        return Ok(bytes);
    }
}

impl PeerMessage {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), anyhow::Error> {
        match get_u8(src)? {
            0..8 | 20 => {
                return Ok(());
            }
            message => return Err(anyhow::anyhow!("unknown message type '{}'", message)),
        };
    }
    pub fn serialize(&self) -> Result<Vec<u8>, anyhow::Error> {
        let bytes = match self {
            PeerMessage::Handshake(handshake) => {
                let mut buf = Vec::with_capacity(68);
                buf.put_u8(handshake.length);
                buf.put_slice(&handshake.bittorrent_protocol);
                buf.put_slice(&handshake.reserved);
                buf.put_slice(&handshake.info_hash);
                buf.put_slice(&handshake.peer_id);

                buf
            }

            PeerMessage::ExtensionHandshake(extension_handshake) => {
                // let payload_bytes = serde_bencode::to_bytes(&extension_handshake)?;
                let payload_bytes = extension_handshake.to_be_bytes()?;

                let mut buf = Vec::with_capacity(
                    RESERVED_LENGTH_BYTES + RESERVED_TAG_BYTES + 1 + payload_bytes.len(),
                );
                let len_slice =
                    u32::to_be_bytes(RESERVED_TAG_BYTES as u32 + 1 + payload_bytes.len() as u32);

                buf.put_slice(&len_slice);
                buf.put_u8(20 as u8); // Extension message ID
                buf.put_u8(0 as u8); // Extension Handshake ID
                buf.put_slice(&payload_bytes);

                buf
            }
            PeerMessage::Interested => {
                let mut buf =
                    Vec::with_capacity(4/* length */ + 1 /* message */ + 0 /*payload */);
                let len_slice = u32::to_be_bytes(1);
                buf.put_slice(&len_slice);
                buf.put_u8(2 as u8);

                buf
            }
            PeerMessage::Request(request_payload) => {
                let payload_bytes = request_payload.to_be_bytes()?;

                println!("Request payload length: {}", payload_bytes.len());
                let mut buf = Vec::with_capacity(
                    RESERVED_LENGTH_BYTES + RESERVED_TAG_BYTES + payload_bytes.len(),
                );
                let len_slice =
                    u32::to_be_bytes(RESERVED_TAG_BYTES as u32 + payload_bytes.len() as u32);

                buf.put_slice(&len_slice);
                buf.put_u8(6 as u8);
                buf.put_slice(&payload_bytes);

                buf
            }
            PeerMessage::ExtensionMessage(extension_message) => {
                let payload_bytes = extension_message.encode()?;

                let mut buf = Vec::with_capacity(
                    RESERVED_LENGTH_BYTES + RESERVED_TAG_BYTES + payload_bytes.len(),
                );
                let len_slice =
                    u32::to_be_bytes(RESERVED_TAG_BYTES as u32 + payload_bytes.len() as u32);

                buf.put_slice(&len_slice);
                buf.put_u8(20 as u8); // Extension message ID
                buf.put_slice(&payload_bytes);

                buf
            }
            message => {
                return Err(anyhow::anyhow!(
                    "Message type '{:?}' not implemented for sending",
                    message
                ));
            }
        };
        Ok(bytes)
    }
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<PeerMessage, anyhow::Error> {
        let len = get_len(src)?;

        println!(
            "Message length: {}, buffer position: {}",
            len,
            src.position()
        );
        let message = match get_u8(src)? {
            0 => PeerMessage::Choke,
            1 => PeerMessage::Unchoke,
            2 => PeerMessage::Interested,
            3 => PeerMessage::NotInterested,
            4 => PeerMessage::Have(src.get_u32()),
            5 => {
                // read the payload and convert to vec
                let payload = get_payload(src, (len - 1) as usize)?;
                PeerMessage::Bitfield(payload)
            }
            6 => {
                // read the payload and convert to request struct
                PeerMessage::Request(PeerRequest {
                    index: 0,
                    begin: 0,
                    length: 0,
                })
            }
            7 => {
                // read the payload and convert to piece struct

                let payload = get_payload(src, (len - 1) as usize)?;

                PeerMessage::Piece(PiecePayload::from_bytes(&payload)?)
            }
            8 => {
                // read the payload and convert to cancel struct
                PeerMessage::Cancel(CancelPayload {
                    index: 0,
                    begin: 0,
                    length: 0,
                })
            }
            20 => {
                let ext_msg = ExtensionMessage::parse(src, len)?;
                PeerMessage::ExtensionMessage(ext_msg)
            }

            message => {
                return Err(anyhow::anyhow!("unknown message type '{}'", message));
            }
        };

        Ok(message)
    }
}

impl PiecePayload {
    pub fn from_bytes(bytes: &[u8]) -> Result<PiecePayload, anyhow::Error> {
        let mut cursor = Cursor::new(bytes);

        if cursor.remaining() < 1 {
            return Err(anyhow::anyhow!("Incomplete PiecePayload header"));
        }

        let index = cursor.get_u32() as usize;
        let begin = cursor.get_u32() as usize;

        let mut piece = vec![0; cursor.remaining()];
        cursor.copy_to_slice(&mut piece);

        Ok(PiecePayload {
            index,
            begin,
            piece,
        })
    }
}

impl fmt::Display for PeerMessage {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PeerMessage::Choke => "Choke".fmt(fmt),
            PeerMessage::Unchoke => "Unchoke".fmt(fmt),
            PeerMessage::Interested => "Interested".fmt(fmt),
            PeerMessage::NotInterested => "NotInterested".fmt(fmt),
            PeerMessage::Bitfield(value) => format!("Bitfield => {:x?}", value).fmt(fmt),
            PeerMessage::Request(request_payload) => {
                format!("Request => {:?}", request_payload).fmt(fmt)
            }
            PeerMessage::Piece(piece_payload) => {
                format!("Piece => {:?}", piece_payload.index).fmt(fmt)
            }
            PeerMessage::Cancel(cancel_payload) => {
                format!("Cancel => {:?}", cancel_payload).fmt(fmt)
            }
            PeerMessage::Have(have_payload) => format!("Have => {}", have_payload).fmt(fmt),
            PeerMessage::Handshake(handshake) => format!("Handshake => {:?}", handshake).fmt(fmt),
            PeerMessage::ExtensionHandshake(extension_handshake) => {
                format!("Extension Handshake => {:?}", extension_handshake).fmt(fmt)
            }
            PeerMessage::ExtensionMessage(extension_message) => {
                format!("Extension Message => {:?}", extension_message).fmt(fmt)
            }
        }
    }
}
