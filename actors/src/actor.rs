use std::{future::Future, pin::Pin, time::Duration};

use async_trait::async_trait;
use tokio::task::JoinHandle;

use crate::{arbiter::spawn_actor, Addr};

pub struct Context<A: Actor> {
    addr: Addr<A>,
}

impl<A: Actor> Context<A> {
    pub(crate) fn new(addr: Addr<A>) -> Self {
        Context { addr }
    }

    pub fn addr(&self) -> &Addr<A> {
        &self.addr
    }

    pub fn run_interval<F>(&self, dur: Duration, f: F) -> JoinHandle<()>
    where
        F: for<'a> Fn(&'a Context<A>) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>
            + Send
            + 'static,
    {
        let addr = self.addr.clone();

        tokio::spawn(async move {
            let ctx = Context { addr };
            let mut interval = tokio::time::interval(dur);

            loop {
                interval.tick().await;
                f(&ctx).await;
            }
        })
    }
}

#[async_trait]
pub trait Actor
where
    Self: Sized + Send + 'static,
{
    async fn started(&mut self, _ctx: &Context<Self>) {}
    async fn stopped(&mut self, _ctx: &Context<Self>) {}

    fn start(self) -> Addr<Self> {
        spawn_actor(self, None)
    }
}
