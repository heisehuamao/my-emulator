use std::sync::{Arc, RwLock};
use crate::network::stack::NetworkStack;
use crate::network::user_app::UserApplication;

pub struct UserContainer {
    apps: Vec<RwLock<Arc<UserApplication>>>,
    stack: Arc<NetworkStack>
}

impl UserContainer {
    pub fn new(stack: Arc<NetworkStack>) -> Self {
        UserContainer {
            apps: Vec::new(), stack
        }
    }
    
    pub fn run(&mut self) {
        // TODO: start schedule user applications
        println!("Running user container.............");
    }
}