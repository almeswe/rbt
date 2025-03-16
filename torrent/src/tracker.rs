use rand::Rng;
use port_scanner;
use reqwest::{
    Result, Client, Url
};

use crate::types::*;
use crate::format::*;

use bencode::types::*;

use std::net::{
    Ipv4Addr, SocketAddrV4
};

fn url_string_as_bytes(x: &[u8]) -> String {
    // ignore UTF-8 checks..
    unsafe { String::from_utf8_unchecked(x.to_owned()) }
}

impl Tracker {
    fn gen_peer_id() -> [u8; 20] {
        let mut id = [0u8; 20];
        let mut rng = rand::rng();
        rng.fill(&mut id);
        id
    }

    fn get_how_much_left(&self) -> String {
        let mut total = 0usize;
        self.torrent.files.iter().for_each(|file| {
            assert!(file.size >= 0);
            total += file.size as usize;
        });
        assert!(total > self.downloaded);
        (total - self.downloaded).to_string()
    }

    #[inline(always)]
    fn get_how_much_uploaded(&self) -> String {
        "0".to_owned()
    }

    #[inline(always)]
    fn get_how_much_downloaded(&self) -> String {
        self.downloaded.to_string()
    }

    fn get_port(&self) -> String {
        // list of possible ports, sorted by priority.
        let ports = vec![6881, 6882, 6883, 6889];
        port_scanner::local_ports_available(ports).first()
            .expect("Cannot reserve port for use.").to_string()
    }

    fn get_tracker_params(&self) -> Vec<(&'static str, String)> {
        //todo: ?
        vec![
            ("info_hash", url_string_as_bytes(&self.torrent.info_hash)),
            ("peer_id", url_string_as_bytes(&self.peer_id)),
            ("uploaded", self.get_how_much_uploaded()),
            ("downloaded", self.get_how_much_downloaded()),
            ("port", self.get_port()),
            ("left", self.get_how_much_left())
        ]
    }
}

impl Tracker {
    pub fn new(torrent: Torrent, client: Client) -> Self {
        Self { 
            downloaded: 0,
            peer_id: Self::gen_peer_id(),
            client: client,
            torrent: torrent,
            interval: None,
            error: None
        }
    }

    fn get_peers(&self, pair: &Pair) -> Option<Vec<Peer>> {
        let mut peers = vec![];
        let from = get_bin_by_key("peers", pair)?;
        assert!(from.len() % 6 == 0);
        for i in (0..from.len()).step_by(6) {
            let ipv4 = Ipv4Addr::new(
                from[i + 0], from[i + 1],
                from[i + 2], from[i + 3]
            );
            let port = (from[i + 4] as u16) << 8 | 
                        from[i + 5] as u16;
            let addr = SocketAddrV4::new(ipv4, port as u16);
            peers.push(Peer::new(self.interval.unwrap(), addr));
        }
        tracing::info!("got {x} peer(s)", x = peers.len());
        Some(peers)
    }

    fn get_interval(&self, pair: &Pair) -> Option<u32> {
        let mut max = *get_num_by_key("interval", pair)?;
        if let Some(min) = get_num_by_key("min interval", pair) {
            max = *min;    
        }
        tracing::info!("timeout: {max}ms");
        Some(max as u32)
    }

    fn get_error(&self, pair: &Pair) -> Option<TrackerError> {
        let reason = get_owned_str_by_key("failure reason", pair);
        match reason {
            None => None, 
            Some(x) => { 
                tracing::error!("[TRACKER] {x}");
                Some(TrackerError { text: x })
            }
        }
    }

    pub async fn request(&mut self) -> Option<Vec<Peer>> {
        let url = Url::parse_with_params(&self.torrent.announce, self.get_tracker_params()).unwrap();
        tracing::debug!("tracker request url: {url:?}");
        let response = self.client.get(url).send().await.ok()?;
        tracing::debug!("tracker response status: {x}", x = response.status());
        let response = response.bytes().await.ok()?;
        tracing::debug!("tracker response text  : {x}", x = url_string_as_bytes(&response));
        tracing::debug!("tracker response length: {x}", x = response.len());
        let root = BencodeDecoder::decode(&response).unwrap();
        let pair = root.try_as_ref().unwrap() as &Pair;
        self.interval = self.get_interval(pair);
        self.error = self.get_error(pair);
        let peers = self.get_peers(pair).unwrap();
        Some(peers)
    }
}