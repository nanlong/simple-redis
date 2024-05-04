use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Null;

impl RespDecode for Null {
    const PREFIX: u8 = b'_';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Null: {:?}",
                buf.get_ref()
            )));
        }

        get_line(buf)?;

        Ok(Null)
    }
}

impl RespEncode for Null {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null_decode() {
        let mut buf = Cursor::new(&b"_\r\n"[..]);
        let result = Null::decode(&mut buf).unwrap();
        assert_eq!(result, Null);
    }

    #[test]
    fn test_null_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = Null::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_null_encode() {
        let null = Null;
        let result = null.encode();
        assert_eq!(result, b"_\r\n");
    }
}
