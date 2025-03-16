use crate::types::*;
use std::{io::{Read, Write}, net::{SocketAddr, SocketAddrV4, TcpStream}, time::Duration};

use std::io::{Result, Error, ErrorKind};

impl Peer {
    pub fn new(timeout: u32, addr: SocketAddrV4) -> Self {
        Self { timeout, bitfield: None, choked: true, addr }
    }

    fn connect(&self) -> Result<TcpStream> {
        let addr = SocketAddr::V4(self.addr);
        let timeout = Duration::from_millis(self.timeout as u64);
        let conn = TcpStream::connect_timeout(&addr, timeout)?;
        conn.set_nodelay(true)?;
        conn.set_read_timeout(Some(timeout))?;
        conn.set_write_timeout(Some(timeout))?;
        tracing::info!("[CONNECTED] {addr}", addr = &self.addr);
        Ok(conn)
    }

    pub fn handshake(&mut self, tracker: &Tracker) -> Result<TcpStream> {
        let mut conn = self.connect()?;
        let handshake = PeerMsg::new_handshake(tracker);
        conn.write(handshake.as_ref())?;
        let mut data = [0u8; 68];
        conn.read_exact(&mut data)?;
        let interested = PeerMsg::new_interested();
        conn.write(interested.as_ref())?;
        // read what peer says..
        let mut data = Vec::new();
        conn.read(&mut data)?;
        let mut data = &data[..];
        while data.len() != 0 {
            let message = PeerMsg::try_parse(&data).unwrap();
            match &message {
                PeerMsg::Have(_)       => (),
                PeerMsg::Choke(_)      => self.choked = true,
                PeerMsg::Unchoke(_)    => self.choked = false,
                PeerMsg::BitField(_) => (),//self.bitfield = Some(vec.clone()),
                _ => {
                    return Err(Error::new(ErrorKind::Unsupported, "Unsupported message in response"))
                }
            }
            data = &data[message.as_ref().len()..];
        }
        if self.choked {
            return Err(Error::new(ErrorKind::ConnectionRefused, "Choked"));
        }
        tracing::info!("[HANDSHAKED] {addr}", addr = &self.addr);
        Ok(conn)
    }

    pub fn request(&self, index: usize, piece_length: usize, conn: &mut TcpStream) -> Result<Vec<u8>> {
        let request = PeerMsg::new_request(index, index * piece_length, piece_length);
        conn.write(request.as_ref())?;
        let mut data = Vec::with_capacity(piece_length);
        conn.read_to_end(&mut data)?;
        Ok(data[14..].to_vec())
    }
}