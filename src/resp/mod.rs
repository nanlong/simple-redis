mod array;
mod bignumber;
mod boolean;
mod bulk_error;
mod bulk_string;
mod double;
mod frame;
mod integer;
mod map;
mod null;
mod set;
mod simple_error;
mod simple_string;

use std::io::Cursor;

use anyhow::Result;
use bytes::Buf;
use thiserror::Error;

use frame::Frame;

#[derive(Debug, Error)]
pub enum RespError {
    #[error("Incomplete")]
    Incomplete,

    #[error("Invalid type: {0}")]
    InvalidType(String),

    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("FromUtf8 error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("ParseInt error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("ParseFloat error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}

pub trait RespDecode: Sized {
    const PREFIX: u8;

    fn decode(buf: &mut Cursor<&[u8]>) -> Result<Self, RespError>;
}

pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

fn get_u8(buf: &mut Cursor<&[u8]>) -> Result<u8, RespError> {
    if !buf.has_remaining() {
        return Err(RespError::Incomplete);
    }

    Ok(buf.get_u8())
}

fn peek_u8(buf: &mut Cursor<&[u8]>) -> Result<u8, RespError> {
    if !buf.has_remaining() {
        return Err(RespError::Incomplete);
    }

    Ok(buf.chunk()[0])
}

fn get_decimal(buf: &mut Cursor<&[u8]>) -> Result<u64, RespError> {
    let line = get_line(buf)?;
    let inner = std::str::from_utf8(line)?.parse()?;
    Ok(inner)
}

fn get_line<'a>(buf: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], RespError> {
    let start = buf.position() as usize;
    let end = buf.get_ref().len() - 1;

    for i in start..end {
        if buf.get_ref()[i] == b'\r' && buf.get_ref()[i + 1] == b'\n' {
            buf.set_position((i + 2) as u64);
            return Ok(&buf.get_ref()[start..i]);
        }
    }

    Err(RespError::Incomplete)
}
