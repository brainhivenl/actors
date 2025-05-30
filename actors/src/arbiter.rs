use tokio::{select, sync::mpsc};
use tokio_util::sync::CancellationToken;

use crate::{Actor, Addr, Context};

#[derive(Default, Clone)]
pub struct Arbiter {
    token: CancellationToken,
}

impl Arbiter {
    pub fn spawn<A: Actor>(&mut self, actor: A) -> Addr<A> {
        spawn_actor(actor, Some(self.token.child_token()))
    }

    pub async fn wait(&self) {
        self.token.cancelled().await;
    }

    pub fn stop(&self) {
        self.token.cancel();
    }
}

pub(crate) fn spawn_actor<A: Actor>(mut actor: A, token: Option<CancellationToken>) -> Addr<A> {
    let (tx, mut rx) = mpsc::channel(5);
    let addr = Addr::new(tx, token.unwrap_or_default());
    let addr2 = addr.clone();

    tokio::spawn(async move {
        let ctx = Context::new(addr);

        tracing::debug!({ actor = A::NAME }, "actor started");
        actor.started(&ctx).await;

        'actor: loop {
            select! {
                _ = ctx.addr().wait() => {
                    tracing::debug!({ actor = A::NAME }, "actor stopped");
                    actor.stopped(&ctx).await;
                    break 'actor;
                }
                msg = rx.recv() => match msg {
                    Some(mut proxy) => proxy.handle(&mut actor, &ctx).await,
                    None => {
                        ctx.addr().stop();
                        break 'actor;
                    }
                }
            }
        }

        tracing::warn!({ actor = A::NAME }, "actor exited");
    });

    addr2
}
