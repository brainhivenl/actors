use std::time::Duration;

use actors::{async_trait, Actor, Handler};
use actors_macros::{Message, Named};

#[derive(Named)]
pub struct Ping;

#[derive(Message)]
#[result_type(&'static str)]
pub struct PingMsg;

#[async_trait]
impl Actor for Ping {
    async fn started(&mut self, ctx: &actors::Context<Self>) {
        println!("PING STARTED!");

        ctx.run_interval(Duration::from_secs(1), |ctx| async move {
            ctx.addr().do_send(PingMsg);
        });
    }
}

#[async_trait]
impl Handler<PingMsg> for Ping {
    async fn handle(&mut self, _ctx: &actors::Context<Self>, _msg: PingMsg) -> &'static str {
        println!("PING!");
        "PONG"
    }
}
