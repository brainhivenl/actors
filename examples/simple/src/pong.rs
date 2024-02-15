use actors::{async_trait, Actor};

pub struct Pong;

pub enum Message {
    Pong,
}

#[async_trait]
impl Actor for Pong {
    type Message = Message;

    async fn started(&mut self, _ctx: &actors::Context<Self>) {
        println!("PONG STARTED!");
    }

    async fn handle(&mut self, _ctx: &actors::Context<Self>, msg: Self::Message) {
        match msg {
            Message::Pong => {
                println!("PONG!");
            }
        }
    }
}
