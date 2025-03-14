use crate::types::*;
use std::{io::{Read, Write}, net::{SocketAddrV4, TcpStream}, net::SocketAddr, time::Duration};

//use tokio::{
//    io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream
//};

impl Peer {
    pub fn new(timeout: u32, addr: SocketAddrV4) -> Self {
        Self { timeout, choked: PeerStatus::PeerChoking, addr }
    }

    fn get_handshake_greeting(&self, peer_id: &[u8], info_hash: &Sha1Hash) -> Vec<u8> {
        let proto = b"BitTorrent protocol";
        let mut greet = Vec::with_capacity(49 + proto.len());
        // size of greet message (in bytes)
        greet.push(49 + proto.len() as u8);
        // protocol string
        greet.extend(proto);
        // reserved 8 zero bytes
        //todo: optimize this?
        for _ in 0..4 {
            greet.push(0);
            greet.push(0);
        }
        // info_hash, 20 bytes
        greet.extend(info_hash);
        // peer_id, 20 bytes too
        greet.extend(peer_id);
        // assert that vec's capacity was calculated correctly
        assert_eq!(greet.len(), 49 + proto.len());
        greet
    }

    //pub async fn handshake(&self, peer_id: &[u8], info_hash: &Sha1Hash) -> Option<TcpStream> {
    //    let greet = self.get_handshake_greeting(peer_id, info_hash);
    //    tracing::debug!("connecting to peer at {x}...", x = &self.addr);
    //    let mut stream = TcpStream::connect(self.addr).await.ok()?;
    //    stream.set_nodelay(true).ok()?;
    //    tracing::debug!("sending handshake {x}", x = &self.addr);
    //    stream.write(&greet).await.ok()?;
    //    tracing::debug!("handshake sent to {x}", x = &self.addr);
    //    //todo: use try_read for non-blockng.
    //    //stream.try_read();
    //    let mut tmp = [0u8; 256];
    //    //tokio::time::sleep(Duration::from_millis(self.timeout as u64)).await;
    //    tracing::debug!("reading handshake response from {x}", x = &self.addr);
    //    stream.read(&mut tmp).await.ok()?;
    //    //stream.try_read(&mut tmp).ok()?;
    //    //stream.try_read();
    //    Some(stream)
    //}

    pub fn handshake(&self, peer_id: &[u8], info_hash: &Sha1Hash) -> Option<TcpStream> {
        let greet = self.get_handshake_greeting(peer_id, info_hash);
        tracing::debug!("connecting to peer at {x}...", x = &self.addr);
        let sockaddr = SocketAddr::V4(self.addr);
        let mut stream = TcpStream::connect_timeout(&sockaddr, Duration::from_secs(1)).ok()?;
        stream.set_nodelay(true).ok()?;
        stream.set_read_timeout(Some(Duration::from_secs(1))).ok()?;
        stream.set_write_timeout(Some(Duration::from_secs(1))).ok()?;
        tracing::debug!("sending handshake {x}", x = &self.addr);
        stream.write(&greet).ok()?;
        tracing::debug!("handshake sent to {x}", x = &self.addr);
        //todo: use try_read for non-blockng.
        //stream.try_read();
        let mut tmp = [0u8; 256];
        //tokio::time::sleep(Duration::from_millis(self.timeout as u64)).await;
        tracing::debug!("reading handshake response from {x}", x = &self.addr);
        stream.read(&mut tmp).ok()?;
        //stream.try_read(&mut tmp).ok()?;
        //stream.try_read();
        Some(stream)
    }
}