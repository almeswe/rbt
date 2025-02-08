use bencode::types::*;
use torrent::types::*;

fn test() -> Option<i32> {
    let data = std::fs::read("/home/almeswe/Projects/dev/rbt/data/t1.torrent").unwrap();
    let root = BencodeItem::try_parse(&data)?;
    let file = Torrent::try_new(root.to_pair()?)?;
    println!("name    : {x}", x = file.name);
    println!("announce: {x}", x = file.announce);
    println!("files   : {x:?}", x = file.files);
    println!("pieces  : {x}*20 bytes", x = file.pieces.len());
    println!("piece   : {x} bytes", x = file.piece_size);
    //let json = to_json_string(&root).unwrap();
    //println!("{json}");
    Some(0)
}

fn main() {
    test().unwrap();
}