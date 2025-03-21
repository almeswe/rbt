use sha1::*;
use std::borrow::Cow;
use bencode::types::*;

use crate::types::*;

fn owned_str_from_bin(x: Vec<u8>) -> Option<String> {
    Some(String::from_utf8(x).ok()?)
}

fn owned_str_from_item(x: &BencodeItem) -> Option<String> {
    match x {
        BencodeItem::Bin(x) => Some(owned_str_from_bin(x.to_owned())?),
        BencodeItem::Str(x) => Some(x.to_owned()),
        _ => None
    }
}

pub fn get_by_key<T: AsRef<str>>(key: T, root: &Pair) -> Option<&BencodeItem> {
    Some(&root.iter().find(|x| &x.0 == key.as_ref())?.1)
}

// todo: use macro instead (maybe?).
pub fn get_num_by_key<T: AsRef<str>>(key: T, root: &Pair) -> Option<&i64> {
    let item = get_by_key(key, root)?;
    match item {
        BencodeItem::Num(x) => Some(x),
        _ => None
    }
}

pub fn get_bin_by_key<T: AsRef<str>>(key: T, root: &Pair) -> Option<&Bin> {
    let item = get_by_key(key, root)?;
    match item {
        BencodeItem::Bin(x) => Some(x),
        _ => None
    }
}

pub fn get_str_by_key<'a, T: AsRef<str>>(key: T, root: &'a Pair) -> Option<Cow<'a, str>> {
    let item = get_by_key(key, root)?;
    match item {
        BencodeItem::Str(x) => Some(Cow::Borrowed(x)),
        BencodeItem::Bin(x) => Some(Cow::Owned(owned_str_from_bin(x.to_owned())?)),
        _ => None
    }
}

pub fn get_list_by_key<T: AsRef<str>>(key: T, root: &Pair) -> Option<&List> {
    let item = get_by_key(key, root)?;
    match item {
        BencodeItem::List(x) => Some(x),
        _ => None
    }
}

pub fn get_pair_by_key<T: AsRef<str>>(key: T, root: &Pair) -> Option<&Pair> {
    let item = get_by_key(key, root)?;
    match item {
        BencodeItem::Pair(x) => Some(x),
        _ => None
    }
}

pub fn get_owned_str_by_key<T: AsRef<str>>(key: T, root: &Pair) -> Option<String> {
    let cow = get_str_by_key(key, root)?;
    match cow {
        Cow::Owned(x) => Some(x),
        Cow::Borrowed(x) => Some(x.to_owned())
    }
}

impl TorrentFile {
    pub fn new(path: String, size: i64) -> TorrentFile {
        TorrentFile { path: path, size: size }
    }
}

impl Torrent {
    fn get_announce(root: &Pair) -> Option<String> {
        get_owned_str_by_key("announce", root)
    }

    fn get_announce_list(root: &Pair) -> Option<Vec<Vec<String>>> {
        let mut vec: Vec<Vec<String>> = vec![];
        let branch = get_list_by_key("announce-list", root)?;
        for (idx, extv) in branch.iter().enumerate() {
            vec.push(vec![]);
            let intv: &Vec<BencodeItem> = extv.try_as_ref()?;
            for item in intv {
                let owned = match item {
                    BencodeItem::Str(x) => Some(x.to_owned()),
                    BencodeItem::Bin(x) => Some(owned_str_from_bin(x.to_owned())?),
                    _ => None
                };
                vec[idx].push(owned?);
            }
        }
        Some(vec)
    }

    fn get_piece_size(root: &Pair) -> Option<i64> {
        let x = get_pair_by_key("info", root)?;
        Some(*get_num_by_key("piece length", x)?)
    }

    fn get_files(root: &Pair) -> Option<Vec<TorrentFile>> {
        let mut vec = vec![];
        let info = get_pair_by_key("info", root)?;
        let list = get_list_by_key("files", info);
        match list {
            None => {
                vec.push(TorrentFile {
                    path: get_owned_str_by_key("name", info)?,
                    size: *get_num_by_key("length", info)?
                });
            },
            Some(list) => {
                for item in list {
                    let item = item.try_as_ref()?;
                    let path = owned_str_from_item(
                        get_list_by_key("path", item)?.first()?
                    )?;
                    vec.push(TorrentFile {
                        path: path.to_owned(),
                        size: *get_num_by_key("length", item)?
                    })
                }
            }
        };
        Some(vec)
    }

    fn get_pieces(root: &Pair) -> Option<Vec<Sha1Hash>> {
        let x = get_pair_by_key("info", root)?;
        let x = get_bin_by_key("pieces", x)?;
        assert!(x.len() % 20 == 0);
        let mut buf = [0u8; 20];
        let mut vec = Vec::with_capacity(x.len() / 20);
        for range in (0..x.len()).step_by(20) {
            buf.copy_from_slice(&x[range..range+20]);
            vec.push(buf);
        }
        Some(vec)
    }

    fn get_info_hash(root: &Pair) -> Option<Sha1Hash> {
        let info = get_by_key("info", root)?;
        let data = BencodeEncoder::encode(info);
        Some(Sha1::digest(data).into())
    }
}

impl Torrent {
    pub fn new(root: &BencodeItem) -> Option<Self> {
        let root: &Pair = root.try_as_ref()?;
        Some(Self {
            announce: Self::get_announce(root)?,
            announce_list: Self::get_announce_list(root)?,
            files: Self::get_files(root)?,
            pieces: Self::get_pieces(root)?,
            piece_size: Self::get_piece_size(root)?,
            info_hash: Self::get_info_hash(root)?
        })
    }
}