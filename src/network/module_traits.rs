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

pub trait AsyncProtocolModule<Pkt> {
    type EncodeResult;
    type DecodeResult;
    async fn encode(&self, p: Pkt) -> Self::EncodeResult;
    async fn decode(&self, p: Pkt) -> Self::DecodeResult;
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

    async fn create(&self, p: Self::CreateParam) -> Self::CreateResult;

    async fn destroy(&self) -> Self::DestroyResult;

    async fn listen(&self, p: Self::ListenParam) -> Self::ListenResult;

    async fn connect(&self, p: Self::ConnParam) -> Self::ConnResult;

    async fn rx(&self, p: Pkt) -> Self::RxResult;

    async fn tx(&self, p: Pkt) -> Self::TxResult;
}