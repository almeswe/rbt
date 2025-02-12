use std::collections::HashMap;

pub type Bytes<'a> = [u8];
pub type List<'a> = Vec<BencodeItem<'a>>;
pub type Pair<'a> = HashMap<String, BencodeItem<'a>>;

#[derive(Debug)]
pub struct BencodeError {
    pub pos: u64,
    pub msg: Option<String>
}

pub struct BencodeParser<'a> {
    pub from: &'a [u8],
    pub data: Option<BencodeItem<'a>>
}

#[derive(Clone, Debug, PartialEq)]
pub enum BencodeExact<'a> {
    Num(i64),
    Str(String),
    Bin(&'a Bytes<'a>),
    List(Box<List<'a>>),
    Pair(Box<Pair<'a>>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct BencodeItem<'a> {
    pub from: &'a [u8],
    pub data: BencodeExact<'a>,
    pub hash: Option<Box<[u8; 20]>>
}