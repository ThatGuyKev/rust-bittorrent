use bytes::{Buf, BufMut};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use std::vec;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const RESERVED_LENGTH_BYTES: usize = 4;
const RESERVED_TAG_BYTES: usize = 1;
pub const STANDARD_BLOCK_SIZE: usize = 16 * 1024; // 16 KiB

#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: [u8; 20],
    pub ip_address: String,
    pub port: String,
    #[serde(skip)]
    pub connection: Option<PeerConnection>,
    pub connected: bool,
}
#[derive(Debug)]
pub struct PeerConnection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Peer {
    pub async fn establish_connection(&mut self) -> Result<(), anyhow::Error> {
        self.connection =
            Some(PeerConnection::new(format!("{}:{}", self.ip_address, self.port)).await?);
        Ok(())
    }

    pub async fn send(&mut self, frame: &PeerMessage) -> Result<(), anyhow::Error> {
        if let Some(conn) = &mut self.connection {
            conn.write_frame(frame).await?;
        } else {
            return Err(anyhow::anyhow!("No connection established"));
        }
        Ok(())
    }

    pub async fn handshake(
        &mut self,
        info_hash: [u8; 20],
        peer_id: String,
        extension: [u8; 8],
    ) -> Result<(), anyhow::Error> {
        if let Some(conn) = &mut self.connection {
            let mut buf = Vec::with_capacity(68);
            buf.put_u8(19);
            buf.put_slice(b"BitTorrent protocol");
            buf.put_slice(&extension);
            buf.put_slice(&info_hash);
            buf.put_slice(peer_id.as_bytes().try_into()?);

            conn.stream.write_all(&buf).await?;

            let read_buf = &mut [0; 68];
            conn.stream.read_exact(read_buf).await?;

            println!("Len: {}", read_buf[0]);
            if read_buf[0] != 19 {
                return Err(anyhow::anyhow!(
                    "Invalid handshake response: incorrect protocol length"
                ));
            }

            println!("Protocol: {}", String::from_utf8(read_buf[1..20].to_vec())?);
            if read_buf[1..20] != b"BitTorrent protocol"[..] {
                return Err(anyhow::anyhow!(
                    "Invalid handshake response: incorrect protocol string"
                ));
            }
            println!("Reserved: {:x?}", &read_buf[20..28]);
            println!("Info Hash: {:x?}", &read_buf[28..48]);
            if read_buf[28..48] != info_hash[..] {
                return Err(anyhow::anyhow!(
                    "Invalid handshake response: info hash does not match"
                ));
            }
            println!("Peer ID: {:x?}", &read_buf[48..68]);
            self.peer_id = (&read_buf[48..68])
                .try_into()
                .map_err(|_| anyhow::anyhow!("Invalid peer ID length"))?;

            println!("Handshake with peer {} successful", self.ip_address);
            Ok(())
        } else {
            return Err(anyhow::anyhow!("No connection established"));
        }
    }

    pub async fn next(&mut self) -> Result<Option<PeerMessage>, anyhow::Error> {
        if let Some(conn) = &mut self.connection {
            return conn.read_frame().await;
        } else {
            return Err(anyhow::anyhow!("No connection established"));
        }
    }
}

impl PeerConnection {
    async fn new(peer_addr: String) -> Result<PeerConnection, anyhow::Error> {
        let stream = TcpStream::connect(peer_addr).await?;
        Ok(PeerConnection {
            stream,
            buffer: vec![0; 20 * STANDARD_BLOCK_SIZE],
            cursor: 0,
        })
    }

