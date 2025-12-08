use std::pin::Pin;
use std::sync::Arc;
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::ProtocolMetaData;

pub(crate) struct NetworkDriver {
    
}

impl AsyncNetIOModule<NetworkPacket> for NetworkDriver {
    type RxResult = (NetworkPacket, Result<ProtocolMetaData, ()>);
    type TxResult = (NetworkPacket, Result<(), ()>);

    async fn rx(self: Arc<Self>, p: NetworkPacket) -> Self::RxResult {
        // let res;
        // (p, res) = self.driver_layer.rx(p).await;
        println!("!!!!!!!!!driver rx test, {:?}", p);
        (p, Ok(crate::network::protocol::ProtocolMetaData::new()))
    }
    async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
        println!("!!!!!!!!!driver tx test. {:?}", p);
        (p, Ok(()))
    }
}