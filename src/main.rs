use bittorrent::torrent::Torrent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let mut torrent = Torrent::from_file("sample.torrent").await?;
    // println!("New Torrent Parsed {:?}", torrent);

    // torrent.update_tracker().await?;

    // torrent.start_download().await?;

    let mut torrent = Torrent::from_magnet("magnet:?xt=urn:btih:3f994a835e090238873498636b98a3e78d1c34ca&dn=magnet2.gif&tr=http%3A%2F%2Fbittorrent-test-tracker.codecrafters.io%2Fannounce").await?;
    // println!("New Torrent from Magnet Parsed {:?}", torrent);
    // torrent.start_download().await?;

    println!("Torrent from Magnet Parsed {:?}", torrent);

    torrent.start_download().await?;

    anyhow::Ok(())
}
