use actors::Actor;

use ping::{Ping, PingMsg};

mod ping;

#[tokio::main]
async fn main() {
    let actor = Ping {};
    let addr = actor.start();

    let response = addr.send(PingMsg {}).await.unwrap();
    println!("{}", response);

    addr.wait().await;
}
