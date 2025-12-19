use std::f32::consts::E;
use std::str::FromStr;

use crate::pieces::Pieces;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::peer::{
    ExtensionHandshakePayload, ExtensionMethods, PeerMessage, PeerRequest, STANDARD_BLOCK_SIZE,
};
use crate::tracker::{DEFAULT_TRACKER_PORT, Peers, Tracker, TrackerResponse};

use crate::utils::url_encode;

#[derive(Deserialize, Debug)]
pub struct Torrent {
    pub client_id: String,
    pub info_hash: [u8; 20],
    pub torrent_file: TorrentFile,
    pub tracker: Tracker,
    pub extensions: ExtensionMethods,
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

#[derive(Deserialize, Debug)]
pub struct MagnetLink {
    pub info_hash: [u8; 20],
    pub name: String,
    pub tracker_url: String,
}

impl FromStr for MagnetLink {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("magnet:?");
        let mut info_hash = [0u8; 20];
        let mut name = String::new();
        let mut tracker_url = String::new();

        let params: Vec<(&str, String)> = serde_urlencoded::from_str(s).expect("error again");

        for param in params.iter() {
            if param.0 == "xt" {
                let str_info_hash = param.1.trim_start_matches("urn:btih:").to_string();
                let info_hash_bytes = hex::decode(str_info_hash)?;
                info_hash.copy_from_slice(&info_hash_bytes);
            } else if param.0 == "dn" {
                name = param.1.to_string();
            } else if param.0 == "tr" {
                tracker_url = param.1.to_string();
            }
        }

        if info_hash.is_empty() || name.is_empty() || tracker_url.is_empty() {
            return Err(anyhow::anyhow!(
                "Missing required parameters in magnet link"
            ));
        }
        Ok(MagnetLink {
            info_hash,
            name,
            tracker_url,
        })
    }
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
            extensions: ExtensionMethods {
                ut_metadata: 1,
                ut_pex: 2,
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
            extensions: ExtensionMethods {
                ut_metadata: 1,
                ut_pex: 2,
            },
        };
        new_torrent.calculate_info_hash()?;
        new_torrent.update_tracker().await?;

        Ok(new_torrent)
    }

    // magnet:?xt=urn:btih:ad42ce8109f54c99613ce38f9b4d87e70f24a165&dn=magnet1.gif&tr=http%3A%2F%2Fbittorrent-test-tracker.codecrafters.io%2Fannounce
    //
    // xt: urn:btih: followed by the 40-char hex-encoded info hash (example: urn:btih:ad42ce8109f54c99613ce38f9b4d87e70f24a165)
    // dn: The name of the file to be downloaded (example: magnet1.gif)
    // tr: The tracker URL (example: http://bittorrent-test-tracker.codecrafters.io/announce)

    pub async fn from_magnet(magnet_uri: &str) -> Result<Torrent, anyhow::Error> {
        let magnet_link: MagnetLink = magnet_uri.parse()?;
        println!("Parsed Magnet Link: {:?}", magnet_link);

        let mut new_torrent = Torrent {
            client_id: "-TR2940-6wfG2wk6wWLc".to_string(),
            info_hash: magnet_link.info_hash,
            torrent_file: TorrentFile {
                announce: magnet_link.tracker_url,
                created_by: "magnet".to_string(),
                info: TorrentInfo {
                    length: 999,
                    name: magnet_link.name,
                    pieces_length: 0,
                    pieces: Pieces(Vec::new()),
                },
            },
            tracker: Tracker {
                port: DEFAULT_TRACKER_PORT.to_string(),
                uploaded: 0,
                downloaded: 0,
                left: 999,
                peers: Peers(Vec::new()),
            },
            extensions: ExtensionMethods {
                ut_metadata: 1,
                ut_pex: 2,
            },
        };

        new_torrent.update_tracker().await?;

        let peer = &mut new_torrent.tracker.peers.0[1];
        println!("Connecting to {} peer", peer.ip_address);

        peer.establish_connection().await?;

        println!("Connected to peer at {}:{}", peer.ip_address, peer.port);
        let mut extension = [0; 8];
        extension[5] = 16; // set the extension protocol bit
        println!("Extension bytes: {:x?}", extension);
        peer.handshake(
            new_torrent.info_hash,
            new_torrent.client_id.clone(),
            extension,
        )
        .await?;

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

        let handshake_payload = ExtensionHandshakePayload {
            m: new_torrent.extensions.clone(),
        };

        peer.send(&PeerMessage::ExtensionHandshake(handshake_payload))
            .await?;
        println!("Sent Extension Handshake message to peer.");

        if let Some(extension_handshake) = peer.next().await? {
            match extension_handshake {
                PeerMessage::ExtensionHandshake(payload) => {
                    println!(
                        "Received Extension Handshake message with payload: {:x?}",
                        payload
                    );
                }
                message => {
                    println!(
                        "Expected Extension Handshake message, but received '{}'.",
                        message
                    );
                }
            }
        }
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
        self.tracker.peers = tracker_response.peers.unwrap_or(Peers(Vec::new()));
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

        peer.handshake(self.info_hash, self.client_id.clone(), [0; 8])
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

        let mut file_data = Vec::new();

        for piece_index in 0..self.torrent_file.info.pieces.0.len() {
            println!("Requesting piece index: {}", piece_index);
            let piece_length = if self.torrent_file.info.pieces.0.len() - 1 == piece_index {
                self.torrent_file.info.length - (piece_index * self.torrent_file.info.pieces_length)
            } else {
                self.torrent_file.info.pieces_length
            };

            println!("Piece length: {}", piece_length);

            let n_blocks = (piece_length + STANDARD_BLOCK_SIZE - 1) / STANDARD_BLOCK_SIZE;
            for block in 0..n_blocks {
                let block_size = if block == n_blocks - 1 {
                    let remaining_block_size = piece_length % STANDARD_BLOCK_SIZE;
                    if remaining_block_size == 0 {
                        STANDARD_BLOCK_SIZE
                    } else {
                        remaining_block_size
                    }
                } else {
                    STANDARD_BLOCK_SIZE
                };

                let request_message =
                    PeerRequest::new(piece_index, block * STANDARD_BLOCK_SIZE, block_size as u32);
                println!("Requesting Block {:?}", request_message);

                peer.send(&PeerMessage::Request(request_message)).await?;

                if let Some(message) = peer.next().await? {
                    match message {
                        PeerMessage::Piece(piece_payload) => {
                            println!("Received Piece message from peer. {}", piece_payload.index);
                            // let file_path = format!(
                            //     "piece_{}_{}.dat",
                            //     piece_payload.index,
                            //     piece_payload.begin / STANDARD_BLOCK_SIZE
                            // );
                            // tokio::fs::write(&file_path, &piece_payload.piece).await?;
                            // println!("Saved piece to {}", file_path);

                            file_data.extend_from_slice(&piece_payload.piece);
                        }
                        message => {
                            println!("Expected Piece message, but received '{}'.", message);
                        }
                    }
                }
            }
        }

        println!("File data length: {}", file_data.len());
        println!("Torrent file length: {}", self.torrent_file.info.length);
        println!("File download completed. Saving to disk...");

        tokio::fs::write(
            format!("downloads/{}", &self.torrent_file.info.name),
            &file_data,
        )
        .await?;
        println!("Saved piece to {}", self.torrent_file.info.name);

        Ok(())
    }
}

impl Default for Torrent {
    fn default() -> Self {
        Self::new()
    }
}
