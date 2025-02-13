#[derive(Debug)]
pub struct TorrentFile {
    pub path: String,
    pub size: i64
}

#[derive(Debug)]
pub struct Torrent<'a> {
    pub announce: Option<String>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub files: Vec<TorrentFile>,
    pub pieces: Vec<&'a [u8]>,
    pub piece_size: i64,
    pub info_hash: [u8; 20]
}