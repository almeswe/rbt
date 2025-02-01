use crate::types::*;
use crate::parse::*;

#[test]
fn test_try_decode_i64() {
    assert_eq!(None, i64::try_parse(""));
    assert_eq!(None, i64::try_parse("i"));
    assert_eq!(None, i64::try_parse("i-"));
    assert_eq!(None, i64::try_parse("i-123-11-"));
    assert_eq!(None, i64::try_parse("ie"));
    assert_eq!(None, i64::try_parse("i-e"));
    assert_eq!(Some(0), i64::try_parse("i0e"));
    assert_eq!(Some(123), i64::try_parse("i123e"));
    assert_eq!(Some(-23), i64::try_parse("i-23e"));
}

#[test]
fn test_try_decode_string() {
    assert_eq!(None, String::try_parse(""));
    assert_eq!(None, String::try_parse("5:abcd"));
    assert_eq!(None, String::try_parse("-4:abcd"));
    assert_eq!(Some("abcd".to_string()), String::try_parse("4:abcd"));
    assert_eq!(Some("".to_string()), String::try_parse("0:"));
    assert_eq!(Some("A".to_string()), String::try_parse("1:A"));
    assert_eq!(Some("A".to_string()), String::try_parse("1:ABCDE"));
}

#[test]
fn test_try_decode_list() {
    assert_eq!(None, List::try_parse(""));
    assert_eq!(Some(vec![]), List::try_parse("le"));
    assert_eq!(Some(vec![BencodeItem::Num(42)]), List::try_parse("li42ee"));
    assert_eq!(Some(vec![BencodeItem::Str(String::from("test"))]), List::try_parse("l4:teste"));
    assert_eq!(
        Some(vec![
            BencodeItem::Num(1),
            BencodeItem::Str(String::from("hello"))
        ]),
        List::try_parse("li1e5:helloe")
    );
    assert_eq!(
        Some(vec![
            BencodeItem::List(vec![
                BencodeItem::Num(1),
                BencodeItem::Str(String::from("nested"))
            ]),
            BencodeItem::Str(String::from("list")),
        ]),
        List::try_parse("lli1e6:nestede4:liste")
    );
}

#[test]
fn test_try_decode_pair() {
    assert_eq!(None, Pair::try_parse(""));
    assert_eq!(Some(Pair::new()), Pair::try_parse("de"));
    let mut pair1 = Pair::new();
    pair1.insert(String::from("wiki"), BencodeItem::Str(String::from("bencode")));
    pair1.insert(String::from("meaning"), BencodeItem::Num(42));
    assert_eq!(Some(pair1.clone()), Pair::try_parse("d4:wiki7:bencode7:meaningi42ee"));
    let mut pair2 = Pair::new();
    pair2.insert(String::from("list"), BencodeItem::List(vec![
        BencodeItem::Num(1),
        BencodeItem::Str(String::from("str2")),
        BencodeItem::Pair(pair1)
    ]));
    //"d4:list l i1e 4:str2 d4:wiki7:bencode7:meaningi42ee e e"
    assert_eq!(Some(pair2), Pair::try_parse("d4:listli1e4:str2d4:wiki7:bencode7:meaningi42eeee"));
}