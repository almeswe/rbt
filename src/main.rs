//use bencode::dump::to_json_string;
//use bencode::types::*;
//use tokio::io::AsyncReadExt;
//use tokio::io::AsyncWriteExt;
//use torrent::types::*;
//use torrent::format::*;

use std::str::FromStr;

use bencode::types::BencodeDecoder;
use torrent::types::*;
use torrent::format::*;

use tokio::net::{TcpSocket, TcpStream};
use std::net::{Ipv4Addr, SocketAddrV4, SocketAddr};

//async fn test() -> Option<i32> {
//    let data = std::fs::read("/home/almeswe/Projects/dev/rbt/data/t1.torrent").ok()?;
//    let item = BencodeItem::try_parse(&data)?;
//    let root = Torrent::try_new(item.into_pair()?)?;
//    //dbg!("here");
//    //let mut stream = connnect(&root).await;
//    //dbg!("handshake..");
//    //stream = handshake(stream, &root).await;
//    //stream.shutdown().await.unwrap();
//    //println!("announce     : {x:?}", x = root.announce);
//    //println!("announce-list: {x:?}", x = root.announce_list);
//    //println!("files        : {x:?}", x = root.files);
//    //println!("pieces       : {x}*20 bytes", x = root.pieces.len());
//    //println!("piece        : {x} bytes", x = root.piece_size);
//    //println!("info hash    : 0x{x:x?}", x = root.info_hash);
//    root.get_peers().await.unwrap();
//    Some(0)
//}

//async fn connnect<'a>(_torrent: &Torrent<'a>) -> TcpStream {
//    let socket = TcpSocket::new_v4().unwrap();
//    let addrv4 = SocketAddrV4::new(
//        Ipv4Addr::from_str("81.5.94.65").unwrap(),
//        49763
//    );
//    dbg!("connecting..");
//    socket.connect(SocketAddr::V4(addrv4)).await.unwrap()
//}
//
//async fn handshake<'a>(mut stream: TcpStream, torrent: &Torrent<'a>) -> TcpStream {
//    let mut handshake = Vec::new();
//    handshake.push(19);
//    handshake.extend_from_slice(b"BitTorrent protocol");
//    handshake.extend_from_slice(&[0; 8]);
//    handshake.extend_from_slice(&torrent.info_hash);
//    handshake.extend_from_slice(&torrent.peer_id);
//    stream.write_all(&handshake).await.unwrap();
//
//    let mut response = [0; 68];
//    let _ = stream.read_exact(&mut response).await;
//    stream
//}

#[tokio::main]
async fn main() {
    let data = std::fs::read("/home/almeswe/Projects/dev/rbt/data/t1.torrent").unwrap();
    let root = BencodeDecoder::decode(&data).unwrap();
    let torrent = Torrent::new(&root).unwrap();
    println!("{torrent:?}");
    //let mut parser = BencodeParser::new();
    //let root = parser.try_parse(&data).unwrap();
    //println!("{x}", x = to_json_string(&root).unwrap());
    //test().await.unwrap();
}