use std::net::IpAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NodeManagerError {
    #[error("Node by the name `{0}` does not exist")]
    NodeDoesNotExist(String),
}

#[derive(Debug, Clone)]
pub enum NodeAddress {
    Network { ip: IpAddr, port: i32 },
    SocketFile { path: String },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub address: NodeAddress,
}
