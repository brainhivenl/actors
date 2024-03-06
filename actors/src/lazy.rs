use tokio::sync::oneshot;

use crate::{Actor, Addr, Handler, Message};

pub enum LazyActor<A: Actor> {
    Started(Addr<A>),
    NotStarted(Option<A>),
}

impl<A: Actor> LazyActor<A> {
    pub fn new(actor: A) -> Self {
        Self::NotStarted(Some(actor))
    }

    pub async fn send<M>(&mut self, msg: M) -> Result<M::Result, oneshot::error::RecvError>
    where
        A: Handler<M>,
        M: Message + Send + 'static,
        M::Result: Send,
    {
        match self {
            LazyActor::Started(addr) => addr.send(msg).await,
            LazyActor::NotStarted(actor) => {
                let addr = actor.take().unwrap().start();
                *self = LazyActor::Started(addr.clone());
                addr.send(msg).await
            }
        }
    }

    pub fn do_send<M>(&mut self, msg: M)
    where
        A: Handler<M>,
        M: Message + Send + 'static,
        M::Result: Send,
    {
        match self {
            LazyActor::Started(addr) => addr.do_send(msg),
            LazyActor::NotStarted(actor) => {
                let addr = actor.take().unwrap().start();
                *self = LazyActor::Started(addr.clone());
                addr.do_send(msg);
            }
        }
    }
}
