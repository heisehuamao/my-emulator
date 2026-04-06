use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicU64;

pub(crate) struct UserEmu {
    u_task: Pin<Box<dyn Future<Output = ()> + Send + Sync>>,
}
pub(crate) struct UserEmuPara {}

pub(crate) struct UserEmuManager {
    user_id_gen: AtomicU64,
    user_emus_all: Arc<RwLock<HashMap<u64, Arc<UserEmu>>>>,
    user_emus_running: Arc<RwLock<Vec<Arc<UserEmu>>>>,
    // TODO: waiting ring, event hash
}

impl UserEmuManager {
    pub(crate) fn new() -> Self {
        UserEmuManager {
            user_id_gen: Default::default(),
            user_emus_all: Arc::new(Default::default()),
            user_emus_running: Arc::new(Default::default()),
        }
    }



    pub(crate) fn start_user(para: UserEmuPara) -> Result<(), ()> {
        Err(())
    }
}