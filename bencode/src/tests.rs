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
    assert_eq!(None, Vec::try_parse(""));
    assert_eq!(Some(vec![]), Vec::try_parse("le"));
    assert_eq!(Some(vec![BencodeItem::Num(42)]), Vec::try_parse("li42ee"));
    assert_eq!(Some(vec![BencodeItem::Str(String::from("test"))]), Vec::try_parse("l4:teste"));
    assert_eq!(
        Some(vec![
            BencodeItem::Num(1),
            BencodeItem::Str(String::from("hello"))
        ]),
        Vec::try_parse("li1e5:helloe")
    );
    assert_eq!(
        Some(vec![
            BencodeItem::List(vec![
                BencodeItem::Num(1),
                BencodeItem::Str(String::from("nested"))
            ]),
            BencodeItem::Str(String::from("list")),
        ]),
        Vec::try_parse("lli1e6:nestede4:liste")
    );
}