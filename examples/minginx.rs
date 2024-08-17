use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::{io, net::{TcpListener, TcpStream}};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    listen_addr: String,
    upstream_addr: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let config = resolve_config();
    let config = Arc::new(config);
    info!("Upstream: {}", config.upstream_addr);
    info!("Listen: {}", config.listen_addr);

    let listener = TcpListener::bind(&config.listen_addr).await?;
    loop {
        let (client, addr) = listener.accept().await?;
        let cloned_config = Arc::clone(&config);
        info!("Accepted connection from: {}", addr);
        tokio::spawn(async move {
            let upstream = TcpStream::connect(&cloned_config.upstream_addr).await?;
            proxy(client, upstream).await?;
            Ok::<(), anyhow::Error>(())
        });
    }
    #[allow(unreachable_code)]
    Ok(())
}

async fn proxy(mut client: TcpStream, mut upstream: TcpStream) -> Result<()> {
    let (mut client_reader, mut client_writer) = client.split();
    let (mut upstream_reader, mut upstream_writer) = upstream.split();
    let client_to_upstream = io::copy(&mut client_reader, &mut upstream_writer);
    let upstream_to_client = io::copy(&mut upstream_reader, &mut client_writer);
    tokio::try_join!(client_to_upstream, upstream_to_client)?;
    Ok(())
}
    
fn resolve_config() -> Config {
    Config {
        listen_addr: "127.0.0.1:8080".to_string(),
        upstream_addr: "127.0.0.1:8081".to_string(),
    }
}