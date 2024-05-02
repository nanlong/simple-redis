use std::net::{Ipv4Addr, SocketAddr};

use anyhow::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 6379));
    let listener = TcpListener::bind(addr).await?;
    info!("Listening on: {}", listener.local_addr()?);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            process_redis_conn(&mut socket, addr).await?;

            Ok::<(), anyhow::Error>(())
        });
    }
}

async fn process_redis_conn(stream: &mut TcpStream, addr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => {
                tracing::info!("Connection closed by client: {}", addr);
                break;
            }
            Ok(n) => {
                info!("Received {} bytes from client: {}", n, addr);
                let line = String::from_utf8_lossy(&buf);
                info!("Received: {:?}", line);
                stream.write_all(b"+OK\r\n").await?;

                tracing::info!("Received {} bytes from client: {}", n, addr);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                tracing::error!("Error reading from client: {}", e);
                break;
            }
        }
    }

    Ok(())
}
