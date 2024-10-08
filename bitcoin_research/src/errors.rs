use thiserror::Error;

#[derive(Error, Debug)]
pub enum BitcoinMessageError {
    #[error("command name too long")]
    CommandNameTooLong,

    #[error("command name has to be ASCII string")]
    CommandNameNonAscii,

    #[error("IO Error during (de)serialization: {0}")]
    SerializationError(#[from] std::io::Error),

    #[error("payload is larger than MAX_SIZE")]
    PayloadTooBig,

    #[error("FromUtf8Error during deserialization: {0}")]
    Utf8DeserializationError(#[from] std::string::FromUtf8Error),

    #[error("unknown command name: {0}")]
    CommandNameUnknown(String),

    #[error("checksum mismatch")]
    ChecksumMismatch,
}
