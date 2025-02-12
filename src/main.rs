use bencode::dump::to_json_string;
use bencode::types::*;

fn test() -> Option<i32> {
    let data = std::fs::read("path").unwrap();
    let root = BencodeItem::try_parse(&data)?;
    println!("{x:?}", x = to_json_string(&root));
    Some(0)
}

fn main() {
    test().unwrap();
}