use crate::{
    nodes::{LocalNodeStorage, Node, NodeAddress, NodeStorage},
    protos::RegisterModuleRequest,
};
use std::net::{IpAddr, Ipv4Addr};

pub struct NodeManager {
    storage: Box<dyn NodeStorage + Send + Sync>,
}

impl Default for NodeManager {
    fn default() -> Self {
        NodeManager {
            storage: Box::new(LocalNodeStorage::default()),
        }
    }
}

impl NodeManager {
    pub fn register_node(&mut self, req: &RegisterModuleRequest) {
        self.storage.add_node(&Node {
            name: req.module_name.clone(),
            address: NodeAddress::Network {
                ip: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port: 3933,
            },
        });
    }
}
