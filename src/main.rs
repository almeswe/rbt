//todo: handle unwraps
//todo: use anyhow
use torrent::types::*;
use bencode::types::BencodeDecoder;

use std::net::TcpStream;
use std::thread;
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};

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

static PEER_HS_THREADS_MAX: u32 = 12;

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
    tracing::info!("torrent.piece_size:    {x:?}", x = torrent.piece_size);
    tracing::debug!("torrent.info_hash:     {x:?}", x = torrent.info_hash);
    torrent
}

async fn ask_peers(mut tracker: Tracker) -> Arc<RwLock<Vec<(Peer, TcpStream)>>> {
    let alive = Arc::new(RwLock::new(Vec::new()));
    let peers = tracker.request().await.unwrap();
    let tracker = Arc::new(RwLock::new(tracker));
    let counter = Arc::new(AtomicU32::new(0));
    for peer in peers {
        while counter.load(Ordering::SeqCst) >= PEER_HS_THREADS_MAX {
            thread::sleep(Duration::from_millis(100));
        }
        let mut peer = peer.clone();
        let (alive, tracker, counter) = (
            Arc::clone(&alive),
            Arc::clone(&tracker),
            Arc::clone(&counter)
        );
        counter.fetch_add(1, Ordering::SeqCst);
        thread::spawn(move || {
            let tracker = tracker.read().unwrap();
            let conn = peer.handshake(&tracker);
            if let Err(e) = conn {
                tracing::error!("[FAILED] {addr}: {e}", addr = &peer.addr);
            }
            else {
                let mut peers = alive.write().unwrap();
                peers.push((peer, conn.unwrap()));
            }
            counter.fetch_sub(1, Ordering::SeqCst);
        });
    }
    while counter.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(100));
    }
    alive
}

async fn download(torrent: &Torrent, peers: Arc<RwLock<Vec<(Peer, TcpStream)>>>) -> std::io::Result<()> {
    let mut peers = peers.write().unwrap();
    // make it sequential for now..
    for (index, piece) in torrent.pieces.iter().enumerate() {
        let length = peers.len();
        let peer = &mut peers[index % length];
        tracing::debug!("[DOWNLOAD] {addr} piece no. {index}", addr = &peer.0.addr);
        let data = peer.0.request(index, torrent.piece_size as usize, &mut peer.1)?;
        tracing::debug!("piece length: {len}", len = data.len());
        thread::sleep(Duration::from_secs(2));
    }
    Ok(())
}

fn get_torrent_path() -> String {
    let argv = std::env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        eprintln!("usage: ./rbt [path_to_torrent]");
        std::process::exit(1);
    }
    return argv[1].clone();
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let path = get_torrent_path();
    init_tracing_subscriber();
    let client = init_reqwest_client();
    let torrent = init_torrent_from_file(path);
    let torrent2 = torrent.clone();
    let tracker = Tracker::new(torrent, client);
    let peers = ask_peers(tracker).await;
    download(&torrent2, peers).await.unwrap();
}