use std::{future::Future, sync::Arc, time::Duration};

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

    pub fn run_background<F, Fut>(&self, f: F) -> JoinHandle<()>
    where
        Fut: Future<Output = ()> + Send + 'static,
        F: FnOnce(Context<A>) -> Fut + Send + Sync + 'static,
    {
        let addr = self.addr.clone();
        tokio::spawn(async move {
            let token = addr.token.clone();

            tokio::select! {
                _ = token.cancelled() => {
                    return;
                }
                _ = f(Context { addr }) => {}
            }
        })
    }

    pub fn run_interval<F, Fut>(&self, dur: Duration, f: F) -> JoinHandle<()>
    where
        Fut: Future<Output = ()> + Send + 'static,
        F: Fn(Arc<Context<A>>) -> Fut + Send + Sync + 'static,
    {
        self.run_background(move |ctx| async move {
            let mut interval = tokio::time::interval(dur);
            let token = ctx.addr().token.clone();
            let rc = Arc::new(ctx);

            loop {
                tokio::select! {
                    _ = token.cancelled() => break,
                    _ = interval.tick() => f(Arc::clone(&rc)).await,
                    _ = rc.addr().wait() => break,
                }
            }
        })
    }
}

#[async_trait]
pub trait Actor: Named
where
    Self: Sized + Send + 'static,
{
    async fn started(&mut self, _ctx: &Context<Self>) {}
    async fn stopped(&mut self, _ctx: &Context<Self>) {}

    fn start(self) -> Addr<Self> {
        spawn_actor(self, None)
    }
}

pub trait Named {
    const NAME: &'static str;
}
