use std::io::Cursor;

use super::{
    array::Array, bignumber::BigNumber, boolean::Boolean, bulk_error::BulkError,
    bulk_string::BulkString, double::Double, integer::Integer, map::Map, null::Null, peek_u8,
    set::Set, simple_error::SimpleError, simple_string::SimpleString, RespDecode, RespEncode,
    RespError,
};

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Frame {
    SimpleString(SimpleString),
    SimpleError(SimpleError),
    Integer(Integer),
    BulkString(BulkString),
    Array(Array),
    Null(Null),
    Boolean(Boolean),
    Double(Double),
    BigNumber(BigNumber),
    BulkError(BulkError),
    Map(Map),
    Set(Set),
}

impl RespDecode for Frame {
    const PREFIX: u8 = 0;

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError> {
        let prefix = peek_u8(buf)?;

        match prefix {
            b'+' => SimpleString::decode(buf).map(Frame::SimpleString),
            b'-' => SimpleError::decode(buf).map(Frame::SimpleError),
            b':' => Integer::decode(buf).map(Frame::Integer),
            b'$' => BulkString::decode(buf).map(Frame::BulkString),
            b'*' => Array::decode(buf).map(Frame::Array),
            b'_' => Null::decode(buf).map(Frame::Null),
            b'#' => Boolean::decode(buf).map(Frame::Boolean),
            b',' => Double::decode(buf).map(Frame::Double),
            b'(' => BigNumber::decode(buf).map(Frame::BigNumber),
            b'!' => BulkError::decode(buf).map(Frame::BulkError),
            b'%' => Map::decode(buf).map(Frame::Map),
            b'~' => Set::decode(buf).map(Frame::Set),
            _ => Err(RespError::InvalidType(format!(
                "Invalid prefix for Frame: {:?}",
                buf.get_ref()
            ))),
        }
    }
}

impl RespEncode for Frame {
    fn encode(&self) -> Vec<u8> {
        match self {
            Frame::SimpleString(inner) => inner.encode(),
            Frame::SimpleError(inner) => inner.encode(),
            Frame::Integer(inner) => inner.encode(),
            Frame::BulkString(inner) => inner.encode(),
            Frame::Array(inner) => inner.encode(),
            Frame::Null(inner) => inner.encode(),
            Frame::Boolean(inner) => inner.encode(),
            Frame::Double(inner) => inner.encode(),
            Frame::BigNumber(inner) => inner.encode(),
            Frame::BulkError(inner) => inner.encode(),
            Frame::Map(inner) => inner.encode(),
            Frame::Set(inner) => inner.encode(),
        }
    }
}

impl From<String> for Frame {
    fn from(s: String) -> Self {
        Frame::SimpleString(SimpleString::new(s))
    }
}

impl From<&str> for Frame {
    fn from(s: &str) -> Self {
        Frame::SimpleString(SimpleString::new(s))
    }
}

impl From<i64> for Frame {
    fn from(i: i64) -> Self {
        Frame::Integer(Integer::new(i))
    }
}

impl<const N: usize> From<&[u8; N]> for Frame {
    fn from(s: &[u8; N]) -> Self {
        Frame::BulkString(BulkString::new(s.to_vec()))
    }
}

impl From<&[u8]> for Frame {
    fn from(s: &[u8]) -> Self {
        Frame::BulkString(BulkString::new(s.to_vec()))
    }
}

impl From<bool> for Frame {
    fn from(b: bool) -> Self {
        Frame::Boolean(Boolean::new(b))
    }
}

impl From<Vec<Frame>> for Frame {
    fn from(f: Vec<Frame>) -> Self {
        Frame::Array(Array::new(f))
    }
}

impl From<f64> for Frame {
    fn from(f: f64) -> Self {
        Frame::Double(Double::new(f))
    }
}

// impl From<Vec<String>> for Frame {
//     fn from(v: Vec<String>) -> Self {
//         Frame::Array(Array::new(v.into_iter().map(|s| s.into()).collect()))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_decode() {
        let mut buf = Cursor::new(&b"+OK\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, "OK".into());

        let mut buf = Cursor::new(&b"-ERR\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, Frame::SimpleError(SimpleError::new("ERR")));

        let mut buf = Cursor::new(&b":1000\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, 1000.into());

        let mut buf = Cursor::new(&b"$6\r\nfoobar\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, b"foobar".into());

        let mut buf = Cursor::new(&b"*2\r\n$3\r\nfoo\r\n$3\r\nbar\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, vec![b"foo".into(), b"bar".into()].into());

        let mut buf = Cursor::new(&b"_\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, Frame::Null(Null));

        let mut buf = Cursor::new(&b"#t\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, true.into());

        let mut buf = Cursor::new(&b",1.234\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, 1.234.into());

        let mut buf = Cursor::new(&b"(1234567890\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(result, Frame::BigNumber(BigNumber::new("1234567890")));

        let mut buf = Cursor::new(&b"!21\r\nSYNTAX invalid syntax\r\n"[..]);
        let result = Frame::decode(&mut buf).unwrap();
        assert_eq!(
            result,
            Frame::BulkError(BulkError::new("SYNTAX invalid syntax"))
        );
    }
}
