pub type Sha1Hash = [u8; 20];

#[derive(Debug)]
pub struct TorrentFile {
    pub path: String,
    pub size: i64
}

#[derive(Debug)]
pub struct Torrent {
    pub announce: String,
    pub announce_list: Vec<Vec<String>>,
    pub files: Vec<TorrentFile>,
    pub pieces: Vec<Sha1Hash>,
    pub piece_size: i64,
    pub info_hash: Sha1Hash
}

#[derive(Debug)]
pub struct Peer {
    pub addr: std::net::SocketAddrV4
}

#[derive(Debug)]
pub struct Tracker<'a> {
    pub downloaded: usize,
    pub peer_id: [u8; 20],
    pub client: &'a reqwest::Client,
    pub torrent: &'a Torrent
}

#[derive(Debug)]
pub struct TrackerError {
    pub text: String
}

#[derive(Debug)]
pub struct TrackerResponse {
    pub peers: Vec<Peer>,
    pub interval: u32,
    pub error: Option<TrackerError>
}