use crate::pieces::Pieces;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::peer::{PeerMessage, PeerRequest};
use crate::tracker::{DEFAULT_TRACKER_PORT, Peers, Tracker, TrackerResponse};

use crate::utils::url_encode;

#[derive(Deserialize, Debug)]
pub struct Torrent {
    pub client_id: String,
    pub info_hash: [u8; 20],
    pub torrent_file: TorrentFile,
    pub tracker: Tracker,
}

#[derive(Deserialize, Debug)]
pub struct TorrentFile {
    pub announce: String,

    #[serde(rename = "created by")]
    pub created_by: String,
    pub info: TorrentInfo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TorrentInfo {
    pub length: usize,
    pub name: String,
    #[serde(rename = "piece length")]
    pub pieces_length: usize,
    pub pieces: Pieces,
}

impl Torrent {
    pub fn calculate_info_hash(&mut self) -> Result<[u8; 20], anyhow::Error> {
        let mut hasher = Sha1::new();
        let info_bytes = serde_bencode::to_bytes(&self.torrent_file.info)?;
        hasher.update(&info_bytes);
        let info_hash = hasher.finalize().try_into()?;
        self.info_hash = info_hash;
        Ok(info_hash)
    }

    pub fn new() -> Torrent {
        Torrent {
            client_id: String::new(),
            info_hash: [0; 20],
            torrent_file: TorrentFile {
                announce: String::new(),
                created_by: String::new(),
                info: TorrentInfo {
                    length: 0,
                    name: String::new(),
                    pieces_length: 0,
                    pieces: Pieces(Vec::new()),
                },
            },
            tracker: Tracker {
                port: DEFAULT_TRACKER_PORT.to_string(),
                uploaded: 0,
                downloaded: 0,
                left: 0,
                peers: Peers(Vec::new()),
            },
        }
    }

    pub async fn from_file(path: &str) -> Result<Torrent, anyhow::Error> {
        // TODO: for memory safety create a sized buffer
        let mut buffer = Vec::new();
        let read_size = File::open(path).await?.read_to_end(&mut buffer).await?;
        println!("Reading torrent file with byte size: {}", read_size);

        let mut new_torrent = Torrent {
            client_id: "-TR2940-6wfG2wk6wWLc".to_string(),
            info_hash: [0; 20],
            torrent_file: serde_bencode::from_bytes(&buffer)?,
            tracker: Tracker {
                port: DEFAULT_TRACKER_PORT.to_string(),
                uploaded: 0,
                downloaded: 0,
                left: 0,
                peers: Peers(Vec::new()),
            },
        };
        new_torrent.calculate_info_hash()?;
        Ok(new_torrent)
    }

    pub async fn update_tracker(&mut self) -> Result<(), anyhow::Error> {
        let left = (self.torrent_file.info.length - self.tracker.downloaded).to_string();

        let params = [
            ("peer_id", &self.client_id),
            ("port", &self.tracker.port),
            ("uploaded", &self.tracker.uploaded.to_string()),
            ("downloaded", &self.tracker.downloaded.to_string()),
            ("left", &left),
            ("compact", &"1".to_string()),
        ];

        let url = format!(
            "{}?info_hash={}",
            self.torrent_file.announce,
            url_encode(&self.info_hash)
        );
        let client = reqwest::Client::new();
        let body = client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .bytes()
            .await?;
        println!("Tracker Response Body: {:?}", body);

        let tracker_response: TrackerResponse = serde_bencode::from_bytes(&body)?;
        println!("Tracker Response {:?}", tracker_response);
        self.tracker.peers = tracker_response.peers;
        Ok(())
    }

    pub async fn start_download(&mut self) -> Result<(), anyhow::Error> {
        println!(
            "Starting download for torrent: {:?}",
            self.torrent_file.info.name
        );
        println!("Total size: {} bytes", self.torrent_file.info.length);
        let peer = &mut self.tracker.peers.0[1];
        println!("Connecting to {} peer", peer.ip_address);

        peer.establish_connection().await?;
        println!("Connected to peer at {}:{}", peer.ip_address, peer.port);

        peer.handshake(self.info_hash, self.client_id.clone())
            .await?;

        print!("Handshake successful. with peer {:x?}\n", peer.peer_id);

        if let Some(bitfield) = peer.next().await? {
            match bitfield {
                PeerMessage::Bitfield(payload) => {
                    println!("Received Bitfield message with payload: {:x?}", payload);
                }
                message => {
                    println!("Expected Bitfield message, but received '{}'.", message);
                }
            }
        }
        peer.send(&PeerMessage::Interested).await?;
        println!("Sent Interested message to peer.");
        if let Some(unchoke) = peer.next().await? {
            match unchoke {
                PeerMessage::Unchoke => {
                    println!("Received Unchoke message from peer.");
                }
                message => {
                    println!("Expected Unchoke message, but received '{}'.", message);
                }
            }
        }

        println!("Starting to request pieces...");

        let request_message = PeerRequest::new(0, 0, 16 * 1024);

        peer.send(&PeerMessage::Request(request_message)).await?;

        if let Some(piece) = peer.next().await? {
            match piece {
                PeerMessage::Piece(piece_payload) => {
                    println!("Received Piece message from peer. {}", piece_payload.index);
                    let file_path = format!("piece_{}.dat", piece_payload.index);
                    tokio::fs::write(&file_path, &piece_payload.piece).await?;
                    println!("Saved piece to {}", file_path);
                }
                message => {
                    println!("Expected Piece message, but received '{}'.", message);
                }
            }
        }

        Ok(())
    }
}

impl Default for Torrent {
    fn default() -> Self {
        Self::new()
    }
}
