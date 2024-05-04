use anyhow::Result;

use super::parse::Parse;
use super::CommandExecute;
use crate::backend::Backend;
use crate::resp::frame::Frame;

#[derive(Debug)]
pub struct Echo {
    pub(crate) message: Frame,
}

impl CommandExecute for Echo {
    fn execute(&self, _backend: Backend) -> Result<Frame> {
        Ok(self.message.clone())
    }
}

impl TryFrom<Frame> for Echo {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "ECHO" {
            anyhow::bail!("Invalid command");
        }

        let message = parse.next()?;
        parse.finish()?;

        Ok(Self { message })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resp::RespDecode;
    use std::io::Cursor;

    #[test]
    fn test_echo_try_from_frame() {
        let input = b"*2\r\n$4\r\necho\r\n$7\r\nmessage\r\n";
        let mut buf = Cursor::new(&input[..]);
        let frame = Frame::decode(&mut buf).unwrap();
        let cmd = Echo::try_from(frame).unwrap();

        assert_eq!(cmd.message, b"message".into());
    }

    #[test]
    fn test_echo_try_from_frame_invalid_command() {
        let input = b"*2\r\n$3\r\nset\r\n$7\r\nmessage\r\n";
        let mut buf = Cursor::new(&input[..]);
        let frame = Frame::decode(&mut buf).unwrap();
        let result = Echo::try_from(frame);

        assert!(result.is_err());
    }

    #[test]
    fn test_echo_execute() {
        let cmd = Echo {
            message: b"message".into(),
        };

        let backend = Backend::new();
        let result = cmd.execute(backend);

        assert_eq!(result.unwrap(), b"message".into());
    }
}
