use std::{collections::BTreeMap, io::Cursor};

use anyhow::Result;
use bytes::Buf;

use super::{get_decimal, get_u8, Frame, RespDecode, RespEncode, RespError};

#[derive(Debug, Hash, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Map {
    inner: BTreeMap<Frame, Frame>,
}

impl Map {
    pub fn new(inner: BTreeMap<Frame, Frame>) -> Self {
        Self { inner }
    }
}

impl RespDecode for Map {
    const PREFIX: u8 = b'%';

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        if get_u8(buf)? != Self::PREFIX {
            return Err(RespError::InvalidType(format!(
                "Invalid prefix for Map: {:?}",
                buf.get_ref()
            )));
        }

        let len = get_decimal(buf)? as usize;
        let mut inner = BTreeMap::new();

        for _ in 0..len {
            if !buf.has_remaining() {
                break;
            }

            let key = Frame::decode(buf)?;
            let value = Frame::decode(buf)?;
            inner.insert(key, value);
        }

        Ok(Self::new(inner))
    }
}

impl RespEncode for Map {
    fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(Self::PREFIX);
        buf.extend(self.inner.len().to_string().as_bytes());
        buf.extend_from_slice(b"\r\n");

        for (key, value) in &self.inner {
            buf.extend(key.encode());
            buf.extend(value.encode());
        }

        buf
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_decode() {
        let mut buf = Cursor::new(&b"%2\r\n+first\r\n:1\r\n+second\r\n:2\r\n"[..]);
        let result = Map::decode(&mut buf).unwrap();
        assert_eq!(result.inner.len(), 2);

        match result.inner.get(&"first".into()).unwrap() {
            Frame::Integer(integer) => assert_eq!(integer.inner, 1),
            _ => panic!("Expected Integer"),
        }

        match result.inner.get(&"second".into()).unwrap() {
            Frame::Integer(integer) => assert_eq!(integer.inner, 2),
            _ => panic!("Expected Integer"),
        }
    }

    #[test]
    fn test_map_encode() {
        let mut inner = BTreeMap::new();
        inner.insert("first".into(), 1.into());
        inner.insert("second".into(), 2.into());

        let map = Map::new(inner);
        let result = map.encode();
        assert_eq!(result, b"%2\r\n+first\r\n:1\r\n+second\r\n:2\r\n");
    }
}
