use actors::{async_trait, Actor, Handler, Message};

pub struct Ping;

pub struct PingMsg;

impl Message for PingMsg {
    type Result = i32;
}

#[async_trait]
impl Actor for Ping {
    async fn started(&mut self, _ctx: &actors::Context<Self>) {
        println!("PING STARTED!");
    }
}

#[async_trait]
impl Handler<PingMsg> for Ping {
    async fn handle(&mut self, _ctx: &actors::Context<Self>, _msg: PingMsg) -> i32 {
        100
    }
}
