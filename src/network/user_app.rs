use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use crate::network::module_traits::AsyncNetIOModule;
use crate::network::packet::NetworkPacket;
use crate::network::stack::NetworkStack;
use crate::network::user_emu::{UserEmu, UserEmuManager};

enum UserApplicationState {
    Uninitialized,
    Init,
    Starting,
    Started,
    Running,
    Stopping,
    Stopped,
}

pub struct UserApplication {
    id: u64,
    name: String,
    state: RwLock<UserApplicationState>,
    stack: Arc<NetworkStack>,
    uemu_manager: UserEmuManager,
}

pub(crate) struct UserApplicationConf {}

impl UserApplication {
    pub(crate) fn new(stack: Arc<NetworkStack>) -> UserApplication {
        UserApplication {
            id: 0,
            name: "".to_string(),
            state: RwLock::new(UserApplicationState::Uninitialized),
            stack, uemu_manager: UserEmuManager::new() }
    }

    pub(crate) fn load(&mut self, conf: UserApplicationConf) -> Result<(), ()> {
        Err(())
    }

    pub(crate) fn start(&mut self) -> Result<(), ()> {
        Err(())
    }

    pub(crate) fn run(&mut self) {
        // start new user_emu, schedule running emus...
    }

    pub(crate) fn stop(&mut self) -> Result<(), ()> {
        Err(())
    }
    
    pub(crate) fn unload(&mut self) -> Result<(), ()> {
        Err(())
    }
}

// impl AsyncNetIOModule<NetworkPacket> for UsrApplication {
//     type RxResult = (NetworkPacket, Result<(), ()>);
//     type TxResult = (NetworkPacket, Result<(), ()>);
//
//     async fn rx(self: Arc<Self>, p: NetworkPacket) -> Self::RxResult {
//         // let res;
//         // (p, res) = self.driver_layer.rx(p).await;
//         println!("!!!!!!!!!app rx test, {:?}", p);
//         (p, Ok(()))
//     }
//     async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
//         println!("!!!!!!!!!app tx test. {:?}", p);
//         (p, Ok(()))
//     }
// }