use actors::{async_trait, Actor};

use crate::pong::{self, Pong};

pub struct Ping;

pub enum Message {
    Ping,
}

#[async_trait]
impl Actor for Ping {
    type Message = Message;

    async fn started(&mut self, _ctx: &actors::Context<Self>) {
        println!("PING STARTED!");
    }

    async fn handle(&mut self, _ctx: &actors::Context<Self>, msg: Self::Message) {
        match msg {
            Message::Ping => {
                println!("PING");

                let addr = Pong {}.start();
                addr.do_send(pong::Message::Pong);
            }
        }
    }
}
