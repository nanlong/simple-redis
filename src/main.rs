use anyhow::Result;
use simple_redis::{network::stream_handle, store::Store};
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let store = Store::new();

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 6379));
    info!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);

        let store = store.clone();

        tokio::spawn(async move {
            if let Err(e) = stream_handle(stream, store).await {
                info!("Error: {:?}", e);
            }
        });
    }
}
