use crate::types::*;

fn try_find(vec: &[u8], x: u8) -> Option<usize> {
    vec.iter().position(|&item| item == x)
}

fn try_parse_num<'a>(from: &'a [u8]) -> Option<BencodeItem<'a>> {
    let ipos = try_find(from, b'i')?;
    let epos = try_find(from, b'e')?;
    let data = String::from_utf8(from[ipos+1..epos].to_vec()).ok()?;
    let range = &from[ipos..epos+1]; 
    Some(BencodeItem::new_num(range, data.parse::<i64>().ok()?))
}

fn try_parse_bin<'a>(from: &'a [u8]) -> Option<BencodeItem<'a>> {
    let cpos = try_find(from, b':')?;
    let size = String::from_utf8(from[..cpos].to_vec()).ok()?;
    let size = size.parse::<usize>().ok()?;
    if size > from[cpos+1..].len() {
        return None;
    }
    let range = &from[..cpos+1+size];
    Some(BencodeItem::new_bin(range, &from[cpos+1..cpos+1+size]))
}

fn try_parse_str<'a>(from: &'a [u8]) -> Option<BencodeItem<'a>> {
    let bytes = try_parse_bin(from)?;
    return match &bytes.data {
        BencodeExact::Bin(x) => {
            let data = String::from_utf8(x.to_vec()).ok()?;
            return Some(BencodeItem::new_str(bytes.from, data));
        },
        _ => None
    };
}

fn try_parse_list<'a>(from: &'a [u8]) -> Option<BencodeItem<'a>> {
    if !from.starts_with(b"l") {
        return None;
    }
    let mut size = 1;
    let mut list = Box::new(List::new());
    let mut part = &from[size..];
    while size < from.len() && !part.starts_with(b"e") {
        let item = BencodeItem::try_parse(part)?;
        size += item.from.len();
        part = &from[size..];
        list.push(item);
    }
    if !part.starts_with(b"e") {
        return None;
    }
    Some(BencodeItem::new_list(&from[..size+1], list))
}

fn try_parse_pair<'a>(from: &'a [u8]) -> Option<BencodeItem<'a>> {
    if !from.starts_with(b"d") {
        return None;
    }
    let mut size = 1;
    let mut pair = Box::new(Pair::new());
    let mut part = &from[size..];
    while size < from.len() && !part.starts_with(b"e") {
        let key = try_parse_str(part)?;
        size += key.from.len();
        part = &from[size..];
        let val = BencodeItem::try_parse(part)?;
        size += val.from.len();
        part = &from[size..];
        let key = String::from(key.to_str()?);
        if let Some(_) = pair.insert(key, val) {
            //todo: some value inserted twice..
            unimplemented!();
        }
    }
    if !part.starts_with(b"e") {
        return None;
    }
    Some(BencodeItem::new_pair(&from[..size+1], pair))
}

impl<'a> BencodeItem<'a> {
    pub fn new_num(from: &'a [u8], x: i64) -> BencodeItem<'a> {
        BencodeItem { from: from, data: BencodeExact::Num(x), hash: None }
    }

    pub fn new_bin(from: &'a [u8], x: &'a Bytes) -> BencodeItem<'a> {
        BencodeItem { from: from, data: BencodeExact::Bin(x), hash: None }
    }

    pub fn new_str(from: &'a [u8], x: String) -> BencodeItem<'a> {
        BencodeItem { from: from, data: BencodeExact::Str(x), hash: None }
    }

    pub fn new_list(from: &'a [u8], x: Box<List<'a>>) -> BencodeItem<'a> {
        BencodeItem { from: from, data: BencodeExact::List(x), hash: None }
    }

    pub fn new_pair(from: &'a [u8], x: Box<Pair<'a>>) -> BencodeItem<'a> {
        BencodeItem { from: from, data: BencodeExact::Pair(x), hash: None }
    }

    pub fn to_num(&self) -> Option<i64> {
        if let BencodeExact::Num(x) = self.data {
            return Some(x);
        }
        None
    }

    pub fn to_str(&self) -> Option<&String> {
        if let BencodeExact::Str(x) = &self.data {
            return Some(x);
        }
        None
    }

    pub fn to_bin(&self) -> Option<&Bytes> {
        if let BencodeExact::Bin(x) = self.data {
            return Some(x);
        }
        None
    }

    pub fn to_list(&self) -> Option<&Box<List>> {
        if let BencodeExact::List(x) = &self.data {
            return Some(x);
        }
        None
    }

    pub fn to_pair(&self) -> Option<&Box<Pair>> {
        if let BencodeExact::Pair(x) = &self.data {
            return Some(x);
        }
        None
    }

    pub fn try_parse(from: &'a [u8]) -> Option<BencodeItem<'a>> {
        if from.is_empty() {
            return None;
        }
        let byte = from[0];
        assert!(byte.is_ascii_graphic());
        return Some(match char::from(byte) {
            'i' => try_parse_num(from)?,
            'l' => try_parse_list(from)?,
            'd' => try_parse_pair(from)?,
            // `String::try_parse` is only used in `Pair`,
            // because we assume that a key will be valid UTF-8 string.
            // But as a standalone string, there can be anything.
            // ** All strings must be UTF-8 encoded, except for pieces, which contains binary data. **
            // (https://en.wikipedia.org/wiki/Torrent_file)
            _ => try_parse_bin(from)?
        });
    }
}

