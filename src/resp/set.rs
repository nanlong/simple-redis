use std::collections::BTreeSet;
use std::io::Cursor;

use anyhow::Result;
use bytes::Buf;

use super::Frame;
use super::{get_decimal, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Set {
    inner: BTreeSet<Frame>,
}

impl Set {
    pub fn new(inner: BTreeSet<Frame>) -> Self {
        Self { inner }
    }
}

impl RespDecode for Set {
    const PREFIX: u8 = b'*';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Set: {:?}",
                buf.get_ref()
            )));
        }

        let len = get_decimal(buf)? as usize;
        let mut inner = BTreeSet::new();

        for _ in 0..len {
            if !buf.has_remaining() {
                break;
            }

            let frame = Frame::decode(buf)?;
            inner.insert(frame);
        }

        Ok(Self::new(inner))
    }
}

impl RespEncode for Set {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend(self.inner.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");

        for frame in &self.inner {
            buf.extend(frame.encode());
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_decode() {
        let mut buf = Cursor::new(&b"*2\r\n:1\r\n:2\r\n"[..]);
        let result = Set::decode(&mut buf).unwrap();
        assert_eq!(result.inner.len(), 2);
        assert!(result.inner.contains(&1.into()));
        assert!(result.inner.contains(&2.into()));
    }

    #[test]
    fn test_set_decode_error() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = Set::decode(&mut buf);
        assert!(result.is_err());
    }

    #[test]
    fn test_set_encode() {
        let set = Set::new(vec![1.into(), 2.into()].into_iter().collect());
        assert_eq!(set.encode(), b"*2\r\n:1\r\n:2\r\n");
    }
}
