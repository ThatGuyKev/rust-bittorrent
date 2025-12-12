use bittorrent::torrent::Torrent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut torrent = Torrent::from_file("sample.torrent").await?;
    println!("New Torrent Parsed {:?}", torrent);

    torrent.update_tracker().await?;

    torrent.start_download().await?;

    anyhow::Ok(())
}
