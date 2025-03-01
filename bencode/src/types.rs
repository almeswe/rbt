#[derive(Debug, PartialEq)]
pub enum BencodeItem {
    Num(i64),
    Bin(Vec<u8>),
    Str(String),
    List(Vec<BencodeItem>),
    // to save item order, use default vector 
    Pair(Vec<(String, BencodeItem)>)
}

pub struct BencodeDecoder;
pub struct BencodeEncoder;