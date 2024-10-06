use crate::{errors::BitcoinMessageError, raw_message::RawMessage, utils::*};
use std::io::Error;
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    time::UNIX_EPOCH,
};
// use bitflags::bitflags;
// use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use core::ops::BitAnd;

const PROTOCOL_VERSION: i32 = 70016;
/// Max payload size, as per Bitcoin protocol docs
const MAX_SIZE: usize = 32 * 1024 * 1024;

#[derive(Debug, Clone)]
/// Bitcoin's Message payload.
pub enum Payload {
    /// An empty payload.
    Empty,

    /// Payload of `version` command
    Version(VersionMessage),
}

impl Payload {
    //   // special case as it needs to know the command name
    //   /// Deserializes [`Payload`] from buffer of bytes.
    //   pub fn from_bytes(
    //       data: &mut impl Read,
    //       command: &Command,
    //   ) -> Result<Self, BitcoinMessageError> {
    //       match command {
    //           Command::Version => Ok(Payload::Version(VersionMessage::from_bytes(data)?)),
    //           Command::VerAck => Ok(Payload::Empty),
    //       }
    //   }
    // }

    // impl BitcoinSerialize for Payload {
    pub fn to_bytes(&self) -> Result<Vec<u8>, BitcoinMessageError> {
        let data = match self {
            Payload::Empty => Ok(vec![]),
            Payload::Version(data) => data.to_bytes(),
        };
        if let Ok(ref d) = data {
            if d.len() > MAX_SIZE {
                return Err(BitcoinMessageError::PayloadTooBig);
            }
        }

        data
    }
}

/// https://en.bitcoin.it/wiki/Protocol_documentation#version
///
/// size | field        | type     | description
/// ---  | -----        | ----     | ------------
/// 4    | version      | i32      | Identifies protocol version being used by the node
/// 8    | services     | u64      | bitfield of features to be enabled for this connection
/// 8    | timestamp    | i64      | standard UNIX timestamp in seconds
/// 26   | addr_recv    | net_addr | The network address of the node receiving this message
/// 26   | addr_from    | net_addr | Field can be ignored.
/// 8    | nonce        | u64      | Node random nonce
/// ?    | user_agent   | var_str  | User Agent (0x00 if string is 0 bytes long)
/// 4    | start_height | i32      | The last block received by the emitting node
/// 1    | relay        | bool     | Whether the remote peer should announce relayed transactions or not, see BIP 0037
/// *********************************************************
/// Almost all integers are encoded in little endian. Only IP or port number are encoded big endian.
/// *********************************************************

#[derive(Debug, Clone)]
pub struct VersionMessage {
    pub protocol_version: i32,
    pub service: u64,
    pub timestamp: i64,
    pub addr_recv: SocketAddrV4,
    pub addr_from: SocketAddrV4,
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: i32,
}

impl VersionMessage {
    pub fn new(addr_recv: SocketAddrV4) -> Self {
      let timestamp = Self::calculate_timestamp();
      VersionMessage {
          protocol_version: PROTOCOL_VERSION,
          service: 0x1,
          timestamp,
          addr_recv,
          //45.154.254.133:8333
          // addr_recv: SocketAddrV4::new(Ipv4Addr::new(45, 154, 254, 133), 8333),
          addr_from: SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080),
          nonce: 0,
          user_agent: "".to_string(),
          start_height: 1,
      }
    }

    fn calculate_timestamp() -> i64 {
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    fn nodenetwork_bitmask(node_net: &i32) -> u64 {
        let mut buffer: u64 = 0x0;
        buffer = buffer.bitand(*node_net as u64);
        buffer
    }

    // supporting only Ipv4 address here ...
    fn netaddr_as_bytes(node_bitmask: &u64, address: &SocketAddrV4) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend_from_slice(&node_bitmask.to_le_bytes());
        let ip_addr_bytes = address.ip().to_ipv6_compatible().octets();

        buffer.extend_from_slice(&ip_addr_bytes);
        buffer.extend_from_slice(&address.port().to_be_bytes());

        buffer
    }

    fn netaddr_from_bytes(buff: &mut Vec<u8>) -> Result<SocketAddrV4, Error> {
        let (_, buff) = parse_frombytes_le::<u64>(buff)?; // node service field
        let (ip_addr, buff) = read_drop_slice(&buff, 16)?;
        let (port_addr, _) = parse_frombytes_be::<u16>(&buff)?;
        let array_ip = <[u8; 4]>::try_from(ip_addr).unwrap();
        Ok(SocketAddrV4::new(Ipv4Addr::from(array_ip), port_addr))
    }
}

impl RawMessage for VersionMessage {
    fn to_bytes(&self) -> Result<Vec<u8>, BitcoinMessageError> {
        let svc_bitmask = Self::nodenetwork_bitmask(&0x1);
        let mut address_bytes = Self::netaddr_as_bytes(&svc_bitmask, &self.addr_recv);
        let mut buffer: Vec<u8> = vec![];
        buffer.extend_from_slice(&self.protocol_version.to_le_bytes());
        buffer.extend_from_slice(&svc_bitmask.to_le_bytes());
        buffer.extend_from_slice(&self.timestamp.to_le_bytes());
        buffer.append(&mut address_bytes);
        buffer.extend_from_slice(&[0x0_u8; 26]); // addr_from
        buffer.extend_from_slice(&self.nonce.to_le_bytes());
        buffer.extend_from_slice(&[self.user_agent.len() as u8]);
        buffer.extend_from_slice(&self.start_height.to_le_bytes());
        buffer.extend_from_slice(&[0]); // relay

        Ok(buffer)
    }

    fn from_bytes(msg: &Vec<u8>) -> Result<Box<Self>, BitcoinMessageError> {
        let (protocol_version, buff) = parse_frombytes_le::<i32>(msg)?;
        let (service, buff) = parse_frombytes_le::<u64>(&buff)?;
        let (timestamp, buff) = parse_frombytes_le::<i64>(&buff)?;

        let address = Self::netaddr_from_bytes(&mut buff.to_vec())?;
        let add_from = Self::netaddr_from_bytes(&mut buff.to_vec())?;
        let (nonce, _) = parse_frombytes_le::<u64>(&buff)?;
        // dropping the remaining fields ...

        Ok(Box::new(VersionMessage {
            protocol_version,
            service,
            timestamp,
            addr_recv: address,
            addr_from: add_from,
            nonce,
            user_agent: "".to_string(), // TODO let user_agent = parser.read_var_string()?;
            start_height: 1,            // TODO let start_height = parser.read_i32_le()?;
        }))
    }
}
