use actors::Actor;

use ping::{Message, Ping};

mod ping;
mod pong;

#[tokio::main]
async fn main() {
    let actor = Ping {};
    let addr = actor.start();

    addr.do_send(Message::Ping);

    addr.wait().await;
}
