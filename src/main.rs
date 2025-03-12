use torrent::types::*;
use bencode::types::BencodeDecoder;

#[tokio::main]
async fn main() {
    let data = std::fs::read("./t1.torrent").unwrap();
    let root = BencodeDecoder::decode(&data).unwrap();
    let torrent = Torrent::new(&root).unwrap();
    println!("{x:?}", x = torrent.announce);
    println!("{x:?}", x = torrent.announce_list);
    println!("{x:?}", x = torrent.files);
    println!("{x:?}", x = torrent.files);
    println!("{x:?}", x = torrent.piece_size);
    println!("{x:?}", x = torrent.info_hash);
}