use std::collections::HashMap;

pub type List = Vec<BencodeItem>;
pub type Pair = HashMap<String, BencodeItem>;

pub trait Bencode {
    fn bsize(&self) -> usize;
    fn try_parse(from: &str) -> Option<Self> 
        where Self: Sized;
}

#[derive(Clone, Debug, PartialEq)]
pub enum BencodeItem {
    Num(i64),
    Str(String),
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

    pub fn try_parse(from: &str) -> Option<Self> {
        if from.is_empty() {
            return None;
        }
        let from = from.strip_prefix("")?;
        let byte = from.as_bytes()[0];
        assert!(byte.is_ascii_graphic());
        return Some(match char::from(byte) {
            'i' => BencodeItem::Num(i64::try_parse(from)?),
            'l' => BencodeItem::List(List::try_parse(from)?),
            'd' => BencodeItem::Pair(Pair::try_parse(from)?),
            _ => BencodeItem::Str(String::try_parse(from)?)
        });
    }         
}