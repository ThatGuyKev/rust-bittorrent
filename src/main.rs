use bittorrent::torrent::Torrent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut torrent = Torrent::from_file("sample.torrent").await?;
    // println!("New Torrent Parsed {:?}", torrent);

    // torrent.update_tracker().await?;

    // torrent.start_download().await?;

    let mut torrent = Torrent::from_magnet("magnet:?xt=urn:btih:ad42ce8109f54c99613ce38f9b4d87e70f24a165&dn=magnet1.gif&tr=http%3A%2F%2Fbittorrent-test-tracker.codecrafters.io%2Fannounce").await?;
    // println!("New Torrent from Magnet Parsed {:?}", torrent);
    // torrent.start_download().await?;

    anyhow::Ok(())
}
