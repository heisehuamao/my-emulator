use std::collections::HashMap;
use std::pin::Pin;
use std::sync::{Arc, RwLock, Weak};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use crate::network::ip::IPAddress;
use crate::network::ipv4::{IPv4Entry, IPv4Key};
use crate::network::module_traits::{AsyncNetIOModule, SyncNetIOModule};
use crate::network::packet::NetworkPacket;
use crate::network::protocol::{NetworkProtocolMng, ProtocolType};
use crate::network::stack::NetworkStack;
use crate::network::user_app::UsrApplication;

pub struct Socket {
    slot: u32,
    seq: u32,
}

impl Socket {
    fn new(slot: u32, seq: u32) -> Socket {
        Socket { slot, seq }
    }
}

pub(crate) struct SocketRes {
    id: Socket,
    proto: ProtocolType,
    owner: Arc<UsrApplication>,
}

impl SocketRes {
    fn new(id: Socket, proto: ProtocolType, owner: Arc<UsrApplication>) -> SocketRes {
        SocketRes {
            id, proto, owner
        }
    }
}

pub(crate) struct NetworkSocketLayer {
    stack: Weak<NetworkStack>,
    sock_map: RwLock<Vec<Option<Arc<SocketRes>>>>,
    id_generator: AtomicU32,
    id_seq: AtomicU32,
    id_max: u32,
    // id_exceeded: AtomicBool,
    id_free_set: RwLock<HashMap<u32, u32>>,
    id_busy_set: RwLock<HashMap<u32, u32>>,
}

impl NetworkSocketLayer {
    pub(crate) fn new() -> Self {
        let max = (1 << 20) as u32; // 1,048,576
        NetworkSocketLayer {
            stack: Default::default(),
            sock_map: RwLock::new(vec![None; max as usize]),
            // common: NetworkProtocolMng::new(ProtocolType::None),
            id_generator: AtomicU32::new(1),
            id_seq: AtomicU32::new(1),
            id_max: max,
            // id_exceeded: Default::default(),
            id_free_set: RwLock::new(HashMap::new()),
            id_busy_set: RwLock::new(HashMap::new()),
        }
    }

    fn allocate_id(&self) -> Result<u32, ()> {
        // 1. Try free-set
        {
            let mut free = self.id_free_set.write().unwrap();
            if let Some((&id, _)) = free.iter().next() {
                free.remove(&id);
                self.id_busy_set.write().unwrap().insert(id, id);
                return Ok(id);
            }
        }

        // 2. Allocate new ID from generator
        let id = self.id_generator.fetch_add(1, Ordering::SeqCst);

        if id >= self.id_max {
            _ = self.id_generator.fetch_sub(1, Ordering::SeqCst);
            return Err(()); // exhausted
        }

        self.id_busy_set.write().unwrap().insert(id, id);
        Ok(id)
    }

    fn revoke_id(&self, id: u32) -> Result<(), ()> {
        let mut busy = self.id_busy_set.write().unwrap();
        if busy.remove(&id).is_some() {
            self.id_free_set.write().unwrap().insert(id, id);
            return Ok(());
        } else {
            return Err(());
        }
    }

    fn socket_create(&self) -> Result<Socket, ()> {
        if let Ok(id) = self.allocate_id() {
            let sk_id = Socket::new(id, self.id_seq.fetch_add(1, Ordering::SeqCst));
            Ok(sk_id)
        } else {
            Err(())
        }
    }

    fn socket_bind(&self, id: &Socket, p: &SockBindParam) -> Result<(), ()> {
        let Some(stack) = self.stack.upgrade() else {
            return Err(());
        };

        // TODO: creat socket res
        match &p.proto {
            ProtocolType::UDP => {
                match &p.ip {
                    IPAddress::IPv4Addr(v4) => {
                        stack.add_udp_v4(v4, p.port)
                    },
                    IPAddress::IPv6Addr(v6) => Err(())
                }
            },
            _ => Err(()),
        }
    }
    
    fn socket_listen(&self, id: &Socket, p: &SockListenParam) -> Result<(), ()> {
        Err(())
    }
    
    // called by rx procedure
    fn socket_accept(&self, id: &Socket, p: &SockAcceptParam) -> Result<(), ()> {
        Err(())
    }
    
    fn socket_connect(&self, id: &Socket, pkt: &mut NetworkPacket, p: &SocketConnectParam) -> Result<(), ()> {
        Err(())
    }
    
    fn socket_send(&self, id: &Socket, pkt: &mut NetworkPacket, p: &SocketSendParam) -> Result<(), ()> {
        Err(())
    }
    
