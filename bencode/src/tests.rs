use crate::types::*;

#[allow(non_camel_case_types)]
type de = BencodeDecoder;
#[allow(non_camel_case_types)]
type en = BencodeEncoder;

fn test_decode_n_encode(from: &[u8]) {
    let item = de::decode(from).unwrap();
    let item = en::encode(&item);
    assert_eq!(from, item);
}

#[test]
fn test_decode_n_encode_num() {
    //#######################################
    assert_eq!(None, de::decode(b""));
    assert_eq!(None, de::decode(b"i"));
    assert_eq!(None, de::decode(b"i-"));
    assert_eq!(None, de::decode(b"i-e"));
    assert_eq!(None, de::decode(b"ie"));
    assert_eq!(None, de::decode(b"i----132e"));
    //#######################################
    test_decode_n_encode(b"i43e");
    test_decode_n_encode(b"i-3e");
}

#[test]
fn test_decode_n_encode_bin_or_str() {
    //#######################################
    assert_eq!(None, de::decode(b"4:123"));
    assert_eq!(None, de::decode(b"4:"));
    //#######################################
    test_decode_n_encode(b"4:1234");
}

#[test]
fn test_decode_n_encode_list() {
    test_decode_n_encode(b"le");
    test_decode_n_encode(b"li42ee");
    test_decode_n_encode(b"l4:teste");
    test_decode_n_encode(b"li1e5:helloe");
    test_decode_n_encode(b"lli1e6:nestede4:liste");
}

#[test]
fn test_decode_n_encode_pair() {
    test_decode_n_encode(b"de");
    test_decode_n_encode(b"d4:wiki7:bencode7:meaningi42ee");
    test_decode_n_encode(b"d4:listli1e4:str2d4:wiki7:bencode7:meaningi42eeee");
}