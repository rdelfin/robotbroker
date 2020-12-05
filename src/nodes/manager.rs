use crate::{
    nodes::{LocalNodeStorage, Node, NodeAddress, NodeManagerError, NodeStorage},
    protos::RegisterNodeRequest,
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
            address: NodeAddress::from_proto_data(
                req.host_ip
                    .as_ref()
                    .ok_or(NodeManagerError::MissingField("host_ip".to_string()))?,
                req.port,
            )?,
        })
    }

    pub fn list_nodes(&mut self) -> Result<Vec<Node>, NodeManagerError> {
        self.storage.get_nodes()
    }
}
