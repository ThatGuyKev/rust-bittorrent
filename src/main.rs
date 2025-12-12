use std::{
    io::{Read, Write},
    net::TcpStream,
};

// Available if you need it!
use bittorrent::torrent::torrent::{Torrent};
use bittorrent::tracker::tracker::TrackerResponse;
use serde_bencode;
use bittorrent::utils::*;

// Usage: your_program.sh decode "<encoded_value>"
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let test_str = "5:hello";
    let (decoded_str, _) = decode_bencoded_value(test_str);
    println!("Decoded string: {}", decoded_str);

    let test_integer = "i42e";
    let (decoded_int, _) = decode_bencoded_value(test_integer);
    println!("Decoded int: {}", decoded_int);

    let test_list = "l5:helloi52ee";
    let (decoded_list, _) = decode_bencoded_value(test_list);
    println!("Decoded List: {}", decoded_list);

    let test_dic = "d3:foo3:bar5:helloi52ee";
    let (decoded_dic, _) = decode_bencoded_value(test_dic);
    println!("Decoded Dic: {}", decoded_dic);

    let mut torrent = Torrent::from_file("sample.torrent").await?;
    println!("New Torrent Parsed {:?}", torrent);

    torrent.update_tracker().await?;

    let mut stream =
        TcpStream::connect(&torrent.tracker.peers.0[0]).expect("Couldn't connect to peer");

    println!("Connected to peer: {}", &torrent.tracker.peers.0[0]);
    stream.write(&[19])?;
    stream.write(b"BitTorrent protocol")?;
    stream.write(b"00000000")?;
    stream.write(&torrent.info_hash)?;
    stream.write(&serde_bencode::to_bytes(&torrent.client_id).unwrap())?;

    let mut buffer = [0; 68];
    stream.read(&mut buffer)?;

    let decoded_handshake = Handshake {
        length: buffer[0],
        bittorrent_protocol: buffer[1..20].try_into().unwrap(),
        reserved: buffer[20..28].try_into().unwrap(),
        info_hash: buffer[28..48].try_into().unwrap(),
        peer_id: buffer[48..68].try_into().unwrap(),
    };
    println!("Decoded handshake: {:?}", decoded_handshake);

    println!("peer ID {:?}", hex::encode(decoded_handshake.peer_id));

    println!("Downloading file,,,");

    anyhow::Ok(())
}

enum PeerMessages {
    Choke,
    Unchoke,
    Interested,
    NotInterested,
    Have,
    Bitfield,
    Request,
    Piece,
    Cancel,
}
#[derive(Debug)]
struct Handshake {
    length: u8,
    bittorrent_protocol: [u8; 19],
    reserved: [u8; 8],
    info_hash: [u8; 20],
    peer_id: [u8; 20],
}

