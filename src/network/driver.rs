use std::pin::Pin;
use crate::network::async_modules::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;

pub(crate) struct NetworkDriver {
    
}

impl AsyncNetIOModule<NetworkPacket> for NetworkDriver {
    type RxResult = Result<(), ()>;
    type TxResult = Result<(), ()>;

    fn rx(&self, p: NetworkPacket) -> Pin<Box<dyn Future<Output = Self::RxResult> >> {
        Box::pin(async move {
            println!("!!!!!!!!!driver rx test, {:?}", p);
            Ok(())
        })
    }
    fn tx(&self, p: NetworkPacket) -> Pin<Box<dyn Future<Output = Self::TxResult>>> {
        Box::pin(async move {
            println!("!!!!!!!!!driver tx test. {:?}", p);
            Ok(())
        })
    }
}