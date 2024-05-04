use crate::resp::frame::Frame;
use crate::resp::{RespDecode, RespEncode, RespError};
use anyhow::Result;
use bytes::Buf;
use bytes::BytesMut;
use std::io::Cursor;
use tokio_util::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct RespFrameCodec;

impl Decoder for RespFrameCodec {
    type Item = Frame;
    type Error = anyhow::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>> {
        let mut cursor = Cursor::new(&buf[..]);

        match Frame::decode(&mut cursor) {
            Ok(frame) => {
                let len = cursor.position() as usize;
                buf.advance(len);
                Ok(Some(frame))
            }
            Err(RespError::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

impl Encoder<Frame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: Frame, buf: &mut BytesMut) -> Result<()> {
        buf.extend(item.encode());
        Ok(())
    }
}
