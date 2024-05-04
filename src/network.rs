use anyhow::Result;
use bytes::{Buf, BytesMut};
use futures::SinkExt;
use std::io::Cursor;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tokio_util::codec::{Decoder, Encoder};
use tracing::info;

use crate::command::Command;
use crate::command::CommandExecute;
use crate::resp::frame::Frame;
use crate::resp::{RespDecode, RespEncode, RespError};
use crate::store::Store;

#[derive(Debug)]
pub struct RespRequest {
    command: Command,
    store: Store,
}

impl RespRequest {
    pub fn new(command: Command, store: Store) -> Self {
        Self { command, store }
    }

    pub fn execute(&self) -> Result<Frame> {
        self.command.execute(self.store.clone())
    }
}

pub async fn stream_handle(stream: TcpStream, store: Store) -> Result<()> {
    let mut framed = Framed::new(stream, RespFrameCodec);

    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                info!("Received frame: {:?}", frame);
                let response = request_handle(frame, store.clone()).await?;
                framed.send(response).await?;
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
}

pub async fn request_handle(frame: Frame, store: Store) -> Result<Frame> {
    let command = Command::try_from(frame)?;
    let request = RespRequest::new(command, store);
    let response = request.execute()?;
    Ok(response)
}

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
