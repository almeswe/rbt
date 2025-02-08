use crate::types::*;

#[test]
fn test_try_decode_i64() {
    assert_eq!(None, i64::try_parse(b""));
    assert_eq!(None, i64::try_parse(b"i"));
    assert_eq!(None, i64::try_parse(b"i-"));
    assert_eq!(None, i64::try_parse(b"i-123-11-"));
    assert_eq!(None, i64::try_parse(b"ie"));
    assert_eq!(None, i64::try_parse(b"i-e"));
    assert_eq!(Some(0), i64::try_parse(b"i0e"));
    assert_eq!(Some(123), i64::try_parse(b"i123e"));
    assert_eq!(Some(-23), i64::try_parse(b"i-23e"));
}

#[test]
fn test_try_decode_string() {
    assert_eq!(None, String::try_parse(b""));
    assert_eq!(None, String::try_parse(b"5:abcd"));
    assert_eq!(None, String::try_parse(b"-4:abcd"));
    assert_eq!(Some("abcd".to_string()), String::try_parse(b"4:abcd"));
    assert_eq!(Some("".to_string()), String::try_parse(b"0:"));
    assert_eq!(Some("A".to_string()), String::try_parse(b"1:A"));
    assert_eq!(Some("A".to_string()), String::try_parse(b"1:ABCDE"));
}

#[test]
fn test_try_decode_list() {
    assert_eq!(None, List::try_parse(b""));
    assert_eq!(Some(vec![]), List::try_parse(b"le"));
    assert_eq!(Some(vec![BencodeItem::Num(42)]), List::try_parse(b"li42ee"));
    assert_eq!(Some(vec![BencodeItem::Str(b"test".to_vec())]), List::try_parse(b"l4:teste"));
    assert_eq!(
        Some(vec![
            BencodeItem::Num(1),
            BencodeItem::Str(b"hello".to_vec())
        ]),
        List::try_parse(b"li1e5:helloe")
    );
    assert_eq!(
        Some(vec![
            BencodeItem::List(vec![
                BencodeItem::Num(1),
                BencodeItem::Str(b"nested".to_vec())
            ]),
            BencodeItem::Str(b"list".to_vec()),
        ]),
        List::try_parse(b"lli1e6:nestede4:liste")
    );
}

#[test]
fn test_try_decode_pair() {
    assert_eq!(None, Pair::try_parse(b""));
    assert_eq!(Some(Pair::new()), Pair::try_parse(b"de"));
    let mut pair1 = Pair::new();
    pair1.insert(String::from("wiki"), BencodeItem::Str(b"bencode".to_vec()));
    pair1.insert(String::from("meaning"), BencodeItem::Num(42));
    assert_eq!(Some(pair1.clone()), Pair::try_parse(b"d4:wiki7:bencode7:meaningi42ee"));
    let mut pair2 = Pair::new();
    pair2.insert(String::from("list"), BencodeItem::List(vec![
        BencodeItem::Num(1),
        BencodeItem::Str(b"str2".to_vec()),
        BencodeItem::Pair(pair1)
    ]));
    //"d4:list l i1e 4:str2 d4:wiki7:bencode7:meaningi42ee e e"
    assert_eq!(Some(pair2), Pair::try_parse(b"d4:listli1e4:str2d4:wiki7:bencode7:meaningi42eeee"));
}