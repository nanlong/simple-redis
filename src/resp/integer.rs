use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Integer {
    pub(crate) inner: i64,
}

impl Integer {
    pub fn new(inner: i64) -> Self {
        Self { inner }
    }
}

impl RespDecode for Integer {
    const PREFIX: u8 = b':';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Integer: {:?}",
                buf.get_ref()
            )));
        }

        let line = get_line(buf)?;
        let inner = std::str::from_utf8(line)?.parse()?;
        Ok(Self::new(inner))
    }
}

impl RespEncode for Integer {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend(self.inner.to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_decode() {
        let mut buf = Cursor::new(&b":1000\r\n"[..]);
        let result = Integer::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 1000);
    }

    #[test]
    fn test_integer_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = Integer::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_integer_encode() {
        let integer = Integer::new(1000);
        let result = integer.encode();
        assert_eq!(result, b":1000\r\n");
    }
}
