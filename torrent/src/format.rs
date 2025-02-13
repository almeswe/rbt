use sha1::*;
use crate::types::*;
use bencode::types::*;

impl TorrentFile {
    pub fn new(path: String, size: i64) -> TorrentFile {
        TorrentFile { path: path, size: size }
    }
}

impl<'a> Torrent<'a> {
    fn get_num_by_key(key: &str, root: &'a Pair) -> Option<i64> {
        root.get(key)?.to_num()
    }

    fn get_bin_by_key(key: &str, root: &'a Pair) -> Option<&'a Bytes<'a>> {
        Some(root.get(key)?.to_bin()?)
    }

    fn get_str_by_key(key: &str, root: &'a Pair) -> Option<String> {
        root.get(key)?.to_str()
    }

    fn get_list_by_key(key: &str, root: &'a Pair) -> Option<&'a List<'a>> {
        Some(root.get(key)?.to_list()?)
    }

    fn get_pair_by_key(key: &str, root: &'a Pair) -> Option<&'a Pair<'a>> {
        Some(root.get(key)?.to_pair()?)
    }
}

impl<'a> Torrent<'a> {
    #[inline]
    fn get_announce(root: &'a Pair) -> Option<String> {
        Self::get_str_by_key("announce", root)
    }

    fn get_announce_list(root: &'a Pair) -> Option<Vec<Vec<String>>> {
        let mut vec = vec![];
        let x = Self::get_list_by_key("announce-list", root)?;
        for i in 0..x.len() {
            let ext_vec = x[i].to_list()?;
            vec.push(vec![]);
            for item in ext_vec {
                vec[i].push(item.to_str()?);
            }
        }
        Some(vec)
    }

    fn get_piece_size(root: &'a Pair) -> Option<i64> {
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
                    path: Self::get_str_by_key("name", info)?,
                    size: Self::get_num_by_key("length", info)?
                });
            },
            Some(x) => {
                for item in x {
                    let item = item.to_pair()?;
                    vec.push(TorrentFile {
                        path: Self::get_list_by_key("path", item)?[0].to_str()?,
                        size: Self::get_num_by_key("length", item)?
                    })
                }
            }
        };
        Some(vec)
    }

    fn get_pieces(root: &'a Pair) -> Option<Vec<&'a [u8]>> {
        let x = Self::get_pair_by_key("info", root)?;
        let x = Self::get_bin_by_key("pieces", x)?;
        assert!(x.len() % 20 == 0);
        let mut vec = Vec::with_capacity(x.len() / 20);
        for range in (0..x.len()).step_by(20) {
            vec.push(&x[range..range+20]);
        }
        Some(vec)
    }

    fn get_info_hash(root: &'a Pair) -> Option<[u8; 20]> {
        let raw = root.get("info")?.from;
        Some(Sha1::digest(raw).into())
    }

    pub fn try_new(root: &'a Pair) -> Option<Torrent<'a>> {
        Some(Torrent {
            announce: Self::get_announce(root),
            announce_list: Self::get_announce_list(root),
            files: Self::get_files(root)?,
            piece_size: Self::get_piece_size(root)?,
            pieces: Self::get_pieces(root)?,
            info_hash: Self::get_info_hash(root)?
        })
    }

}