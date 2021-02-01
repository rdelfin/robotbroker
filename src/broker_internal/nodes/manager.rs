use crate::{
    broker_internal::nodes::{LocalNodeStorage, Node, NodeManagerError, NodeStorage},
    protos::broker::RegisterNodeRequest,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::time::Instant;
use tempdir::TempDir;

pub struct NodeManager {
    storage: Box<dyn NodeStorage + Send + Sync>,
    temp_dir: TempDir,
}

impl Default for NodeManager {
    fn default() -> Self {
        NodeManager {
            storage: Box::new(LocalNodeStorage::default()),
            temp_dir: TempDir::new("robotbroker").unwrap(),
        }
    }
}

impl NodeManager {
    pub fn register_node(&mut self, req: &RegisterNodeRequest) -> Result<Node, NodeManagerError> {
        let node = Node {
            name: req.node_name.clone(),
            uds: self.generate_uds(),
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

    fn generate_uds(&self) -> String {
        const FILE_LEN: usize = 20;

        let filename: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(FILE_LEN)
            .map(char::from)
            .collect();

        let mut path = self.temp_dir.path().to_owned();
        path.push(&filename);
        path.set_extension("sock");
        path.to_str().unwrap().to_string()
    }
}
