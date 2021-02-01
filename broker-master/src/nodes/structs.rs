use broker_protos::broker::Node as ProtoNode;
use std::time::Instant;
use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum NodeManagerError {
    #[error("Node by the name `{0}` does not exist")]
    NodeDoesNotExist(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub uds: String,
    pub last_hb: Instant,
}

impl Into<Status> for NodeManagerError {
    fn into(self) -> Status {
        match self {
            NodeManagerError::NodeDoesNotExist(_) => {
                Status::failed_precondition(format!("{:?}", self))
            }
        }
    }
}

impl Into<ProtoNode> for Node {
    fn into(self) -> ProtoNode {
        ProtoNode { name: self.name }
    }
}
