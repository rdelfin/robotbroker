use std::{collections::HashMap, net::IpAddr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeManagerError {
    #[error("Node by the name `{0}` does not exist")]
    NodeDoesNotExist(String),
}

pub enum NodeAddress {
    Network { ip: IpAddr, port: i32 },
    SocketFile { path: String },
}

pub struct Node {
    pub name: String,
    pub address: NodeAddress,
}

pub trait NodeStorage {
    fn add_node(node: Node) -> Result<(), NodeManagerError>;
    fn remove_node(name: &str) -> Result<(), NodeManagerError>;
    fn get_address(name: &str) -> Result<NodeAddress, NodeManagerError>;
}
