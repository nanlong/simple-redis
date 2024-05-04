use std::io::Cursor;

use anyhow::Result;

use super::{get_int, get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BulkString {
    pub(crate) inner: Vec<u8>,
}

impl BulkString {
    pub fn new(inner: impl Into<Vec<u8>>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

impl RespDecode for BulkString {
    const PREFIX: u8 = b'$';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for BulkString: {:?}",
                buf.get_ref()
            )));
        }

        let len = get_int(buf)?;

        let inner = if len <= 0 {
            vec![]
        } else {
            let len = len as usize;

            let inner = get_line(buf)?.to_vec();

            if inner.len() != len {
                return Err(RespError::InvalidType(format!(
                    "Invalid length for BulkString: {:?}",
                    buf.get_ref()
                )));
            }

            inner
        };

        Ok(Self::new(inner))
    }
}

impl RespEncode for BulkString {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);

        let len = self.inner.len();

        if len == 0 {
            buf.extend_from_slice(b"-1");
        } else {
            buf.extend(len.to_string().as_bytes());
            buf.extend_from_slice(b"\r\n");
            buf.extend(&self.inner);
        }

        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bulk_string_decode() {
        let mut buf = Cursor::new(&b"$5\r\nhello\r\n"[..]);
        let result = BulkString::decode(&mut buf).unwrap();
        assert_eq!(result.inner, b"hello");
    }

    #[test]
    fn test_bulk_string_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = BulkString::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_bulk_string_encode() {
        let bulk_string = BulkString::new("hello");
        let result = bulk_string.encode();
        assert_eq!(result, b"$5\r\nhello\r\n");
    }

    #[test]
    fn test_null_bulk_string_decode() {
        let mut buf = Cursor::new(&b"$-1\r\n"[..]);
        let result = BulkString::decode(&mut buf).unwrap();
        assert_eq!(result.inner, b"");
    }

    #[test]
    fn test_null_bulk_string_encode() {
        let bulk_string = BulkString::new("");
        let result = bulk_string.encode();
        assert_eq!(result, b"$-1\r\n");
    }
}