    pub async fn read_frame(&mut self) -> Result<Option<PeerMessage>, anyhow::Error> {
        loop {
            // Attempt to parse frame here
            if let Ok(Some(frame)) = self.parse_frame() {
                return Ok(Some(frame));
            }

            // Not enough buffered data to parse a frame.
            // Read more data

            // Ensure the buffer has capacity
            if self.buffer.len() == self.cursor {
                println!(
                    "Resizing buffer from {} to {}",
                    self.buffer.len(),
                    self.buffer.len() * 2
                );
                self.buffer.resize(self.cursor * 2, 0);
            }

            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;
            // 0 means "end of stream"
            if n == 0 {
                println!("End of stream reached");
                if self.cursor == 0 {
                    println!("Cursor is at 0, returning None");
                    return Ok(None);
                } else {
                    return Err(anyhow::anyhow!("connection reset by peer"));
                }
            } else {
                println!("Read {} bytes from stream", n);
                self.cursor += n;
            }
        }
    }

    pub fn parse_frame(&mut self) -> Result<Option<PeerMessage>, anyhow::Error> {
        let mut buf = Cursor::new(&self.buffer[..]);
        println!(
            "Parsing frame from buffer of length {}, cursor at {}",
            self.buffer.len(),
            self.cursor
        );

        let len = get_len(&mut buf)?;

        println!("Current buffer position: {}", buf.position());
        println!("Peeked message length: {}", len);

        if len == 0 {
            // keep-alive message
            println!("Keep-alive message received");
            return Err(anyhow::anyhow!("keep-alive message"));
        }

        if buf.has_remaining() {
            let next_byte = peek_u8(&mut buf)?;
            println!("Next byte (message ID): {}", next_byte);
        } else {
            println!("No more bytes remaining in buffer after length");
        }

        if len + 4 > self.cursor as u32 {
            println!(
                "Not enough data to parse message: expected length {}, but only have {} bytes",
                len, self.cursor
            );
            return Err(anyhow::anyhow!("not enough data"));
        }

        match PeerMessage::check(&mut buf) {
            Ok(_) => {
                buf.set_position(0);

                let frame = PeerMessage::parse(&mut buf)?;

                // Advance the buffer by removing the consumed bytes
                let buf_pos = buf.position() as usize;
                self.buffer.drain(..buf_pos);

                self.cursor = if buf_pos > self.cursor {
                    0
                } else {
                    self.cursor - buf_pos
                };
                println!(
                    "Advancing buffer by {} bytes, new buffer length {}",
                    buf_pos,
                    self.buffer.len()
                );

                return Ok(Some(frame));
            }
            Err(e) => {
                println!("Not enough data to parse a message: {}", e);
                return Err(e);
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &PeerMessage) -> Result<(), anyhow::Error> {
        match frame {
            PeerMessage::Handshake(handshake) => {
                let mut buf = Vec::with_capacity(68);
                buf.put_u8(handshake.length);
                buf.put_slice(&handshake.bittorrent_protocol);
                buf.put_slice(&handshake.reserved);
                buf.put_slice(&handshake.info_hash);
                buf.put_slice(&handshake.peer_id);

                self.stream.write_all(&buf).await?;
            }

            PeerMessage::ExtensionHandshake(extension_handshake) => {
                let payload_bytes = serde_bencode::to_bytes(&extension_handshake)?;

                let mut buf = Vec::with_capacity(
                    RESERVED_LENGTH_BYTES + RESERVED_TAG_BYTES + payload_bytes.len(),
                );
                let len_slice =
                    u32::to_be_bytes(RESERVED_TAG_BYTES as u32 + payload_bytes.len() as u32);

                buf.put_slice(&len_slice);
                buf.put_u8(20 as u8); // Extension message ID
                buf.put_slice(&payload_bytes);

                self.stream.write_all(&buf).await?;
            }
            PeerMessage::Interested => {
                let mut buf =
                    Vec::with_capacity(4/* length */ + 1 /* message */ + 0 /*payload */);
                let len_slice = u32::to_be_bytes(1);
                buf.put_slice(&len_slice);
                buf.put_u8(2 as u8);

                self.stream.write_all(&buf).await?;
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

                self.stream.write_all(&buf).await?;
            }
            PeerMessage::ExtensionMessage(extension_message) => {
                let (ext_msg_id, payload_bytes) = match extension_message {
                    ExtensionMessage::Handshake => {
                        // TODO: fix this handshake
                        let handshake_payload = ExtensionHandshakePayload {
                            m: ExtensionMethods {
                                ut_metadata: 1,
                                ut_pex: 2,
                            },
                        };

                        let payload = serde_bencode::to_bytes(&handshake_payload)?;

                        (0u8, payload)
                    }
                    ExtensionMessage::Metadata(metadata_message) => {
                        let payload = serde_bencode::to_bytes(&metadata_message)?;
                        (1u8, payload) // TODO: get extension ID dynamically
                    }
                };

                let mut buf = Vec::with_capacity(
                    RESERVED_LENGTH_BYTES + RESERVED_TAG_BYTES + 1 /* ext msg id */ + payload_bytes.len(),
                );
                let len_slice =
                    u32::to_be_bytes(RESERVED_TAG_BYTES as u32 + 1 + payload_bytes.len() as u32);

                buf.put_slice(&len_slice);
                buf.put_u8(20 as u8); // Extension message ID
                buf.put_u8(ext_msg_id);
                buf.put_slice(&payload_bytes);

                self.stream.write_all(&buf).await?;
            }
            message => {
                return Err(anyhow::anyhow!(
                    "Message type '{:?}' not implemented for sending",
                    message
                ));
            }
        }

        Ok(())
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

#[derive(Debug)]
pub enum ExtensionMessage {
    Handshake,
    Metadata(MetadataMessage),
}

#[derive(Debug, Serialize)]
pub enum MetadataMessage {
    Request {
        piece: u32,
    },
    Data {
        piece: u32,
        total_size: u32,
        data: Vec<u8>,
    },
    Reject {
        piece: u32,
    },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionHandshakePayload {
    pub m: ExtensionMethods,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtensionMethods {
    pub ut_metadata: u8,
    pub ut_pex: u8,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerRequest {
    index: usize,
    begin: usize,
    length: u32,
}

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

impl ExtensionMessage {
    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<ExtensionMessage, anyhow::Error> {
        let ext_msg_id = get_u8(src)?;
        match ext_msg_id {
            _ => Err(anyhow::anyhow!(
                "Unknown extension message ID '{}'",
                ext_msg_id
            )),
        }
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

                println!("Decoded piece payload size: {}", payload.len());
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
                // Extension message
                let ext_msg_id = get_u8(src)?;
                if ext_msg_id == 0 {
                    let payload = get_payload(src, (len - 2) as usize)?;
                    let ext_handshake: ExtensionHandshakePayload =
                        serde_bencode::from_bytes(&payload).map_err(|e| {
                            anyhow::anyhow!("Failed to decode extension handshake: {}", e)
                        })?;
                    PeerMessage::ExtensionHandshake(ext_handshake)
                } else {
                    return Err(anyhow::anyhow!(
                        "Unknown extension message ID '{}'",
                        ext_msg_id
                    ));
                }
            }

            message => {
                return Err(anyhow::anyhow!("unknown message type '{}'", message));
            }
        };

        Ok(message)
    }
}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, anyhow::Error> {
    if !src.has_remaining() {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    Ok(src.get_u8())
}

fn get_len(src: &mut Cursor<&[u8]>) -> Result<u32, anyhow::Error> {
    if src.remaining() < 4 {
        return Err(anyhow::anyhow!("Incomplete"));
    }
    Ok(src.get_u32())
}

fn peek_u8(src: &mut Cursor<&[u8]>) -> Result<u8, anyhow::Error> {
    if !src.has_remaining() {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    Ok(src.chunk()[0])
}

fn get_payload(src: &mut Cursor<&[u8]>, length: usize) -> Result<Vec<u8>, anyhow::Error> {
    if src.remaining() < length {
        return Err(anyhow::anyhow!("Incomplete"));
    }

    let mut payload = vec![0; length];
    src.copy_to_slice(&mut payload);

    Ok(payload)
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
