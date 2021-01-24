use crate::broker_internal::nodes::{Node, NodeManagerError};
use std::collections::HashMap;

pub trait NodeStorage {
    fn add_node(&mut self, node: &Node) -> Result<(), NodeManagerError>;
    fn remove_node(&mut self, name: &str) -> Result<(), NodeManagerError>;
    fn get_nodes(&self) -> Result<Vec<Node>, NodeManagerError>;
}

#[derive(Default)]
pub struct LocalNodeStorage {
    data: HashMap<String, Node>,
}

impl NodeStorage for LocalNodeStorage {
    fn add_node(&mut self, node: &Node) -> Result<(), NodeManagerError> {
        self.data.insert(node.name.to_string(), node.clone());
        Ok(())
    }

    fn remove_node(&mut self, name: &str) -> Result<(), NodeManagerError> {
        if !self.data.contains_key(name) {
            Err(NodeManagerError::NodeDoesNotExist(name.to_string()))
        } else {
            self.data.remove(name);
            Ok(())
        }
    }

    fn get_nodes(&self) -> Result<Vec<Node>, NodeManagerError> {
        Ok(self.data.values().cloned().collect())
    }
}
