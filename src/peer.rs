use bytes::{Buf, BufMut};
use serde::Serialize;
use std::io::Cursor;
use std::{any, vec};
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

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

    pub async fn write(&mut self, frame: &PeerMessage) -> Result<(), anyhow::Error> {
        if let Some(conn) = &mut self.connection {
            conn.write_frame(frame).await?;
        } else {
            return Err(anyhow::anyhow!("No connection established"));
        }
        Ok(())
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
            // TODO: find a way to calculate a suitable buffer size
            buffer: vec![0; 4096],
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
                self.buffer.resize(self.cursor * 2, 0);
            }

            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;
            // 0 means"end of stream"
            if n == 0 {
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err(anyhow::anyhow!("connection reset by peer"));
                }
            } else {
                self.cursor += n;
            }
        }
    }

    pub fn parse_frame(&self) -> Result<Option<PeerMessage>, anyhow::Error> {
        let mut buf = Cursor::new(&self.buffer[..]);
        let len = get_len(&mut buf)?;
        if len == 0 {
            // keep-alive message
            return Err(anyhow::anyhow!("keep-alive message"));
        }

        match PeerMessage::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = PeerMessage::parse(&mut buf)?;

                return Ok(Some(frame));
            }
            Err(e) => return Err(e),
        }
    }

    pub async fn write_frame(&mut self, frame: &PeerMessage) -> Result<(), anyhow::Error> {
        
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
    Bitfield(Vec<u8>),
    // 'request' messages contain an index, begin, and length.
    // The last two are byte offsets. Length is generally a power of two unless it gets truncated by the end of the file.
    // All current implementations use 2^14 (16 kiB), and close connections which request an amount greater than that.
    Request {
        index: usize,
        begin: usize,
        length: usize,
    },
    // 'piece' messages contain an index, begin, and piece. Note that they are correlated with request messages implicitly.
    // It's possible for an unexpected piece to arrive if choke and unchoke messages are sent in quick succession and/or transfer is going very slowly.
    Piece {
        index: usize,
        begin: usize,
        piece: Vec<u8>,
    },
    // 'cancel' messages have the same payload as request messages.
    // They are generally only sent towards the end of a download, during what's called 'endgame mode'.
    // When a download is almost complete, there's a tendency for the last few pieces to all be downloaded off a single hosed modem line, taking a very long time.
    // To make sure the last few pieces come in quickly, once requests for all pieces a given downloader doesn't have yet are currently pending,
    // it sends requests for everything to everyone it's downloading from.
    // To keep this from becoming horribly inefficient, it sends cancels to everyone else every time a piece arrives.
    Cancel {
        index: usize,
        begin: usize,
        length: usize,
    },
    Handshake(Handshake),
}

impl PeerMessage {
    pub fn check(src: &mut Cursor<&[u8]>) -> Result<(), anyhow::Error> {
        match get_u8(src)? {
            b'0'..b'6' => {
                return Ok(());
            }
            message => return Err(anyhow::anyhow!("unknown message type '{}'", message)),
        };
    }

    pub fn parse(src: &mut Cursor<&[u8]>) -> Result<PeerMessage, anyhow::Error> {
        let message = match get_u8(src)? {
            b'0' => PeerMessage::Choke,
            b'1' => PeerMessage::Unchoke,
            b'2' => PeerMessage::Interested,
            b'3' => PeerMessage::NotInterested,
            b'4' => PeerMessage::Have(src.get_u32()),
            b'5' => {
                // read the payload and convert to vec
                PeerMessage::Bitfield(vec![])
            }
            b'6' => {
                // read the payload and convert to request struct
                PeerMessage::Request {
                    index: 0,
                    begin: 0,
                    length: 0,
                }
            }
            b'7' => {
                // read the payload and convert to piece struct
                PeerMessage::Piece {
                    index: 0,
                    begin: 0,
                    piece: vec![],
                }
            }
            b'8' => {
                // read the payload and convert to cancel struct
                PeerMessage::Cancel {
                    index: 0,
                    begin: 0,
                    length: 0,
                }
            }

            message => return Err(anyhow::anyhow!("unknown message type '{}'", message)),
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
