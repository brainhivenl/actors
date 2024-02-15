use async_trait::async_trait;
use tokio::sync::oneshot::Sender;

use crate::{Actor, Context};

#[async_trait]
pub trait Handler<M>
where
    Self: Actor,
    M: Message + Send,
{
    async fn handle(&mut self, ctx: &Context<Self>, msg: M) -> M::Result;
}

pub trait Message {
    type Result;
}

#[async_trait]
pub trait Proxy<A: Actor>: Send {
    async fn handle(&mut self, actor: &mut A, ctx: &Context<A>);
}

pub struct MessageProxy<M: Message> {
    message: Option<M>,
    response: Option<Sender<M::Result>>,
}

impl<M: Message> MessageProxy<M> {
    pub fn new(message: M, response: Option<Sender<M::Result>>) -> Self {
        MessageProxy {
            message: Some(message),
            response,
        }
    }
}

#[async_trait]
impl<A, M> Proxy<A> for MessageProxy<M>
where
    A: Actor,
    A: Handler<M>,
    M: Message + Send,
    M::Result: Send,
{
    async fn handle(&mut self, actor: &mut A, ctx: &Context<A>) {
        let message = self
            .message
            .take()
            .expect("unable to process message twice");

        let result = <A as Handler<M>>::handle(actor, ctx, message).await;

        if let Some(tx) = self.response.take() {
            let _ = tx.send(result);
        }
    }
}
