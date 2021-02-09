use crate::{
    nodes::{LocalNodeStorage, Node, NodeManagerError, NodeStorage},
    uds::UdsGenerator,
};
use broker_protos::broker::RegisterNodeRequest;
use std::{sync::Arc, time::Instant};

pub struct NodeManager {
    storage: Box<dyn NodeStorage + Send + Sync>,
    uds_generator: Arc<UdsGenerator>,
}

impl NodeManager {
    pub fn new(uds_generator: Arc<UdsGenerator>) -> NodeManager {
        NodeManager {
            storage: Box::new(LocalNodeStorage::default()),
            uds_generator,
        }
    }

    pub fn register_node(&mut self, req: &RegisterNodeRequest) -> Result<Node, NodeManagerError> {
        let node = Node {
            name: req.node_name.clone(),
            uds: self.uds_generator.generate_uds(),
            last_hb: Instant::now(),
        };
        self.storage.add_node(&node)?;
        Ok(node)
    }

    pub fn list_nodes(&mut self) -> Result<Vec<Node>, NodeManagerError> {
        self.storage.get_nodes()
    }

    pub fn update_heartbeat(&mut self, name: &str) -> Result<(), NodeManagerError> {
        self.storage.update_heartbeat(name, Instant::now())
    }
}
