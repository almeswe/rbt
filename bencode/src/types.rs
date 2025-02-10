use std::collections::HashMap;

pub type Bytes = Vec<u8>;
pub type List = Vec<BencodeItem>;
pub type Pair = HashMap<String, BencodeItem>;

pub trait Bencode {
    fn bencode_len(&self) -> usize;
    fn try_parse(from: &[u8]) -> Option<Self> 
        where Self: Sized;
}

#[derive(Clone, Debug, PartialEq)]
pub enum BencodeItem {
    Num(i64),
    Str(Bytes),
    List(List),
    Pair(Pair)
}

impl BencodeItem {
    pub fn size(&self) -> usize {
        //todo: refactor
        match self {
            BencodeItem::Num(x) => x.bencode_len(),
            BencodeItem::Str(x) => x.bencode_len(),
            BencodeItem::List(x) => x.bencode_len(),
            BencodeItem::Pair(x) => x.bencode_len()
        }
    }

    pub fn to_num(&self) -> Option<u64> {
        if let BencodeItem::Num(x) = self {
            return Some(*x as u64);
        }
        None
    }

    pub fn to_pair(&self) -> Option<&Pair> {
        if let BencodeItem::Pair(x) = self {
            return Some(x);
        }
        None
    }

    pub fn to_list(&self) -> Option<&List> {
        if let BencodeItem::List(x) = self {
            return Some(x);
        }
        None
    }

    pub fn to_bytes(&self) -> Option<&Bytes> {
        if let BencodeItem::Str(x) = self {
            return Some(x);
        }
        None
    }

    pub fn to_string(&self) -> Option<String> {
        if let BencodeItem::Str(x) = self {
            return String::from_utf8(x.to_vec()).ok();
        }
        None
    }

    pub fn try_parse(from: &[u8]) -> Option<Self> {
        if from.is_empty() {
            return None;
        }
        let byte = from[0];
        assert!(byte.is_ascii_graphic());
        return Some(match char::from(byte) {
            'i' => BencodeItem::Num(i64::try_parse(from)?),
            'l' => BencodeItem::List(List::try_parse(from)?),
            'd' => BencodeItem::Pair(Pair::try_parse(from)?),
            // `String::try_parse` is only used in `Pair`,
            // because we assume that a key will be valid UTF-8 string.
            // But as a standalone string, there can be anything.
            // ** All strings must be UTF-8 encoded, except for pieces, which contains binary data. **
            // (https://en.wikipedia.org/wiki/Torrent_file)
            _ => BencodeItem::Str(Bytes::try_parse(from)?)
        });
    }         
}