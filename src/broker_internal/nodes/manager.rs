use crate::{
    broker_internal::nodes::{LocalNodeStorage, Node, NodeManagerError, NodeStorage},
    protos::broker::RegisterNodeRequest,
};

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
    pub fn register_node(&mut self, req: &RegisterNodeRequest) -> Result<(), NodeManagerError> {
        self.storage.add_node(&Node {
            name: req.node_name.clone(),
        })
    }

    pub fn list_nodes(&mut self) -> Result<Vec<Node>, NodeManagerError> {
        self.storage.get_nodes()
    }
}
