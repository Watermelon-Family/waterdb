use std::io::Write;

use tracing::debug;
use tracing_subscriber::{layer::SubscriberExt, fmt, util::SubscriberInitExt};
use waterdb::client::Client;

#[tokio::main]
async fn main() -> waterdb::Result<()> {
    tracing_subscriber::registry().with(fmt::layer()).init();

    let addr = format!("{}:{}", waterdb::DEFAULT_IP, waterdb::DEFAULT_PORT);
    let prompt = waterdb::DEFAULT_PROMPT;

    debug!("{:?}", addr);

    let mut client = Client::connect(&addr).await?;

    loop {
        print!("{}> ", prompt);
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        let _ = std::io::stdin().read_line(&mut line)?;
        let line = line.trim().to_string();
        if line == "EXIT" {
            println!("bye");
            break;
        } else {
            let response = client.run(line).await?;
            println!("{:?}", response);
        }
    }

    Ok(())
}