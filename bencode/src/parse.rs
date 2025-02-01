use crate::types::*;

impl Bencode for i64 {
    fn bsize(&self) -> usize {
        self.to_string().len() + 2
    }
    
    fn try_parse(from: &str) -> Option<i64> {
        let ipos = from.find('i')?;
        let epos = from.find('e')?;
        from[ipos+1..epos].parse::<i64>().ok()
    }
}

impl Bencode for String {
    fn bsize(&self) -> usize {
        self.len() + self.len().to_string().len() + 1
    }
    
    fn try_parse(from: &str) -> Option<String> {
        let cpos = from.find(':')?;
        let size = from[..cpos].parse::<usize>().ok()?;
        if size > from[cpos+1..].len() {
            return None;
        }
        Some(String::from(&from[cpos+1..cpos+1+size]))
    }
}

impl Bencode for List {
    fn bsize(&self) -> usize {
        self.iter().map(|z| z.size()).sum::<usize>() + 2
    }

    fn try_parse(from: &str) -> Option<List> {
        if !from.starts_with('l') {
            return None;
        }
        let mut list = List::new();
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

impl Bencode for Pair {
    fn bsize(&self) -> usize {
        self.iter().map(|z| z.0.bsize() + z.1.size()).sum::<usize>() + 2
    }
    
    fn try_parse(from: &str) -> Option<Pair> {
        if !from.starts_with('d') {
            return None;
        }
        let mut pair = Pair::new();
        let mut from = &from[1..];
        while from.len() > 0 && !from.starts_with('e') {
            let key = String::try_parse(from)?;
            from = &from[key.bsize()..];
            let val = BencodeItem::try_parse(from)?;
            from = &from[val.size()..];
            if let Some(_) = pair.insert(key, val) {
                //todo: some value inserted twice..
                unimplemented!();
            }
        }
        if !from.starts_with('e') {
            return None;
        }
        Some(pair)
    }
}