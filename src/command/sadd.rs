use anyhow::Result;

use super::parse::Parse;
use super::CommandExecute;
use crate::backend::Backend;
use crate::resp::frame::Frame;

#[derive(Debug)]
pub struct Sadd {
    pub(crate) key: String,
    pub(crate) field: String,
}

impl CommandExecute for Sadd {
    fn execute(&self, backend: Backend) -> Result<Frame> {
        match backend.sadd(&self.key, &self.field) {
            true => Ok(1.into()),
            false => Ok(0.into()),
        }
    }
}

impl TryFrom<Frame> for Sadd {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "SADD" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        let field = parse.next_string()?;
        parse.finish()?;

        Ok(Self { key, field })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Backend;
    use crate::resp::RespDecode;
    use std::io::Cursor;

    fn parse_cmd(input: &[u8]) -> Result<Sadd> {
        let mut buf = Cursor::new(input);
        let frame = Frame::decode(&mut buf).unwrap();
        Sadd::try_from(frame)
    }

    #[test]
    fn test_sadd_try_from_frame() {
        let input = b"*3\r\n$4\r\nSADD\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let cmd = parse_cmd(input).unwrap();

        assert_eq!(cmd.key, "key");
        assert_eq!(cmd.field, "value");
    }

    #[test]
    fn test_sadd_try_from_frame_invalid_command() {
        let input = b"*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let result = parse_cmd(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_sadd_execute() {
        let input = b"*3\r\n$4\r\nSADD\r\n$3\r\nkey\r\n$5\r\nvalue\r\n";
        let cmd = parse_cmd(input).unwrap();

        let backend = Backend::new();
        let result = cmd.execute(backend);

        assert_eq!(result.unwrap(), 1.into());
    }
}
