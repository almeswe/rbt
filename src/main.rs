use bencode::{dump::to_json_string, types::*};
use torrent::types::*;

fn test() -> Option<i32> {
    let data = std::fs::read("/home/almeswe/Projects/dev/rbt/data/t1.torrent").unwrap();
    let root = BencodeItem::try_parse(&data)?;
    //println!("{x}", x=to_json_string(&root).unwrap());
    let file = Torrent::try_new(root.to_pair()?)?;
    println!("announce     : {x:?}", x = file.announce);
    println!("announce-list: {x:?}", x = file.announce_list);
    println!("files        : {x:?}", x = file.files);
    println!("pieces       : {x}*20 bytes", x = file.pieces.len());
    println!("piece        : {x} bytes", x = file.piece_size);
    //let json = to_json_string(&root).unwrap();
    //println!("{json}");
    Some(0)
}

fn main() {
    test().unwrap();
}