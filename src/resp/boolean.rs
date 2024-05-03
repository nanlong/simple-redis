use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Boolean {
    inner: bool,
}

impl Boolean {
    pub fn new(inner: bool) -> Self {
        Self { inner }
    }
}

impl RespDecode for Boolean {
    const PREFIX: u8 = b'#';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Boolean: {:?}",
                buf.get_ref()
            )));
        }

        let line = get_line(buf)?;
        let inner = match std::str::from_utf8(line)? {
            "t" => true,
            "f" => false,
            _ => {
                return Err(RespError::InvalidType(format!(
                    "Invalid value for Boolean: {:?}",
                    buf.get_ref()
                )))
            }
        };

        Ok(Self::new(inner))
    }
}

impl RespEncode for Boolean {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend(if self.inner { b"t" } else { b"f" });
        buf.extend_from_slice(b"\r\n");
        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_decode() {
        let mut buf = Cursor::new(&b"#t\r\n"[..]);
        let result = Boolean::decode(&mut buf).unwrap();
        assert!(result.inner);

        let mut buf = Cursor::new(&b"#f\r\n"[..]);
        let result = Boolean::decode(&mut buf).unwrap();
        assert!(!result.inner);
    }

    #[test]
    fn test_boolean_decode_error() {
        let mut buf = Cursor::new(&b"#x\r\n"[..]);
        let result = Boolean::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_encode() {
        let value = Boolean::new(true);
        let result = value.encode();
        assert_eq!(result, b"#t\r\n");

        let value = Boolean::new(false);
        let result = value.encode();
        assert_eq!(result, b"#f\r\n");
    }
}
