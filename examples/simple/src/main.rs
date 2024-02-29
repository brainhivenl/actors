use actors::Actor;

use ping::{Ping, PingMsg};

mod ping;

#[tokio::main]
async fn main() {
    let actor = Ping {};
    let addr1 = actor.start();

    let response = addr1.send(PingMsg {}).await.unwrap();
    println!("{}", response);

    addr1.wait().await;
}
