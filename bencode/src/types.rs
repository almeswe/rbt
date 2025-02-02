use std::collections::HashMap;

pub type Bytes = Vec<u8>;
pub type List = Vec<BencodeItem>;
pub type Pair = HashMap<String, BencodeItem>;

pub trait Bencode {
    fn bsize(&self) -> usize;
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
            BencodeItem::Num(x) => x.bsize(),
            BencodeItem::Str(x) => x.bsize(),
            BencodeItem::List(x) => x.bsize(),
            BencodeItem::Pair(x) => x.bsize()
        }
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