impl<'a> BencodeParser<'a> {
    pub fn new(from: &'a [u8]) -> Self  {
        BencodeParser { from: from, data: None }
    }
}

#[test]
fn test_try_parse_num() {
    assert_eq!(None, try_parse_num(b""));
    assert_eq!(None, try_parse_num(b"i"));
    assert_eq!(None, try_parse_num(b"i-"));
    assert_eq!(None, try_parse_num(b"i-123-11-"));
    assert_eq!(None, try_parse_num(b"ie"));
    assert_eq!(None, try_parse_num(b"i-e"));
    assert_eq!(Some(0), try_parse_num(b"i0e").unwrap().to_num());
    assert_eq!(Some(123), try_parse_num(b"i123e").unwrap().to_num());
    assert_eq!(Some(-23), try_parse_num(b"i-23e").unwrap().to_num());
    assert_eq!(b"i0e", try_parse_num(b"i0e").unwrap().from);
    assert_eq!(b"i123e", try_parse_num(b"i123e").unwrap().from);
    assert_eq!(b"i-23e", try_parse_num(b"i-23e").unwrap().from);
}

#[test]
fn test_try_parse_str() {
    assert_eq!(None, try_parse_str(b""));
    assert_eq!(None, try_parse_str(b"5:abcd"));
    assert_eq!(None, try_parse_str(b"-4:abcd"));
    assert_eq!(Some(&String::from("abcd")), try_parse_str(b"4:abcd").unwrap().to_str());
    assert_eq!(Some(&String::from("")), try_parse_str(b"0:").unwrap().to_str());
    assert_eq!(Some(&String::from("A")), try_parse_str(b"1:A").unwrap().to_str());
    assert_eq!(Some(&String::from("A")), try_parse_str(b"1:ABCDE").unwrap().to_str());
    assert_eq!(b"4:abcd", try_parse_str(b"4:abcd").unwrap().from);
    assert_eq!(b"1:A", try_parse_str(b"1:A").unwrap().from);
    assert_eq!(b"1:A", try_parse_str(b"1:ABCD").unwrap().from);
    assert_eq!(b"10:123456789A", try_parse_str(b"10:123456789A").unwrap().from);
}

#[test]
fn test_try_parse_list() {
    assert_eq!(None, try_parse_list(b""));
    assert_eq!(Some(&Box::new(vec![])), try_parse_list(b"le").unwrap().to_list());
    assert_eq!(Some(&Box::new(
        vec![BencodeItem::new_num(b"i42e", 42)])),
        try_parse_list(b"li42ee").unwrap().to_list()
    );
    assert_eq!(Some(&Box::new(
        vec![BencodeItem::new_bin(b"4:test", b"test")])),
        try_parse_list(b"l4:teste").unwrap().to_list()
    );
    assert_eq!(
        Some(&Box::new(vec![
            BencodeItem::new_num(b"i1e", 1),
            BencodeItem::new_bin(b"5:hello", b"hello")
        ])),
        try_parse_list(b"li1e5:helloe").unwrap().to_list()
    );
    assert_eq!(
        Some(&Box::new(vec![
                BencodeItem::new_list(b"li1e6:nestede", Box::new(vec![
                    BencodeItem::new_num(b"i1e", 1),
                    BencodeItem::new_bin(b"6:nested", b"nested")
                ])),
                BencodeItem::new_bin(b"4:list", b"list"),
            ])
        ),
        try_parse_list(b"lli1e6:nestede4:liste").unwrap().to_list()
    );
    assert_eq!(b"li1e5:helloe", try_parse_list(b"li1e5:helloe").unwrap().from);
    assert_eq!(b"lli1e6:nestede4:liste", try_parse_list(b"lli1e6:nestede4:liste").unwrap().from);
}

#[test]
fn test_try_parse_pair() {
    assert_eq!(None, try_parse_pair(b""));
    assert_eq!(Some(&Box::new(Pair::new())), try_parse_pair(b"de").unwrap().to_pair());
    let mut pair1 = Pair::new();
    pair1.insert(String::from("wiki"), BencodeItem::new_bin(b"7:bencode", b"bencode"));
    pair1.insert(String::from("meaning"), BencodeItem::new_num(b"i42e", 42));
    assert_eq!(Some(&Box::new(pair1.clone())), try_parse_pair(b"d4:wiki7:bencode7:meaningi42ee").unwrap().to_pair());
    let mut pair2 = Pair::new();
    pair2.insert(String::from("list"),
        BencodeItem::new_list(b"li1e4:str2d4:wiki7:bencode7:meaningi42eee", Box::new(vec![
            BencodeItem::new_num(b"i1e", 1),
            BencodeItem::new_bin(b"4:str2", b"str2"),
            BencodeItem::new_pair(b"d4:wiki7:bencode7:meaningi42ee", Box::new(pair1))
        ]))
    );
    assert_eq!(Some(&Box::new(pair2)), try_parse_pair(b"d4:listli1e4:str2d4:wiki7:bencode7:meaningi42eeee").unwrap().to_pair());
    assert_eq!(b"d4:wiki7:bencode7:meaningi42ee", try_parse_pair(b"d4:wiki7:bencode7:meaningi42ee").unwrap().from);
    assert_eq!(b"d4:listli1e4:str2d4:wiki7:bencode7:meaningi42eeee", try_parse_pair(b"d4:listli1e4:str2d4:wiki7:bencode7:meaningi42eeee").unwrap().from);
}