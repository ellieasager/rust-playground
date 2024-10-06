use crate::errors::BitcoinMessageError;

/// Trait defining a data structure that can be serialized to bitcoin protocol "wire" data without any outside input.
pub trait RawMessage {
    /// Performs the serialization.
    fn to_bytes(&self) -> Result<Vec<u8>, BitcoinMessageError>;

    /// Constructs `Self` from binary data.
    fn from_bytes(data: &Vec<u8>) -> Result<Box<Self>, BitcoinMessageError>
    where
        Self: std::marker::Sized;
}
