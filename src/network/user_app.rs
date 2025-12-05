use std::pin::Pin;
use std::sync::Arc;
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;
use crate::network::stack::NetworkStack;

pub struct UsrApplication {
    stack: Arc<NetworkStack>,
}

impl UsrApplication {
    pub fn new(stack: Arc<NetworkStack>) -> UsrApplication {
        UsrApplication { stack }
    }
}

impl AsyncNetIOModule<NetworkPacket> for UsrApplication {
    type RxResult = (NetworkPacket, Result<(), ()>);
    type TxResult = (NetworkPacket, Result<(), ()>);

    async fn rx(self: Arc<Self>, p: NetworkPacket) -> Self::RxResult {
        // let res;
        // (p, res) = self.driver_layer.rx(p).await;
        println!("!!!!!!!!!app rx test, {:?}", p);
        (p, Ok(()))
    }
    async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
        println!("!!!!!!!!!app tx test. {:?}", p);
        (p, Ok(()))
    }
}