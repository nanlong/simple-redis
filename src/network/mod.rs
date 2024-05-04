mod codec;
mod request;

use crate::command::Command;
use crate::resp::frame::Frame;
use crate::store::Store;
use anyhow::Result;
use codec::RespFrameCodec;
use futures::SinkExt;
use request::RespRequest;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;
use tracing::info;

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
