
use flume::{Receiver, Sender};

#[derive(Debug)]
pub(crate) struct TinyConnection<T> {
    rx: Receiver<T>,
    tx: Sender<T>
}

impl<T> TinyConnection<T> {
    pub(crate) fn new(rx: Receiver<T>, tx: Sender<T>) -> Self {
        Self { rx, tx }
    }

    pub(crate) fn try_send(&self, t: T) -> Result<(), ()> {
        let tx_res = self.tx.try_send(t);
        match tx_res {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub(crate) fn try_recv(&self) -> Result<T, ()> {
        let rx_res = self.rx.try_recv();
        match rx_res {
            Ok(t) => Ok(t),
            Err(_) => Err(()),
        }
    }
}