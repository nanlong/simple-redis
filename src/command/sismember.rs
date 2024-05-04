use anyhow::Result;

use super::parse::Parse;
use super::CommandExecute;
use crate::backend::Backend;
use crate::resp::frame::Frame;

#[derive(Debug)]
pub struct Sismember {
    key: String,
    field: String,
}

impl CommandExecute for Sismember {
    fn execute(&self, backend: Backend) -> Result<Frame> {
        match backend.sismember(&self.key, &self.field) {
            true => Ok(1.into()),
            false => Ok(0.into()),
        }
    }
}

impl TryFrom<Frame> for Sismember {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "SISMEMBER" {
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

    fn parse_cmd(input: &[u8]) -> Result<Sismember> {
        let mut buf = Cursor::new(input);
        let frame = Frame::decode(&mut buf).unwrap();
        Sismember::try_from(frame)
    }

    #[test]
    fn test_sismember_try_from_frame() {
        let input = b"*3\r\n$9\r\nsismember\r\n$5\r\nmyset\r\n$3\r\none\r\n";
        let cmd = parse_cmd(input).unwrap();

        assert_eq!(cmd.key, "myset");
        assert_eq!(cmd.field, "one");
    }

    #[test]
    fn test_sismember_try_from_frame_invalid_command() {
        let input = b"*3\r\n$3\r\nset\r\n$5\r\nmyset\r\n$3\r\none\r\n";
        let cmd = parse_cmd(input);

        assert!(cmd.is_err());
    }

    #[test]
    fn test_sismember_execute() {
        let input = b"*3\r\n$9\r\nsismember\r\n$5\r\nmyset\r\n$3\r\none\r\n";
        let cmd = parse_cmd(input).unwrap();

        let backend = Backend::new();
        let result = cmd.execute(backend);

        assert!(result.is_ok());
    }
}
