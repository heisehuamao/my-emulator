use std::pin::Pin;
use std::sync::Arc;
use crate::network::ipv4::{Ipv4Entry, Ipv4Key};
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolHeaderType};
use crate::network::user_app::UsrApplication;

pub(crate) struct SocketRes {
    id: u64,
    owner: Arc<UsrApplication>,
}

pub(crate) struct NetworkSocket {
    common: NetworkProtocolMng<u64, Arc<SocketRes>>,
}

impl NetworkSocket {
    pub(crate) fn new() -> Self {
        NetworkSocket {
            common: NetworkProtocolMng::new(ProtocolHeaderType::None),
        }
    }
}

impl AsyncNetIOModule<NetworkPacket> for NetworkSocket {
    type RxResult = (NetworkPacket, Result<(), ()>);
    type TxResult = (NetworkPacket, Result<(), ()>);

    async fn rx(self: Arc<Self>, p: NetworkPacket) -> Self::RxResult {
        // let res;
        // (p, res) = self.driver_layer.rx(p).await;
        println!("!!!!!!!!!sock rx test, {:?}", p);
        (p, Ok(()))
    }
    async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
        println!("!!!!!!!!!sock tx test. {:?}", p);
        (p, Ok(()))
    }
}