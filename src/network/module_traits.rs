use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

// for stack block
pub trait AsyncNetIOModule<Pkt> {
    type RxResult;
    type TxResult;
    // type OutputErr;

//     fn rx(self: Arc<Self>, p: Pkt) -> Pin<Box<dyn Future<Output = Self::RxResult>>>;
//     fn tx(self: Arc<Self>, p: Pkt) -> Pin<Box<dyn Future<Output = Self::TxResult>>>;
    async fn rx(self: Arc<Self>, p: Pkt) -> Self::RxResult;
    async fn tx(self: Arc<Self>, p: Pkt) -> Self::TxResult;
}

pub trait SyncNetIOModule<Pkt> {
    type RxResult;
    type TxResult;

    fn rx(&self, p: Pkt) -> Self::RxResult;
    fn tx(&self, p: Pkt) -> Self::TxResult;
}

pub trait AsyncSocketModule<Pkt> {
    type CreateParam;
    type ListenParam;
    type ConnParam;

    type CreateResult;
    type DestroyResult;
    type ListenResult;
    type ConnResult;
    type RxResult;
    type TxResult;

    fn create(&self, p: Self::CreateParam)
              -> Pin<Box<dyn Future<Output = Self::CreateResult>>>;

    fn destroy(&self)
               -> Pin<Box<dyn Future<Output = Self::DestroyResult>>>;

    fn listen(&self, p: Self::ListenParam)
              -> Pin<Box<dyn Future<Output = Self::ListenResult>>>;

    fn connect(&self, p: Self::ConnParam)
               -> Pin<Box<dyn Future<Output = Self::ConnResult>>>;

    fn rx(&self, p: Pkt)
          -> Pin<Box<dyn Future<Output = Self::RxResult>>>;

    fn tx(&self, p: Pkt)
          -> Pin<Box<dyn Future<Output = Self::TxResult>>>;
}