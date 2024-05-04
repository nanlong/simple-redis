use anyhow::Result;

use super::parse::Parse;
use super::CommandExecute;
use crate::resp::frame::Frame;
use crate::store::Store;

#[derive(Debug)]
pub struct Smembers {
    pub(crate) key: String,
}

impl CommandExecute for Smembers {
    fn execute(&self, store: Store) -> Result<Frame> {
        let result = store.smembers(&self.key);

        match result {
            Some(set) => Ok(set
                .iter()
                .map(|s| s.as_bytes().into())
                .collect::<Vec<Frame>>()
                .into()),
            None => Ok(Vec::<Frame>::new().into()),
        }
    }
}

impl TryFrom<Frame> for Smembers {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "SMEMBERS" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        parse.finish()?;

        Ok(Self { key })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespDecode;
    use crate::store::Store;
    use std::io::Cursor;

    fn parse_cmd(input: &[u8]) -> Result<Smembers> {
        let mut buf = Cursor::new(input);
        let frame = Frame::decode(&mut buf).unwrap();
        Smembers::try_from(frame)
    }

    #[test]
    fn test_smembers_try_from_frame() {
        let input = b"*2\r\n$8\r\nSMEMBERS\r\n$3\r\nkey\r\n";
        let cmd = parse_cmd(input).unwrap();

        assert_eq!(cmd.key, "key");
    }

    #[test]
    fn test_smembers_try_from_frame_invalid_command() {
        let input = b"*2\r\n$3\r\nSET\r\n$3\r\nkey\r\n";
        let mut buf = Cursor::new(&input[..]);
        let frame = Frame::decode(&mut buf).unwrap();
        let result = Smembers::try_from(frame);

        assert!(result.is_err());
    }

    #[test]
    fn test_smembers_execute() {
        let input = b"*2\r\n$8\r\nSMEMBERS\r\n$3\r\nkey\r\n";
        let cmd = parse_cmd(input).unwrap();

        let store = Store::new();
        let result = cmd.execute(store);

        assert_eq!(result.unwrap(), Vec::<Frame>::new().into());
    }
}
