use tokio::sync::{mpsc, oneshot};
use tokio_util::sync::CancellationToken;

use crate::{
    handler::{MessageProxy, Proxy},
    Actor, Handler, Message,
};

pub struct Addr<A: Actor> {
    tx: mpsc::Sender<Box<dyn Proxy<A>>>,
    pub(crate) token: CancellationToken,
}

impl<A> Clone for Addr<A>
where
    A: Actor,
{
    fn clone(&self) -> Self {
        Addr {
            tx: self.tx.clone(),
            token: self.token.clone(),
        }
    }
}

impl<A: Actor> Addr<A> {
    pub fn new(tx: mpsc::Sender<Box<dyn Proxy<A>>>, token: CancellationToken) -> Self {
        Addr { tx, token }
    }

    pub fn stop(&self) {
        self.token.cancel();
    }

    pub async fn send<M>(&self, msg: M) -> Result<M::Result, oneshot::error::RecvError>
    where
        A: Handler<M>,
        M: Message + Send + 'static,
        M::Result: Send,
    {
        let tx = self.tx.clone();
        let (mtx, mrx) = oneshot::channel();

        tokio::spawn(async move {
            let proxy = MessageProxy::new(msg, Some(mtx));
            let _ = tx.send(Box::new(proxy)).await;
        });

        mrx.await
    }

    pub fn do_send<M>(&self, msg: M)
    where
        A: Handler<M>,
        M: Message + Send + 'static,
        M::Result: Send,
    {
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let proxy = MessageProxy::new(msg, None);
            let _ = tx.send(Box::new(proxy)).await;
        });
    }

    pub async fn wait(&self) {
        self.token.cancelled().await;
    }
}
