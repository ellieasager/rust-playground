use std::net::{Ipv4Addr, SocketAddrV4};

use crate::{
    errors::BitcoinMessageError,
    payload::{Payload, VersionMessage},
    utils::{parse_frombytes_le, read_drop_slice},
};

// const PROTOCOL_VERSION: i32 = 70016;

// const START_STRING: [u8; 4] = [0xf9, 0xbe, 0xb4, 0xd9];
pub const MAGIC_NUMBER: u32 = 0xD9B4BEF9;  //mainnet
// pub const MAGIC_NUMBER: u32 = 0xDAB5BFFA; //testnet

/// Message structure (see https://en.bitcoin.it/wiki/Protocol_documentation#Message_structure)
///
/// size | field    | type     | description
/// ---  | -----    | ----     | ------------
/// 4    | magic    | u32      | Magic value
/// 12   | command  | [u8; 12] | ASCII string i
/// 4    | length   | u32      | Length of payload in number of bytes
/// 4    | checksum | u32      | First 4 bytes of sha256(sha256(payload))
/// ?    | payload  | Vec<u8>  | The actual data
///
/// Defines a Bitcoin protocol message.
#[derive(Debug, Clone)]
pub struct BtcMessage {
    /// Magic bytes indicating the originating network; used to seek to next message when stream state is unknown.
    magic_number: u32,

    /// Identifies what message type is contained in the payload.
    pub command: [u8; 12],

    /// The payload of this message.
    payload: Payload,
    checksum: u32,
}

impl BtcMessage {
    /// Creates new [`Message`].
    pub fn new(command: [u8; 12], payload: Payload, checksum: u32) -> Self {
        Self {
            magic_number: MAGIC_NUMBER,
            command,
            payload,
            checksum,
        }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, BitcoinMessageError> {
        let mut buff = Vec::new();
        buff.extend_from_slice(&self.magic_number.to_le_bytes());
        buff.extend_from_slice(&self.command);
        buff.extend_from_slice(&(self.payload.to_bytes().unwrap().len() as u32).to_le_bytes());
        buff.extend_from_slice(&self.checksum.to_ne_bytes());
        buff.extend_from_slice(&self.payload.to_bytes().unwrap());
        Ok(buff)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Box<Self>, BitcoinMessageError> {
        println!("{:?}", data);
        let (magic, buff) = parse_frombytes_le::<u32>(&data.to_vec())?;
        let (cmd, buff) = read_drop_slice(&buff, 12)?;
        println!("{:?}", cmd);
        let command = <[u8; 12]>::try_from(cmd).unwrap();
        // let (length, buff) = parse_frombytes_le::<u32>(&buff)?;
        let (checksum, _) = parse_frombytes_le::<u32>(&buff)?;
        let address = SocketAddrV4::new(Ipv4Addr::new(162, 120,69, 182), 8333);
 

        Ok(Box::new(Self {
            magic_number: magic,
            command,
            payload: Payload::Version(VersionMessage::new(address)), // not checking the payload
            checksum,
        }))
        // Err(BitcoinMessageError::ChecksumMismatch)
    }
}