    // NOTE: called by rx procedure
    fn socket_recv(&self, id: &Socket, pkt: &mut NetworkPacket, p: &SocketRecvParam) -> Result<(), ()> {
        Err(())
    }
    
    fn socket_close(&self, id: &Socket) -> Result<(), ()> {
        Err(())
    }

    // note: called after bidirectionally closed
    fn socket_destroy(&self, socket: &Socket) -> Result<(), ()> {
        if let Ok(_) = self.revoke_id(socket.slot) {
            // TODO: delete sub res
            Ok(())
        } else {
            Err(())
        }
    }

    // fn create_udp_socket(&self, socket: &Socket, p: &SockCreateParam) -> Result<(), ()> {
    //     Err(())
    // }

}

pub struct SockCreateParam {}

pub struct SockBindParam {
    proto: ProtocolType,
    port: u16,
    ip: IPAddress,
}

pub struct SockListenParam {
}

pub struct SockConnParam {}
pub struct SockAcceptParam {}

pub struct SocketConnectParam{}

pub struct SocketSendParam{}

pub struct SocketRecvParam{}

// impl SyncSocketModule<NetworkPacket> for NetworkSocketLayer {
//     type Identifier = Socket;
//     type CreateParam = SockCreateParam;
//     type BindParam = SockBindParam;
//     type ListenParam = SockListenParam;
//     type ConnParam = SockConnParam;
//     type AcceptParam = SockAcceptParam;
//     type CreateResult = Result<Self::Identifier, ()>;
//     type DestroyResult = ();
//     type ListenResult = Result<(), ()>;
//     type ConnResult = ();
//     type RxResult = ();
//     type TxResult = ();
//     type AcceptResult = ();
// 
//    
//     fn create(&self, p: Self::CreateParam) -> Self::CreateResult {
//         // create a socket Identifier
//         let socket_ret = self.create_socket();
//         match socket_ret {
//             Ok(s) => Ok(s),
//             Err(_) => Err(()),
//         }
//     }
// 
//     fn destroy(&self, id: Self::Identifier) -> Self::DestroyResult {
//         todo!()
//     }
// 
//     fn bind(&self, id: Self::Identifier, p: Self::BindParam) -> Self::ListenResult {
//         let Some(stack) = self.stack.upgrade() else {
//             return Err(());
//         };
//         
//         match &p.proto {
//             ProtocolType::UDP => {
//                 match &p.ip {
//                     IPAddress::IPv4Addr(v4) => {
//                         stack.add_udp_v4(v4, p.port)
//                     },
//                     IPAddress::IPv6Addr(v6) => Err(())
//                 }
//             },
//             _ => Err(()),
//         }
//     }
// 
//     fn listen(&self, id: Self::Identifier, p: Self::ListenParam) -> Self::ListenResult {
//         todo!()
//     }
// 
//     fn connect(&self, id: Self::Identifier, p: Self::ConnParam) -> Self::ConnResult {
//         todo!()
//     }
// 
//     fn accept(&self, id: Self::Identifier, p: Self::AcceptParam) -> Self::AcceptResult {
//         todo!()
//     }
// 
//     fn rx(&self, id: Self::Identifier, p: NetworkPacket) -> Self::RxResult {
//         todo!()
//     }
// 
//     fn tx(&self, id: Self::Identifier, p: NetworkPacket) -> Self::TxResult {
//         todo!()
//     }
// }

impl SyncNetIOModule<NetworkPacket> for NetworkSocketLayer {
    type RxResult = (NetworkPacket, Result<(), ()>);
    type TxResult = (NetworkPacket, Result<(), ()>);

    fn rx(self: &Self, p: NetworkPacket) -> Self::RxResult {
        // let res;
        // (p, res) = self.driver_layer.rx(p).await;
        println!("!!!!!!!!!sock rx test, {:?}", p);
        (p, Ok(()))
    }
    fn tx(self: &Self, p: NetworkPacket) -> Self::TxResult {
        println!("!!!!!!!!!sock tx test. {:?}", p);
        (p, Ok(()))
    }
}

// impl AsyncNetIOModule<NetworkPacket> for NetworkSocketLayer {
//     type RxResult = (NetworkPacket, Result<(), ()>);
//     type TxResult = (NetworkPacket, Result<(), ()>);
//
//     async fn rx(self: Arc<Self>, p: NetworkPacket) -> Self::RxResult {
//         // let res;
//         // (p, res) = self.driver_layer.rx(p).await;
//         println!("!!!!!!!!!sock rx test, {:?}", p);
//         (p, Ok(()))
//     }
//     async fn tx(self: Arc<Self>, p: NetworkPacket) -> Self::TxResult {
//         println!("!!!!!!!!!sock tx test. {:?}", p);
//         (p, Ok(()))
//     }
// }