use std::io::Cursor;
use std::ops::Deref;

use anyhow::Result;
use bytes::Buf;

use super::Frame;
use super::{get_decimal, get_u8, RespDecode, RespEncode, RespError};

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Array {
    pub(crate) inner: Vec<Frame>,
}

impl Array {
    pub fn new(inner: Vec<Frame>) -> Self {
        Self { inner }
    }
}

impl RespDecode for Array {
    const PREFIX: u8 = b'*';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Array: {:?}",
                buf.get_ref()
            )));
        }

        let len = get_decimal(buf)? as usize;
        let mut inner = Vec::with_capacity(len);

        for _ in 0..len {
            if !buf.has_remaining() {
                break;
            }

            let frame = Frame::decode(buf)?;
            inner.push(frame);
        }

        Ok(Self::new(inner))
    }
}

impl RespEncode for Array {
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

impl Deref for Array {
    type Target = Vec<Frame>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_decode() {
        let mut buf = Cursor::new(&b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"[..]);
        let frame = Array::decode(&mut buf).unwrap();
        assert_eq!(frame, Array::new(vec![b"foo".into(), b"bar".into(),]));
    }

    #[test]
    fn test_array_encode() {
        let frame = Array::new(vec![b"foo".into(), b"bar".into()]);
        assert_eq!(frame.encode(), b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n");
    }
}
