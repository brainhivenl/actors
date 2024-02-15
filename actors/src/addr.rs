use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::Actor;

pub struct Addr<A: Actor> {
    tx: Sender<A::Message>,
    token: CancellationToken,
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
    pub fn new(tx: Sender<A::Message>) -> Self {
        Addr {
            tx,
            token: CancellationToken::new(),
        }
    }

    pub fn stop(&self) {
        self.token.cancel();
    }

    pub fn do_send(&self, msg: A::Message) {
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let _ = tx.send(msg).await;
        });
    }

    pub async fn wait(&self) {
        self.token.cancelled().await;
    }
}
