//todo: handle unwraps
//todo: use anyhow
use torrent::types::*;
use bencode::types::BencodeDecoder;

use std::thread;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU32, Ordering};

//use tokio::sync::RwLock;

use std::env;
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

static PEER_HS_THREADS_MAX: u32 = 6;

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

async fn ask_peers(mut tracker: Tracker) {
    let peers = tracker.request().await.unwrap();
    let tracker = Arc::new(RwLock::new(tracker));
    let counter = Arc::new(AtomicU32::new(0));
    for peer in peers {
        if counter.load(Ordering::SeqCst) >= PEER_HS_THREADS_MAX {
            thread::sleep(Duration::from_millis(100));
            continue;
        }
        let (tracker, counter, peer) = (
            Arc::clone(&tracker),
            Arc::clone(&counter),
            peer.clone(), 
        );
        counter.fetch_add(1, Ordering::SeqCst);
        thread::spawn(move || {
            let tracker = tracker.read().unwrap();
            let stream = peer.handshake(
                &tracker.peer_id,
                &tracker.torrent.info_hash
            );
            if let Some(_) = stream {
                tracing::debug!("connected to peer at {x}", x = &peer.addr);
            }
            else {
                tracing::debug!("connection to peer at {x} failed", x = &peer.addr);
            }
            counter.fetch_sub(1, Ordering::SeqCst);
        });
        //tokio::spawn(async move {
        //    let tracker = tracker.read().await;
        //    let stream = peer.handshake(&tracker.peer_id,
        //        &tracker.torrent.info_hash).await;
        //    if let Some(_) = stream {
        //        tracing::debug!("connected to peer at {x}", x = &peer.addr);
        //    }
        //    else {
        //        tracing::debug!("connection to peer at {x} failed", x = &peer.addr);
        //    }
        //    counter.fetch_sub(1, Ordering::SeqCst);
        //});
    }
    while counter.load(Ordering::SeqCst) != 0 {
        thread::sleep(Duration::from_millis(100));
        //tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

fn get_torrent_path() -> String {
    let argv = env::args().collect::<Vec<String>>();
    if argv.len() != 2 {
        panic!("usage: ./rbt [path_to_torrent]");
    }
    return argv[1].clone();
}

//todo: change `path` to `argv[1]`
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let path = get_torrent_path();
    init_tracing_subscriber();
    let client = init_reqwest_client();
    let torrent = init_torrent_from_file(path);
    let tracker = Tracker::new(torrent, client);
    ask_peers(tracker).await;
}