use prost::Message;
use crate::proto::{EventDispatcher, event, notification};
use crate::{get_broadcaster, get_torrent_service};

/// Register all event handlers
pub fn register_handlers(dispatcher: &EventDispatcher) {
    // Register StartTorrent event
    dispatcher.register("StartTorrent", |payload| {
        let request = event::StartTorrentRequest::decode(&payload[..])
            .map_err(|e| format!("Failed to decode StartTorrentRequest: {}", e))?;

        // Use actual bittorrent backend to start torrent
        let torrent_service = get_torrent_service();
        
        // Block on async operation (we're in a sync context)
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;
        
        let result = runtime.block_on(async {
            // Check if it's a magnet link or file path
            if request.torrent_path.starts_with("magnet:") {
                torrent_service.start_torrent_from_magnet(&request.torrent_path).await
            } else {
                torrent_service.start_torrent_from_file(&request.torrent_path).await
            }
        }).map_err(|e| format!("Failed to start torrent: {}", e))?;

        let (torrent_id, name, total_size) = result;

        let response = event::StartTorrentResponse {
            torrent_id: torrent_id.clone(),
            name: name.clone(),
            total_size,
        };

        // Spawn a task to start actual download and send progress notifications
        spawn_download_task(torrent_id.clone(), name, total_size);

        Ok(response.encode_to_vec())
    });

    // Register GetTorrentStatus event
    dispatcher.register("GetTorrentStatus", |payload| {
        let request = event::GetTorrentStatusRequest::decode(&payload[..])
            .map_err(|e| format!("Failed to decode GetTorrentStatusRequest: {}", e))?;

        let torrent_service = get_torrent_service();
        
        let status = torrent_service.get_status(&request.torrent_id)
            .map_err(|e| format!("Failed to get torrent status: {}", e))?;

        let response = event::GetTorrentStatusResponse {
            torrent_id: status.torrent_id,
            status: status.status,
            downloaded: status.downloaded,
            total_size: status.total_size,
            progress: status.progress,
        };

        Ok(response.encode_to_vec())
    });
}

/// Start download task and send progress notifications
fn spawn_download_task(torrent_id: String, name: String, total_size: i64) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let broadcaster = get_broadcaster();
        let torrent_service = get_torrent_service();
        
        runtime.block_on(async {
            // Start the actual download
            if let Err(e) = torrent_service.start_download(&torrent_id).await {
                eprintln!("Error downloading torrent {}: {}", torrent_id, e);
                return;
            }

            // Monitor progress
            if let Some(torrent_arc) = torrent_service.get_torrent(&torrent_id) {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    
                    let torrent = torrent_arc.read();
                    let downloaded = torrent.tracker.downloaded as i64;
                    let progress = downloaded as f64 / total_size as f64;
                    
                    // Calculate download speed (simplified)
                    let download_speed = 1024 * 1024; // TODO: Calculate actual speed
                    
                    let notification = notification::TorrentProgressNotification {
                        torrent_id: torrent_id.clone(),
                        progress,
                        downloaded,
                        total_size,
                        download_speed,
                    };

                    broadcaster.broadcast("TorrentProgress", notification.encode_to_vec());

                    // Check if download is complete
                    if downloaded >= total_size {
                        let completion = notification::TorrentCompletedNotification {
                            torrent_id: torrent_id.clone(),
                            name: name.clone(),
                            total_size,
                        };
                        broadcaster.broadcast("TorrentCompleted", completion.encode_to_vec());
                        break;
                    }
                }
            }
        });
    });
}
