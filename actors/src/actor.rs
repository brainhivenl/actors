use async_trait::async_trait;
use tokio::{select, sync::mpsc};

use crate::Addr;

pub struct Context<A: Actor> {
    addr: Addr<A>,
}

impl<A: Actor> Context<A> {
    pub fn addr(&self) -> &Addr<A> {
        &self.addr
    }
}

#[async_trait]
pub trait Actor
where
    Self: Sized + Send + 'static,
    Self::Message: Send,
{
    type Message;

    async fn started(&mut self, _ctx: &Context<Self>) {}
    async fn stopped(&mut self, _ctx: &Context<Self>) {}
    async fn handle(&mut self, _ctx: &Context<Self>, _msg: Self::Message);

    fn start(mut self) -> Addr<Self> {
        let (tx, mut rx) = mpsc::channel(5);
        let addr = Addr::new(tx);

        let addr2 = addr.clone();

        tokio::spawn(async move {
            let ctx = Context { addr: addr2 };

            self.started(&ctx).await;

            loop {
                select! {
                    _ = ctx.addr().wait() => {
                        self.stopped(&ctx).await;
                        return;
                    }
                    msg = rx.recv() => match msg {
                        Some(msg) => self.handle(&ctx, msg).await,
                        None => {
                            ctx.addr().stop();
                            continue;
                        }
                    }
                }
            }
        });

        addr
    }
}
