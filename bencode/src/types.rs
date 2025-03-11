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

pub trait TryAsRef<T> where T: ?Sized {
    fn try_as_ref(&self) -> Option<&T>;
}