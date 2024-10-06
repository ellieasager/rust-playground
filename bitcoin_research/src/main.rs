use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4};

use bitcoin_research::btc_message::BtcMessage;
use bitcoin_research::command::Command;
use bitcoin_research::payload::{Payload, VersionMessage};
use bitcoin_research::raw_message::RawMessage;
use bitcoin_research::utils::{checksum, VERSION_COMMAND, VER_ACK_COMMAND};

fn handshake() {
    // let address = SocketAddrV4::new(Ipv4Addr::new(127, 0,0, 1), 18445);
    // 162.120.69.182
    let address = SocketAddrV4::new(Ipv4Addr::new(162, 120,69, 182), 8333);
    let mut stream = std::net::TcpStream::connect(address).unwrap();

    let version_message = VersionMessage::new(address);
    let version_message_hash = checksum(version_message.to_bytes().unwrap());
    // let vers_check : [u8; 4] = version_message_hash[..4].try_into().unwrap();
    let checksum = u32::from_ne_bytes(version_message_hash);
    let payload = Payload::Version(version_message);
    let btc_message = BtcMessage::new(VERSION_COMMAND, payload, checksum);
    println!("SENDING: {:?}", &btc_message.to_bytes().unwrap());

    // Send version message
    stream.write_all(&btc_message.to_bytes().unwrap()).unwrap();
    stream.flush().unwrap();

    let mut rec_buff = [0; 24];
    // Receive version payload.
    println!("Waiting for version answer...");
    let _ = stream.read_exact(&mut rec_buff); // return the size of the buffer (don't need it now)

    
    let v_answer = BtcMessage::from_bytes(&mut rec_buff).unwrap();
    println!("RECEIVED: {:?}", v_answer);
    if v_answer.command != VERSION_COMMAND {
        println!("Command: {:?}", v_answer.command);
        println!("Expected: {:?}", Command::Version);
        println!("ERROR: Wrong command");
    } else {
        println!(
            "connection established. Msg received {:?}",
            v_answer.command
        );
    }

    stream.flush().unwrap();

    let mut rec_buff = [0; 24];
    // let mut rec_buff = Vec::new();
    // Receive version payload.
    println!("Waiting for verack answer...");
    let _ = stream.read_exact(&mut rec_buff); // return the size of the buffer (don't need it now)
    // println!("RECEIVED: {:?} bytes", how_many);
    let v_answer = BtcMessage::from_bytes(&mut rec_buff).unwrap();
    println!("RECEIVED: {:?}", v_answer);
    if v_answer.command != VER_ACK_COMMAND {
        println!("Command: {:?}", v_answer.command);
        println!("Expected: {:?}", Command::VerAck.to_bytes());
        println!("ERROR: Wrong command");
    } else {
        println!(
            "connection established. Msg received {:?}",
            v_answer.command
        );
    }
}

// const BITCOIN_PROTOCOL_VERSION: i32 = 70016; // matches bitcoin core v24
fn main() {
    handshake();
}
