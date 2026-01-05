use crate::peer::{connection::PeerConnection, extension::ExtensionMethods};
use crate::peer::message::PeerMessage;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Peer {
    pub peer_id: [u8; 20],
    pub ip_address: String,
    pub port: String,
    #[serde(skip)]
    pub connection: Option<PeerConnection>,
    pub connected: bool,
    pub extensions: Option<ExtensionMethods>,
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
            return conn.handshake(info_hash, peer_id, extension).await;
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
