use std::io::Cursor;

use anyhow::Result;

use super::{get_line, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, PartialEq)]
pub struct Double {
    inner: f64,
}

impl Eq for Double {}

impl PartialOrd for Double {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Double {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.inner.partial_cmp(&other.inner) {
            Some(ordering) => ordering,
            None => std::cmp::Ordering::Less,
        }
    }
}

impl Double {
    pub fn new(inner: f64) -> Self {
        Self { inner }
    }
}

impl RespDecode for Double {
    const PREFIX: u8 = b',';

    // decode with format: ,[<+|->]<integral>[.<fractional>][<E|e>[sign]<exponent>]\r\n
    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Double: {:?}",
                buf.get_ref()
            )));
        }

        let line = get_line(buf)?;
        let inner = std::str::from_utf8(line)?.parse::<f64>()?;

        Ok(Self::new(inner))
    }
}

impl RespEncode for Double {
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
    fn test_double_decode() {
        let mut buf = Cursor::new(&b",12.345\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 12.345);

        let mut buf = Cursor::new(&b",12.345e-2\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 12.345e-2);

        let mut buf = Cursor::new(&b",12.345E-2\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 12.345e-2);

        let mut buf = Cursor::new(&b",12.345e2\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 12.345e2);

        let mut buf = Cursor::new(&b",12.345E2\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 12.345e2);

        let mut buf = Cursor::new(&b",12.345e+2\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 12.345e2);

        let mut buf = Cursor::new(&b",-12.345E+2\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, -12.345e2);

        let mut buf = Cursor::new(&b",+1.23456e-9\r\n"[..]);
        let result = Double::decode(&mut buf).unwrap();
        assert_eq!(result.inner, 1.23456e-9);
    }

    #[test]
    fn test_double_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = Double::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_double_encode() {
        let double = Double::new(12.34555);
        let result = double.encode();
        assert_eq!(result, b",12.34555\r\n");
    }
}
