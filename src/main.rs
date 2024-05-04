use anyhow::Result;
use simple_redis::{backend::Backend, network::stream_handle};
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let backend = Backend::new();

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 6379));
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);

        let backend = backend.clone();

        tokio::spawn(async move {
            if let Err(e) = stream_handle(stream, backend).await {
                info!("Error: {:?}", e);
            }
        });
    }
}
