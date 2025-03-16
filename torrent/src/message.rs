use std::arch::x86_64;

use crate::types::*;

const PEER_MSG_CHOKE: u8 = 0u8;
const PEER_MSG_UNCHOKE: u8 = 1u8;
const PEER_MSG_INTERESTED: u8 = 2u8;
const PEER_MSG_NOT_INTERESTED: u8 = 3u8;
const PEER_MSG_HAVE: u8 = 4u8;
//const PEER_MSG_BITFIELD: u8 = 5u8;
const PEER_MSG_REQUEST: u8 = 6u8;
//const PEER_MSG_PIECE: u8 = 7u8;
//const PEER_MSG_CANCEL: u8 = 8u8;
//const PEER_MSG_PORT: u8 = 9u8;

impl AsRef<[u8]> for PeerMsg {
    fn as_ref(&self) -> &[u8] {
        match self {
            PeerMsg::Have(x)       |
            PeerMsg::Choke(x)      |
            PeerMsg::Unchoke(x)    | 
            PeerMsg::Request(x)    | 
            PeerMsg::BitField(x)   |
            PeerMsg::KeepAlive(x)  |
            PeerMsg::Handshake(x)  |
            PeerMsg::Interested(x) |
            PeerMsg::NotInterested(x) => &x 
        }
    }
}

impl PeerMsg {
    fn u32_from_be(from: &[u8]) -> u32 {
        assert!(from.len() >= 4);
        (from[0] as u32) << 24 |
        (from[1] as u32) << 16 |
        (from[2] as u32) <<  8 |
        (from[3] as u32)
    } 

    fn try_parse_have(length: u32, from: &[u8]) -> Option<Self> {
        if length != 4 {
            return None;
        }
        let piece = Self::u32_from_be(&from[5..]);
        Some(Self::new_have(piece))
    }

    fn try_parse_handshake(from: &[u8]) -> Option<Self> {
        if from.len() < 68 || from[0] != 19 {
            return None;
        }
        if b"BitTorrent protocol" != &from[1..20] {
            return None;
        }
        Some(Self::Handshake(from[..69].to_vec()))
    }

    pub fn try_parse(from: &[u8]) -> Option<Self> {
        if from.len() < 4 {
            return None;
        }
        let length = Self::u32_from_be(&from);
        if length == 0 {
            return Some(Self::new_keep_alive());
        }
        let message = from[4];
        if length == 1 {
            return match message {
                PEER_MSG_CHOKE          => Some(Self::new_choke()),
                PEER_MSG_UNCHOKE        => Some(Self::new_unchoke()),
                PEER_MSG_INTERESTED     => Some(Self::new_interested()),
                PEER_MSG_NOT_INTERESTED => Some(Self::new_not_interested()),
                _ => None
            };
        }
        return match message {
            PEER_MSG_HAVE => Self::try_parse_have(length as u32, from),
            //PEER_MSG_BITFIELD => Self::try_parse_bitfield(from),
            //PEER_MSG_REQUEST => Self::try_parse_request(from),
            //PEER_MSG_PIECE => Self::try_parse_piece(from),
            //PEER_MSG_CANCEL => Self::try_parse_cancel(from),
            //PEER_MSG_PORT => Self::try_parse_port(from),
            _ => Self::try_parse_handshake(from)
        };
    }
}

impl PeerMsg {
    pub fn new_keep_alive() -> Self {
        let mut message = Vec::with_capacity(1);
        message.extend((0 as u32).to_be_bytes());
        Self::KeepAlive(message)
    }

    pub fn new_choke() -> Self {
        let mut message = Vec::with_capacity(5);
        message.extend((1 as u32).to_be_bytes());
        message.push(PEER_MSG_CHOKE);
        Self::Choke(message)
    }

    pub fn new_unchoke() -> Self {
        let mut message = Vec::with_capacity(5);
        message.extend((1 as u32).to_be_bytes());
        message.push(PEER_MSG_UNCHOKE);
        Self::Unchoke(message)
    }

    pub fn new_interested() -> Self {
        let mut message = Vec::with_capacity(5);
        message.extend((1 as u32).to_be_bytes());
        message.push(PEER_MSG_INTERESTED);
        Self::Interested(message)
    }

    pub fn new_not_interested() -> Self {
        let mut message = Vec::with_capacity(5);
        message.extend((1 as u32).to_be_bytes());
        message.push(PEER_MSG_NOT_INTERESTED);
        Self::NotInterested(message)
    }

    pub fn new_have(piece: u32) -> Self {
        let mut message = Vec::with_capacity(9);
        message.extend((5 as u32).to_be_bytes());
        message.push(PEER_MSG_HAVE);
        message.extend(piece.to_be_bytes());
        Self::Have(message)
    }

    pub fn new_request(index: usize, begin: usize, length: usize) -> Self {
        let mut message = Vec::with_capacity(14);
        message.extend((13 as u32).to_be_bytes());
        message.push(PEER_MSG_REQUEST);
        message.extend((index as u32).to_be_bytes());
        // todo: what if size is greater than ~4.3 GiB?
        let begin = (begin as u32).wrapping_mul(length as u32);
        message.extend(begin.to_be_bytes());
        message.extend((index as u32).to_be_bytes());
        Self::Request(message)
    }

    pub fn new_handshake(tracker: &Tracker) -> Self {
        let proto = b"BitTorrent protocol";
        let mut message = Vec::with_capacity(49 + proto.len());
        // size of greet message (in bytes)
        message.push(proto.len() as u8);
        // protocol string
        message.extend(proto);
        // reserved 8 zero bytes
        //todo: optimize this?
        for _ in 0..4 {
            message.push(0);
            message.push(0);
        }
        // info_hash, 20 bytes
        message.extend(tracker.torrent.info_hash);
        // peer_id, 20 bytes too
        message.extend(tracker.peer_id);
        // assert that vec's capacity was calculated correctly
        assert_eq!(message.len(), 49 + proto.len());
        Self::Handshake(message)
    }
}