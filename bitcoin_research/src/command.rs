use std::fmt::Display;

use crate::errors::BitcoinMessageError;

// const COMMAND_NAME_SIZE: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enum corresponding to the `command_name` from Message header.
pub enum Command {
    /// `version` command_name
    Version,

    /// `verack` command_name
    VerAck,
}

impl Command {
    /// Converts [`Command`] into byte vector.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Command::Version => "version",
            Command::VerAck => "verack",
        };

        write!(f, "{}", s)
    }
}

impl TryFrom<&str> for Command {
    type Error = BitcoinMessageError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "version" => Ok(Command::Version),
            "verack" => Ok(Command::VerAck),
            x => Err(BitcoinMessageError::CommandNameUnknown(x.to_string())),
        }
    }
}

impl From<Command> for String {
    fn from(c: Command) -> Self {
        c.to_string()
    }
}

impl From<Command> for Vec<u8> {
    fn from(c: Command) -> Self {
        c.to_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_as_string() {
        assert_eq!(Command::Version.to_string(), "version");
        assert_eq!(Command::VerAck.to_string(), "verack");
    }

    #[test]
    fn string_as_command() {
        assert_eq!(Command::try_from("version").unwrap(), Command::Version);
        assert_eq!(Command::try_from("verack").unwrap(), Command::VerAck);
    }

    #[test]
    fn command_as_bytes() {
        assert_eq!(Command::Version.to_bytes(), b"version");
        assert_eq!(Command::VerAck.to_bytes(), b"verack");
    }
}
