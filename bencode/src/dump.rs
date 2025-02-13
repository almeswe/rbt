use serde::Serializer;
use serde::ser::{
    Serialize,
    SerializeSeq,
    SerializeMap
};
use crate::types::*; 

fn serialize_num<S: Serializer>(x: i64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(x)
}

fn serialize_bin<S: Serializer>(x: &Bytes, serializer: S) -> Result<S::Ok, S::Error> {
    let err = String::from_utf8(x.to_vec()).ok();
    if let Some(x) = err {
        serializer.serialize_str(&x)
    }
    else {
        serializer.serialize_bytes(&x)
    }
}

fn serialize_str<S: Serializer>(x: &String, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(x)
}

fn serialize_list<S: Serializer>(x: &List, serializer: S) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_seq(Some(x.len()))?;
    for item in x {
        seq.serialize_element(item)?;
    }
    seq.end()
}

fn serialize_pair<S: Serializer>(x: &Pair, serializer: S) -> Result<S::Ok, S::Error> {
    let mut seq = serializer.serialize_map(Some(x.len()))?;
    for item in x {
        seq.serialize_entry(item.0, item.1)?;
    }
    seq.end()
}

impl<'a> Serialize for BencodeItem<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match &self.data {
            BencodeExact::Num(x) => serialize_num(*x, serializer),
            BencodeExact::Bin(x) => serialize_bin(x, serializer),
            BencodeExact::Str(x) => serialize_str(x, serializer),
            BencodeExact::List(x) => serialize_list(x, serializer),
            BencodeExact::Pair(x) => serialize_pair(x, serializer)
        }
    }
}

pub fn to_json_string(item: &BencodeItem) -> Option<String> {
    serde_json::to_string(&item).ok()
}