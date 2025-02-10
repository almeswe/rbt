use bencode::types::*;

#[derive(Debug)]
pub struct TorrentFile {
    pub path: String,
    pub size: u64
}

#[derive(Debug)]
pub struct Torrent<'a> {
    pub announce: Option<String>,
    pub announce_list: Option<Vec<Vec<String>>>,
    pub files: Vec<TorrentFile>,
    pub pieces: Vec<&'a [u8]>,
    pub piece_size: u64
}

impl TorrentFile {
    pub fn new(path: String, size: u64) -> TorrentFile {
        TorrentFile { path: path, size: size }
    }
}

impl<'a> Torrent<'a> {
    fn get_num_by_key(key: &str, root: &Pair) -> Option<u64> {
        root.get(key)?.to_num()
    }

    fn get_list_by_key(key: &str, root: &'a Pair) -> Option<&'a List> {
        root.get(key)?.to_list()
    }

    fn get_bytes_by_key(key: &str, root: &'a Pair) -> Option<&'a Bytes> {
        root.get(key)?.to_bytes()
    }

    fn get_string_by_key(key: &str, root: &Pair) -> Option<String> {
        root.get(key)?.to_string()
    }

    fn get_pair_by_key(key: &str, root: &'a Pair) -> Option<&'a Pair> {
        root.get(key)?.to_pair()
    }

    fn get_announce(root: &Pair) -> Option<String> {
        Self::get_string_by_key("announce", root)
    }

    fn get_announce_list(root: &'a Pair) -> Option<Vec<Vec<String>>> {
        let mut vec = vec![];
        let x = Self::get_list_by_key("announce-list", root)?;
        for i in 0..x.len() {
            let ext_vec = x[i].to_list()?;
            vec.push(vec![]);
            for item in ext_vec {
                vec[i].push(item.to_string()?);
            }
        }
        Some(vec)
    }

    fn get_piece_size(root: &'a Pair) -> Option<u64> {
        let x = Self::get_pair_by_key("info", root)?;
        Some(Self::get_num_by_key("piece length", x)?)
    }

    fn get_files(root: &'a Pair) -> Option<Vec<TorrentFile>> {
        let mut vec = vec![];
        let info = Self::get_pair_by_key("info", root)?;
        let x = Self::get_list_by_key("files", info);
        match x {
            None => {
                vec.push(TorrentFile {
                    size: Self::get_num_by_key("length", info)?,
                    path: Self::get_string_by_key("name", info)?
                });
            },
            Some(x) => {
                for item in x {
                    let item = item.to_pair()?;
                    vec.push(TorrentFile {
                        size: Self::get_num_by_key("length", item)?,
                        path: Self::get_list_by_key("path", item)?[0].to_string()?
                    })
                }
            }
        };
        Some(vec)
    }

    fn get_pieces(root: &'a Pair) -> Option<Vec<&'a [u8]>> {
        let x = Self::get_pair_by_key("info", root)?;
        let x = Self::get_bytes_by_key("pieces", x)?;
        assert!(x.len() % 20 == 0);
        let mut vec = Vec::with_capacity(x.len() / 20);
        for range in (0..x.len()).step_by(20) {
            vec.push(&x[range..range+20]);
        }
        Some(vec)
    }

    pub fn try_new(root: &'a Pair) -> Option<Torrent<'a>> {
        Some(Torrent {
            announce: Self::get_announce(root),
            announce_list: Self::get_announce_list(root),
            files: Self::get_files(root)?,
            piece_size: Self::get_piece_size(root)?,
            pieces: Self::get_pieces(root)?
        })
    }
}