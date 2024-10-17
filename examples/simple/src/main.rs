use actors::Actor;

use ping::Ping;
use tracing_subscriber::EnvFilter;

mod ping;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let actor = Ping {};
    let addr1 = actor.start();

    println!("waiting 5 seconds to stop");
    let addr = addr1.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        addr.stop();
    });

    addr1.wait().await;

    println!("ping should be stopped");
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
