use anyhow::Result;
use simple_redis::network;
use simple_redis::Backend;
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let addr = "0.0.0.0:6379";
    info!("Simple-Redist is listening on {addr}");

    let listener = TcpListener::bind(addr).await?;
    let backend = Backend::new();
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from :{raddr}");
        let cloned_backed = backend.clone();

        tokio::spawn(async move {
            match network::stream_handler(stream, cloned_backed).await {
                Ok(_) => {
                    info!("Connection from {raddr} exited");
                }
                Err(e) => {
                    warn!("handle error for {raddr}: {e:?}");
                }
            }
        });
    }
}
