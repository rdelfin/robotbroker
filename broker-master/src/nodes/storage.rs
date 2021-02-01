use crate::nodes::{Node, NodeManagerError};
use std::{collections::HashMap, time::Instant};

pub trait NodeStorage {
    fn add_node(&mut self, node: &Node) -> Result<(), NodeManagerError>;
    fn remove_node(&mut self, name: &str) -> Result<(), NodeManagerError>;
    fn get_nodes(&self) -> Result<Vec<Node>, NodeManagerError>;
    fn update_heartbeat(&mut self, name: &str, ts: Instant) -> Result<(), NodeManagerError>;
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

    fn update_heartbeat(&mut self, name: &str, ts: Instant) -> Result<(), NodeManagerError> {
        let node = self
            .data
            .get_mut(name)
            .ok_or_else(|| NodeManagerError::NodeDoesNotExist(name.to_string()))?;
        node.last_hb = ts;
        Ok(())
    }
}
