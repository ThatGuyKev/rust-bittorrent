use crate::peer::peer::Peers;
use serde::{Deserialize, Serialize};

pub const DEFAULT_TRACKER_PORT: usize = 6881;

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
