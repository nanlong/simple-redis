use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BigNumber {
    inner: String,
}

impl BigNumber {
    pub fn new(inner: impl Into<String>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

impl RespDecode for BigNumber {
    const PREFIX: u8 = b'(';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for BigNumber: {:?}",
                buf.get_ref()
            )));
        }

        let line = get_line(buf)?.to_vec();
        let inner = String::from_utf8(line)
            .map_err(|_| RespError::InvalidType("Invalid BigNumber".to_string()))?;

        Ok(Self::new(inner))
    }
}

impl RespEncode for BigNumber {
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
    fn test_big_number_decode() {
        let mut buf = Cursor::new(&b"(1234567890\r\n"[..]);
        let result = BigNumber::decode(&mut buf).unwrap();
        assert_eq!(result.inner, "1234567890");

        let mut buf = Cursor::new(&b"(+1234567890\r\n"[..]);
        let result = BigNumber::decode(&mut buf).unwrap();
        assert_eq!(result.inner, "+1234567890");

        let mut buf = Cursor::new(&b"(-1234567890\r\n"[..]);
        let result = BigNumber::decode(&mut buf).unwrap();
        assert_eq!(result.inner, "-1234567890");
    }

    #[test]
    fn test_big_number_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = BigNumber::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_big_number_encode() {
        let big_number = BigNumber::new("1234567890");
        let result = big_number.encode();
        assert_eq!(result, b"(1234567890\r\n");
    }
}
