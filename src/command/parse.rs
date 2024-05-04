use crate::resp::frame::Frame;
use anyhow::Result;
use std::ops::Deref;
use std::vec::IntoIter;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid type: {0}")]
    InvalidType(String),

    #[error("No more parts")]
    EndOfParts,

    #[error("Not finished")]
    NotFinished,

    #[error("From utf8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, Clone)]
pub struct Parse {
    parts: IntoIter<Frame>,
    length: usize,
}

impl Deref for Parse {
    type Target = IntoIter<Frame>;

    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}

impl Parse {
    pub fn try_new(frame: Frame) -> Result<Self, ParseError> {
        let array = match frame {
            Frame::Array(array) => array,
            _ => return Err(ParseError::InvalidType(format!("for parse {:?}", frame))),
        };

        Ok(Self {
            length: array.inner.len(),
            parts: array.inner.into_iter(),
        })
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn peek_string(&mut self) -> Result<String, ParseError> {
        self.clone().next_string()
    }

    pub fn next(&mut self) -> Result<Frame, ParseError> {
        self.parts.next().ok_or(ParseError::EndOfParts)
    }

    pub fn next_string(&mut self) -> Result<String, ParseError> {
        let frame = self.next()?;
        match frame {
            Frame::SimpleString(s) => Ok(s.inner),
            Frame::BulkString(s) => Ok(String::from_utf8(s.inner)?),
            _ => Err(ParseError::InvalidType(format!("for string {:?}", frame))),
        }
    }

    pub fn finish(&mut self) -> Result<(), ParseError> {
        if self.parts.next().is_none() {
            Ok(())
        } else {
            Err(ParseError::NotFinished)
        }
    }
}

impl TryFrom<Frame> for Parse {
    type Error = anyhow::Error;

    fn try_from(frame: Frame) -> Result<Self> {
        Parse::try_new(frame).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_try_new() {
        let frame: Frame = vec!["get".into(), "key".into()].into();
        let actual = Parse::try_new(frame).unwrap();
        let expected = Parse {
            length: 2,
            parts: vec!["get".into(), "key".into()].into_iter(),
        };

        assert_eq!(actual.length, expected.length);
        assert_eq!(
            actual.parts.collect::<Vec<_>>(),
            expected.parts.collect::<Vec<_>>()
        );
    }
}
