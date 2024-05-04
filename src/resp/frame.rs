use enum_dispatch::enum_dispatch;
use std::io::Cursor;

use super::{
    array::Array, bignumber::BigNumber, boolean::Boolean, bulk_error::BulkError,
    bulk_string::BulkString, double::Double, integer::Integer, map::Map, null::Null, peek_u8,
    set::Set, simple_error::SimpleError, simple_string::SimpleString, RespDecode, RespError,
};

#[enum_dispatch(RespEncode)]
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
            b'+' => SimpleString::decode(buf).map(Into::into),
            b'-' => SimpleError::decode(buf).map(Into::into),
            b':' => Integer::decode(buf).map(Into::into),
            b'$' => BulkString::decode(buf).map(Into::into),
            b'*' => Array::decode(buf).map(Into::into),
            b'_' => Null::decode(buf).map(Into::into),
            b'#' => Boolean::decode(buf).map(Into::into),
            b',' => Double::decode(buf).map(Into::into),
            b'(' => BigNumber::decode(buf).map(Into::into),
            b'!' => BulkError::decode(buf).map(Into::into),
            b'%' => Map::decode(buf).map(Into::into),
            b'~' => Set::decode(buf).map(Into::into),
            _ => Err(RespError::InvalidType(format!(
                "Invalid prefix for Frame: {:?}",
                buf.get_ref()
            ))),
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
