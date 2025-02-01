use crate::types::*;

impl Bencode for i64 {
    fn try_parse(from: &str) -> Option<i64> {
        let ipos = from.find('i')?;
        let epos = from.find('e')?;
        from[ipos+1..epos].parse::<i64>().ok()
    }
}

impl Bencode for String {
    fn try_parse(from: &str) -> Option<String> {
        let cpos = from.find(':')?;
        let size = from[..cpos].parse::<usize>().ok()?;
        if size > from[cpos+1..].len() {
            return None;
        }
        Some(String::from(&from[cpos+1..cpos+1+size]))
    }
}

impl Bencode for Vec<BencodeItem> {
    fn try_parse(from: &str) -> Option<Vec<BencodeItem>> {
        if !from.starts_with('l') {
            return None;
        }
        let mut list = vec![];
        let mut from = &from[1..];
        while from.len() > 0 && !from.starts_with('e') {
            list.push(BencodeItem::try_parse(from)?);
            from = &from[list.last()?.size()..];
        }
        if !from.starts_with('e') {
            return None;
        }
        Some(list)
    }
}