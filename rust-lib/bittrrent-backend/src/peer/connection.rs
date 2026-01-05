use bytes::{Buf, BufMut};
use core::fmt;
use std::io::Cursor;
use tokio::io::AsyncWriteExt;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::utils::{get_len, peek_u8};

use crate::peer::message::PeerMessage;
pub struct PeerConnection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

pub const STANDARD_BLOCK_SIZE: usize = 16 * 1024; // 16 KiB

impl PeerConnection {
    pub async fn new(peer_addr: String) -> Result<PeerConnection, anyhow::Error> {
        let stream = TcpStream::connect(peer_addr).await?;
        Ok(PeerConnection {
            stream,
            buffer: vec![0; 20 * STANDARD_BLOCK_SIZE],
            cursor: 0,
        })
    }

    pub async fn handshake(
        &mut self,
        info_hash: [u8; 20],
        peer_id: String,
        extension: [u8; 8],
    ) -> Result<(), anyhow::Error> {
        let mut buf = Vec::with_capacity(68);
        buf.put_u8(19);
        buf.put_slice(b"BitTorrent protocol");
        buf.put_slice(&extension);
        buf.put_slice(&info_hash);
        buf.put_slice(peer_id.as_bytes().try_into()?);

        self.stream.write_all(&buf).await?;

        let read_buf = &mut [0; 68];
        self.stream.read_exact(read_buf).await?;

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

        Ok(())
    }

    pub async fn read_frame(&mut self) -> Result<Option<PeerMessage>, anyhow::Error> {
        loop {
            // Attempt to parse frame here
            println!(
                "Buffer size: {}, Cursor: {}",
                self.buffer.len(),
                self.cursor
            );
            if let Ok(Some(frame)) = self.parse_frame() {
                return Ok(Some(frame));
            }
            println!("Cursor at {}", self.cursor);

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
                    println!("Cursor at {}, returning error", self.cursor);
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
                println!(
                    "Advancing buffer by {} bytes, new buffer length {}",
                    buf_pos,
                    self.buffer.len()
                );

                self.buffer.drain(..buf_pos);

                self.cursor = if buf_pos > self.cursor {
                    0
                } else {
                    self.cursor - buf_pos
                };

                return Ok(Some(frame));
            }
            Err(e) => {
                println!("Not enough data to parse a message: {}", e);
                return Err(e);
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &PeerMessage) -> Result<(), anyhow::Error> {
        let buf = frame.serialize()?;
        self.stream.write_all(&buf).await?;
        Ok(())
    }
}

impl fmt::Debug for PeerConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PeerConnection")
            .field("stream", &"TcpStream")
            .field("buffer_length", &self.buffer.len())
            .field("cursor", &self.cursor)
            .finish()
    }
}
