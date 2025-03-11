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

pub struct Tracker<'a> {
    torrent: &'a Torrent
}