use crate::types::*;

impl BencodeItem {
    fn size(&self) -> usize {
        //todo: bad way to calculate bencoded length..
        BencodeEncoder::encode(self).len()
    }
}

impl BencodeDecoder {
    fn find(from: &[u8], what: u8) -> Option<usize> {
        from.iter().position(|x| *x == what)
    }

    fn try_decode_isize(from: &[u8]) -> Option<isize> {
        let base = from.to_owned();
        let data = String::from_utf8(base).ok()?;
        Some(data.parse().ok()?)
    }

    fn try_decode_num(from: &[u8]) -> Option<BencodeItem> {
        let ipos = Self::find(from, b'i')?;
        let epos = Self::find(from, b'e')?;
        let base = &from[ipos+1..epos];
        let data = Self::try_decode_isize(base)?;
        Some(BencodeItem::Num(data as i64))
    }

    fn try_decode_bin(from: &[u8]) -> Option<BencodeItem> {
        let cpos = Self::find(from, b':')?;
        let base = &from[..cpos];
        let size = Self::try_decode_isize(base)? as usize;
        if size <= from[cpos+1..].len() {
            let data = from[cpos+1..cpos+1+size].to_owned();
            return Some(BencodeItem::Bin(data));
        }
        None
    }

    fn try_decode_str(from: &[u8]) -> Option<BencodeItem> {
        if let BencodeItem::Bin(base) = Self::try_decode_bin(from)? {
            let data = String::from_utf8(base).ok()?;
            return Some(BencodeItem::Str(data));
        }
        None
    }

    fn try_decode_list(from: &[u8]) -> Option<BencodeItem> {
        let mut list = Vec::new();
        let lpos = Self::find(from, b'l')?;
        let mut from = &from[lpos+1..];
        while from.len() > 0 && from[0] != b'e' {
            let item = Self::try_decode_any(from)?;
            from = &from[item.size()..];
            list.push(item);
        }
        Some(BencodeItem::List(list))
    }

    fn try_decode_pair(from: &[u8]) -> Option<BencodeItem> {
        let mut pair = Vec::new();
        let lpos = Self::find(from, b'd')?;
        let mut from = &from[lpos+1..];
        while from.len() > 0 && from[0] != b'e' {
            let key = Self::try_decode_str(from)?;
            from = &from[key.size()..];
            let val = Self::try_decode_any(from)?;
            from = &from[val.size()..];
            pair.push((key.try_into().ok()?, val));
        }
        Some(BencodeItem::Pair(pair))
    }

    fn try_decode_any(from: &[u8]) -> Option<BencodeItem> {
        let byte = from.first()?;
        match byte {
            b'i' => Self::try_decode_num(from),
            b'l' => Self::try_decode_list(from),
            b'd' => Self::try_decode_pair(from),
            _ => Self::try_decode_bin(from)
        }
    }
}

impl BencodeDecoder {
    #[inline(always)]
    pub fn decode(from: &[u8]) -> Option<BencodeItem> {
        Self::try_decode_any(from)
    }
}

impl TryInto<String> for BencodeItem {
    type Error = String;
    
    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            BencodeItem::Str(x) => Ok(x),
            BencodeItem::Bin(x) => String::from_utf8(x).map_err(|e| e.to_string()),
            _ => Err("Cannot convert to String, wrong BencodeItem".to_owned())
        }
    }
}

impl TryAsRef<str> for BencodeItem {
    fn try_as_ref(&self) -> Option<&str> {
        match &self {
            BencodeItem::Str(x) => Some(x),
            _ => None
        }
    }
}

impl TryAsRef<Vec<BencodeItem>> for BencodeItem {
    fn try_as_ref(&self) -> Option<&Vec<BencodeItem>> {
        match &self {
            BencodeItem::List(x) => Some(x),
            _ => None
        }
    }
}


impl TryAsRef<Vec<(String, BencodeItem)>> for BencodeItem {
    fn try_as_ref(&self) -> Option<&Vec<(String, BencodeItem)>> {
        match &self {
            BencodeItem::Pair(x) => Some(x),
            _ => None
        }
    }
}