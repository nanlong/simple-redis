use anyhow::Result;

use super::parse::Parse;
use super::CommandExecute;
use super::NULL;
use crate::backend::Backend;
use crate::resp::frame::Frame;

#[derive(Debug)]
pub struct Hmget {
    pub(crate) key: String,
    pub(crate) fields: Vec<String>,
}

impl CommandExecute for Hmget {
    fn execute(&self, backend: Backend) -> Result<Frame> {
        let mut result = Vec::with_capacity(self.fields.len());

        for field in &self.fields {
            match backend.hget(&self.key, field) {
                Some(value) => result.push(value),
                None => result.push(NULL.clone()),
            }
        }

        Ok(result.into())
    }
}

impl TryFrom<Frame> for Hmget {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        let mut parse = Parse::try_new(frame)?;
        let command = parse.next_string()?.to_uppercase();

        if command != "HMGET" {
            anyhow::bail!("Invalid command");
        }

        let key = parse.next_string()?;
        let fields_len = parse.length() - 2;
        let mut fields = Vec::with_capacity(fields_len);

        for _ in 0..fields_len {
            let field = parse.next_string()?;
            fields.push(field);
        }

        parse.finish()?;

        Ok(Self { key, fields })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::Backend;
    use crate::resp::RespDecode;
    use std::io::Cursor;

    fn parse_cmd(input: &[u8]) -> Result<Hmget> {
        let mut buf = Cursor::new(input);
        let frame = Frame::decode(&mut buf).unwrap();
        Hmget::try_from(frame)
    }

    #[test]
    fn test_hmget_try_from_frame() {
        let input = b"*5\r\n$5\r\nhmget\r\n$6\r\nmyhash\r\n$6\r\nfield1\r\n$6\r\nfield2\r\n$7\r\nnofield\r\n";
        let cmd = parse_cmd(&input[..]);

        assert!(cmd.is_ok());

        let cmd = cmd.unwrap();
        assert_eq!(cmd.key, "myhash");
        assert_eq!(cmd.fields, vec!["field1", "field2", "nofield"]);
    }

    #[test]
    fn test_hmget_try_from_frame_invalid_command() {
        let input = b"*2\r\n$3\r\nset\r\n$7\r\nmessage\r\n";
        let cmd = parse_cmd(&input[..]);

        assert!(cmd.is_err());
    }

    #[test]
    fn test_hmget_execute() {
        let backend = Backend::new();

        backend.hset("myhash", "field1", b"value1".into());
        backend.hset("myhash", "field2", b"value2".into());

        let input = b"*5\r\n$5\r\nhmget\r\n$6\r\nmyhash\r\n$6\r\nfield1\r\n$6\r\nfield2\r\n$7\r\nnofield\r\n";
        let cmd = parse_cmd(&input[..]).unwrap();

        let result = cmd.execute(backend).unwrap();

        match result {
            Frame::Array(array) => {
                assert_eq!(array.len(), 3);
                assert_eq!(array[0], b"value1".into());
                assert_eq!(array[1], b"value2".into());
                assert_eq!(array[2], *NULL);
            }
            _ => panic!("Expected Array"),
        }
    }
}
