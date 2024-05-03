use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleString {
    pub(crate) inner: String,
}

impl SimpleString {
    pub fn new(inner: impl ToString) -> Self {
        Self {
            inner: inner.to_string(),
        }
    }
}

impl RespDecode for SimpleString {
    const PREFIX: u8 = b'+';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for SimpleString: {:?}",
                buf.get_ref()
            )));
        }

        let line = get_line(buf)?.to_vec();
        let inner = String::from_utf8(line)?;
        Ok(Self::new(inner))
    }
}

impl RespEncode for SimpleString {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend(self.inner.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

impl From<&str> for SimpleString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_string_decode() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = SimpleString::decode(&mut buf).unwrap();
        assert_eq!(result.inner, "OK");
    }

    #[test]
    fn test_simple_string_decode_error() {
        let mut buf = Cursor::new(&b"-ERR\r\n"[..]);
        let result = SimpleString::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_string_encode() {
        let simple_string = SimpleString::new("OK");
        let result = simple_string.encode();
        assert_eq!(result, b"+OK\r\n");
    }

    #[test]
    fn test_simple_string_encode_empty() {
        let simple_string = SimpleString::new("");
        let result = simple_string.encode();
        assert_eq!(result, b"+\r\n");
    }
}
