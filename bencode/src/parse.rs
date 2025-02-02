use crate::types::*;

trait FindItem {
    fn find(&self, x: u8) -> Option<usize>;
}

impl FindItem for &[u8] {
    fn find(&self, x: u8) -> Option<usize> {
        //todo: optimize it?
        for i in 0..self.len() {
            if self[i] == x {
                return Some(i);
            }
        }
        None
    }
}

impl Bencode for i64 {
    fn bsize(&self) -> usize {
        self.to_string().len() + 2
    }
    
    fn try_parse(from: &[u8]) -> Option<i64> {
        let ipos = from.find(b'i')?;
        let epos = from.find(b'e')?;
        let from = String::from_utf8(from[ipos+1..epos].to_vec()).ok()?;
        from.parse::<i64>().ok()
    }
}

impl Bencode for Bytes {
    fn bsize(&self) -> usize {
        self.len() + self.len().to_string().len() + 1
    }
    
    fn try_parse(from: &[u8]) -> Option<Bytes> {
        let cpos = from.find(b':')?;
        let size = String::from_utf8(from[..cpos].to_vec()).ok()?;
        let size = size.parse::<usize>().ok()?;
        if size > from[cpos+1..].len() {
            return None;
        }
        Some(from[cpos+1..cpos+1+size].to_vec())
    }
}

impl Bencode for String {
    fn bsize(&self) -> usize {
        self.len() + self.len().to_string().len() + 1
    }
    
    fn try_parse(from: &[u8]) -> Option<String> {
        String::from_utf8(Bytes::try_parse(from)?).ok()
    }
}

impl Bencode for List {
    fn bsize(&self) -> usize {
        self.iter().map(|z| z.size()).sum::<usize>() + 2
    }

    fn try_parse(from: &[u8]) -> Option<List> {
        if !from.starts_with(b"l") {
            return None;
        }
        let mut list = List::new();
        let mut from = &from[1..];
        while from.len() > 0 && !from.starts_with(b"e") {
            list.push(BencodeItem::try_parse(from)?);
            from = &from[list.last()?.size()..];
        }
        if !from.starts_with(b"e") {
            return None;
        }
        Some(list)
    }
}

impl Bencode for Pair {
    fn bsize(&self) -> usize {
        self.iter().map(|z| z.0.bsize() + z.1.size()).sum::<usize>() + 2
    }
    
    fn try_parse(from: &[u8]) -> Option<Pair> {
        if !from.starts_with(b"d") {
            return None;
        }
        let mut pair = Pair::new();
        let mut from = &from[1..];
        while from.len() > 0 && !from.starts_with(b"e") {
            let key = String::try_parse(from)?;
            from = &from[key.bsize()..];
            let val = BencodeItem::try_parse(from)?;
            from = &from[val.size()..];
            if let Some(_) = pair.insert(key, val) {
                //todo: some value inserted twice..
                unimplemented!();
            }
        }
        if !from.starts_with(b"e") {
            return None;
        }
        Some(pair)
    }
}