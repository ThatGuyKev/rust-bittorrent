use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use bittorrent::torrent::Torrent;
use anyhow::Result;

/// Manages the lifecycle and state of active torrents
pub struct TorrentService {
    torrents: Arc<RwLock<HashMap<String, Arc<RwLock<Torrent>>>>>,
}

impl TorrentService {
    pub fn new() -> Self {
        Self {
            torrents: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Start a torrent from a file path
    pub async fn start_torrent_from_file(&self, path: &str) -> Result<(String, String, i64)> {
        let torrent = Torrent::from_file(path).await?;
        
        let torrent_id = format!("torrent_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let name = torrent.torrent_file.info.name.clone();
        let total_size = torrent.torrent_file.info.length as i64;
        
        // Store the torrent
        self.torrents.write().insert(
            torrent_id.clone(),
            Arc::new(RwLock::new(torrent))
        );
        
        Ok((torrent_id, name, total_size))
    }

    /// Start a torrent from a magnet link
    pub async fn start_torrent_from_magnet(&self, magnet_uri: &str) -> Result<(String, String, i64)> {
        let torrent = Torrent::from_magnet(magnet_uri).await?;
        
        let torrent_id = format!("torrent_{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        let name = torrent.torrent_file.info.name.clone();
        let total_size = torrent.torrent_file.info.length as i64;
        
        // Store the torrent
        self.torrents.write().insert(
            torrent_id.clone(),
            Arc::new(RwLock::new(torrent))
        );
        
        Ok((torrent_id, name, total_size))
    }

    /// Get torrent status
    pub fn get_status(&self, torrent_id: &str) -> Result<TorrentStatus> {
        let torrents = self.torrents.read();
        let torrent_arc = torrents
            .get(torrent_id)
            .ok_or_else(|| anyhow::anyhow!("Torrent not found: {}", torrent_id))?;
        
        let torrent = torrent_arc.read();
        
        Ok(TorrentStatus {
            torrent_id: torrent_id.to_string(),
            status: "downloading".to_string(), // TODO: Add proper status tracking
            downloaded: torrent.tracker.downloaded as i64,
            total_size: torrent.torrent_file.info.length as i64,
            progress: torrent.tracker.downloaded as f64 / torrent.torrent_file.info.length as f64,
        })
    }

    /// Start downloading a torrent
    pub async fn start_download(&self, torrent_id: &str) -> Result<()> {
        let torrents = self.torrents.read();
        let torrent_arc = torrents
            .get(torrent_id)
            .ok_or_else(|| anyhow::anyhow!("Torrent not found: {}", torrent_id))?
            .clone();
        
        // Release the read lock before starting download
        drop(torrents);
        
        // Start download in the background
        let mut torrent = torrent_arc.write();
        torrent.start_download().await?;
        
        Ok(())
    }

    /// Get a reference to a torrent for monitoring
    pub fn get_torrent(&self, torrent_id: &str) -> Option<Arc<RwLock<Torrent>>> {
        self.torrents.read().get(torrent_id).cloned()
    }

    /// List all active torrents
    pub fn list_torrents(&self) -> Vec<String> {
        self.torrents.read().keys().cloned().collect()
    }
}

impl Default for TorrentService {
    fn default() -> Self {
        Self::new()
    }
}

pub struct TorrentStatus {
    pub torrent_id: String,
    pub status: String,
    pub downloaded: i64,
    pub total_size: i64,
    pub progress: f64,
}
