//todo: handle unwraps
//todo: use anyhow
use torrent::types::*;
use bencode::types::BencodeDecoder;

use std::path::Path;
use std::time::Duration;

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt
};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder
};

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn init_reqwest_client() -> Client {
    // set default User-Agent and Connection headers
    let mut map = HeaderMap::new();
    map.insert("Connection", HeaderValue::from_str("keep-alive").unwrap());
    map.insert("User-Agent", HeaderValue::from_str("python-requests/2.32.3").unwrap());
    // make client from builder
    ClientBuilder::new()
        .timeout(Duration::from_secs(5))
        .default_headers(map)
        .build().unwrap()
}

fn init_torrent_from_file<P: AsRef<Path>>(path: P) -> Torrent {
    let data = std::fs::read(path).unwrap();
    let root = BencodeDecoder::decode(&data).unwrap();
    let torrent = Torrent::new(&root).unwrap();
    tracing::debug!("torrent.announce:      {x:?}", x = torrent.announce);
    tracing::debug!("torrent.announce_list: {x:?}", x = torrent.announce_list);
    tracing::debug!("torrent.files:         {x:?}", x = torrent.files);
    tracing::debug!("torrent.piece_size:    {x:?}", x = torrent.piece_size);
    tracing::debug!("torrent.info_hash:     {x:?}", x = torrent.info_hash);
    torrent
}

//todo: change `path` to `argv[1]`
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let path = "/home/almeswe/Projects/rbt/data/t2.torrent"; 
    init_tracing_subscriber();
    let client = init_reqwest_client();
    let torrent = init_torrent_from_file(path);
    let mut tracker = Tracker::new(&torrent, &client);
    tracker.request().await.unwrap();
}