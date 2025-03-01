use crate::types::*;

impl BencodeEncoder {
    fn encode_utf8_str(item: &str, to: &mut Vec<u8>) {
        item.as_bytes().iter().for_each(|x| to.push(*x));
    }

    fn encode_num(item: &i64, to: &mut Vec<u8>) {
        let repr = item.to_string();
        to.push(b'i');
        Self::encode_utf8_str(&repr, to);
        to.push(b'e');
    }

    fn encode_bin(item: &[u8], to: &mut Vec<u8>) {
        let repr = item.len().to_string();
        Self::encode_utf8_str(&repr, to);
        to.push(b':');
        item.iter().for_each(|x| to.push(*x));
    }

    fn encode_str(item: &str, to: &mut Vec<u8>) {
        Self::encode_bin(item.as_bytes(), to);
    }

    fn encode_list(item: &Vec<BencodeItem>, to: &mut Vec<u8>) {
        to.push(b'l');
        item.iter().for_each(|x| 
            Self::encode_any(&x, to)
        );
        to.push(b'e');
    }

    fn encode_pair(item: &Vec<(String, BencodeItem)>, to: &mut Vec<u8>) {
        to.push(b'd');
        item.iter().for_each(|x| {
            Self::encode_str(&x.0, to);
            Self::encode_any(&x.1, to);
        });
        to.push(b'e');
    }

    fn encode_any(item: &BencodeItem, to: &mut Vec<u8>) {
        match item {
            BencodeItem::Num(x) => Self::encode_num(&x, to),
            BencodeItem::Bin(x) => Self::encode_bin(&x, to),
            BencodeItem::Str(x) => Self::encode_str(&x, to),
            BencodeItem::List(x) => Self::encode_list(&x, to),
            BencodeItem::Pair(x) => Self::encode_pair(&x, to),
        }
    }
}

impl BencodeEncoder {
    #[inline(always)]
    pub fn encode(any: &BencodeItem) -> Vec<u8> {
        let mut bytes = Vec::new();
        Self::encode_any(any, &mut bytes);
        bytes
    }
}