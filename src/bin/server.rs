use tokio::{net::TcpListener, signal};
use tracing_subscriber::{layer::SubscriberExt, fmt, util::SubscriberInitExt};
use waterdb::server;

#[tokio::main]
pub async fn main() -> waterdb::Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let addr = format!("{}:{}", waterdb::DEFAULT_IP, waterdb::DEFAULT_PORT);
    
    // Bind a TCP listener
    let listener = TcpListener::bind(&addr).await?;

    server::run(listener, signal::ctrl_c()).await;

    Ok(())
}