use crate::protos::broker::Node as ProtoNode;
use thiserror::Error;
use tonic::Status;

#[derive(Error, Debug)]
pub enum NodeManagerError {
    #[error("Node by the name `{0}` does not exist")]
    NodeDoesNotExist(String),
    #[error("Invalid IPv6 data format, {0} bytes provided, 16 expected")]
    InvalidIPv6Format(usize),
    #[error("Field `{0}` was expected but not provided.")]
    MissingField(String),
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
}

impl Into<Status> for NodeManagerError {
    fn into(self) -> Status {
        match self {
            NodeManagerError::MissingField(_) => Status::invalid_argument(format!("{:?}", self)),
            NodeManagerError::InvalidIPv6Format(_) => {
                Status::invalid_argument(format!("{:?}", self))
            }
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
