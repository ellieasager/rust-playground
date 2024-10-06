use sha2::{Digest, Sha256};

pub const CHECKSUM_SIZE: usize = 4;
pub const VERSION_COMMAND: [u8; 12] = *b"version\0\0\0\0\0";
pub const VER_ACK_COMMAND: [u8; 12] = *b"verack\0\0\0\0\0\0";

/// Computes Bitcoin checksum for given data
pub fn checksum(data: Vec<u8>) -> [u8; 4] {
    let h1 = Sha256::new().chain_update(data).finalize();
    let h2 = Sha256::new().chain_update(h1).finalize();

    let mut buf = [0u8; CHECKSUM_SIZE];
    buf.clone_from_slice(&h2[..CHECKSUM_SIZE]);
    buf
}

// Generic parser using the FromBytes trait from the num (num_traits) crate
// could be a trait and maybe better to put a constrait on the type
pub fn parse_frombytes_be<T>(buff: &Vec<u8>) -> Result<(T, Vec<u8>), std::io::Error>
where
    T: FromEndian + Sized,
{
    let size = core::mem::size_of::<T>();
    match read_drop_slice(buff, size) {
        Ok((res, remaining)) => Ok((FromEndian::from_be(res), remaining)),
        Err(e) => Err(e),
    }
}

pub fn parse_frombytes_le<T>(buff: &Vec<u8>) -> Result<(T, Vec<u8>), std::io::Error>
where
    T: FromEndian + Sized,
{
    let size = core::mem::size_of::<T>();
    match read_drop_slice(buff, size) {
        Ok((res, remaining)) => Ok((FromEndian::from_le(res), remaining)),
        Err(e) => Err(e),
    }
}
// if correct, return the parsed value and the new vector without it , similar to Parsec
// pub fn read_drop_slice<'a>(buff: &Vec<u8>, size: usize) -> Result<(&'a [u8], Vec<u8>), std::io::Error> {
pub fn read_drop_slice(buff: &Vec<u8>, size: usize) -> Result<(&[u8], Vec<u8>), std::io::Error> {
    if buff.len() >= size {
        Ok((&buff[0..size], buff[size..].to_vec()))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "Buffer too small",
        ))
    }
}

// An attempt of a generic BigEndian/LittleEndian parser for numeric types
pub trait FromEndian {
    fn from_be(msg: &[u8]) -> Self
    where
        Self: Sized;
    fn from_le(msg: &[u8]) -> Self
    where
        Self: Sized;
}

impl FromEndian for i32 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        i32::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        i32::from_le_bytes(bytes)
    }
}

impl FromEndian for i64 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        i64::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        i64::from_le_bytes(bytes)
    }
}

impl FromEndian for u16 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(msg);
        u16::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(msg);
        u16::from_le_bytes(bytes)
    }
}

impl FromEndian for u32 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        u32::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        u32::from_le_bytes(bytes)
    }
}

impl FromEndian for u64 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        u64::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        u64::from_le_bytes(bytes)
    }
}
