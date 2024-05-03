use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleError {
    inner: String,
}

impl SimpleError {
    pub fn new(inner: impl ToString) -> Self {
        Self {
            inner: inner.to_string(),
        }
    }
}

impl RespDecode for SimpleError {
    const PREFIX: u8 = b'-';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for SimpleError: {:?}",
                buf.get_ref()
            )));
        }

        let line = get_line(buf)?.to_vec();
        let inner = String::from_utf8(line)?;
        Ok(Self::new(inner))
    }
}

impl RespEncode for SimpleError {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend(self.inner.as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_error_decode() {
        let mut buf = Cursor::new(&b"-ERR\r\n"[..]);
        let result = SimpleError::decode(&mut buf).unwrap();
        assert_eq!(result.inner, "ERR");
    }

    #[test]
    fn test_simple_error_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = SimpleError::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_error_encode() {
        let error = SimpleError::new("ERR");
        let result = error.encode();
        assert_eq!(result, b"-ERR\r\n");
    }
}
