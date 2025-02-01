use std::collections::HashMap;

pub trait Bencode {
    fn try_parse(from: &str) -> Option<Self> 
        where Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum BencodeItem {
    Num(i64),
    Str(String),
    List(Vec<BencodeItem>),
    Pair(HashMap<String, BencodeItem>)
}

impl BencodeItem {
    pub fn size(&self) -> usize {
        //todo: refactor
        match self {
            BencodeItem::Num(x) => x.to_string().len() + 2,
            BencodeItem::Str(x) => x.len() + x.len().to_string().len() + 1,
            BencodeItem::List(x) => x.iter().map(|z| z.size()).sum::<usize>() + 2,
            _ => todo!()
            //BencodeItem::Pair(x) => x.iter().map(|z| z.0.size()).sum::<usize>() +
            //                        x.iter().map(|z| z.1.size()).sum::<usize>()
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
            'l' => BencodeItem::List(Vec::try_parse(from)?),
            //todo: return here.
            _ => BencodeItem::Str(String::try_parse(from)?)
        });
    }         
}