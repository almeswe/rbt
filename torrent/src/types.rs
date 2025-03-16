pub type Sha1Hash = [u8; 20];

#[derive(Debug, Clone)]
pub struct TorrentFile {
    pub path: String,
    pub size: i64
}

#[derive(Debug, Clone)]
pub struct Torrent {
    pub announce: String,
    pub announce_list: Vec<Vec<String>>,
    pub files: Vec<TorrentFile>,
    pub pieces: Vec<Sha1Hash>,
    pub piece_size: i64,
    pub info_hash: Sha1Hash
}

#[derive(Debug, Clone)]
pub enum PeerStatus {
    AmChoking,
    AmIntereseted,
    PeerChoking,
    PeerInterested
}

pub enum PeerMsg {
    KeepAlive(Vec<u8>),
    Have(Vec<u8>),
    Choke(Vec<u8>),
    Unchoke(Vec<u8>),
    Request(Vec<u8>),
    BitField(Vec<u8>),
    Handshake(Vec<u8>),
    Interested(Vec<u8>),
    NotInterested(Vec<u8>)
}

#[derive(Debug, Clone)]
pub struct Peer {
    pub choked: bool,
    pub timeout: u32,
    //pub have: Vec<u32>,
    pub bitfield: Option<Vec<u8>>,
    pub addr: std::net::SocketAddrV4,
}

#[derive(Debug)]
pub struct TrackerError {
    pub text: String
}

#[derive(Debug)]
pub struct TrackerResponse<'a> {
    pub peers: &'a Option<Vec<Peer>>,
    pub interval: &'a Option<u32>,
    pub error: &'a Option<TrackerError>
}

#[derive(Debug)]
pub struct Tracker {
    pub downloaded: usize,
    pub peer_id: [u8; 20],
    pub client: reqwest::Client,
    pub torrent: Torrent,
    // set after request is sent
    //pub peers: Vec<Peer>,
    pub interval: Option<u32>,
    pub error: Option<TrackerError>
}