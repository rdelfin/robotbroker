use crate::protos::register_module_request::HostIp;
use std::net::{IpAddr, Ipv4Addr};
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
pub enum NodeAddress {
    Network { ip: IpAddr, port: u32 },
    SocketFile { path: String },
}

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub address: NodeAddress,
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

impl NodeAddress {
    pub fn from_proto_data(ip: &HostIp, port: u32) -> Result<NodeAddress, NodeManagerError> {
        let ip = match ip {
            HostIp::Ipv4(val) => IpAddr::V4(Ipv4Addr::new(
                get_byte(*val, 3),
                get_byte(*val, 2),
                get_byte(*val, 1),
                get_byte(*val, 0),
            )),
            HostIp::Ipv6(bytes) => {
                if bytes.len() != 16 {
                    return Err(NodeManagerError::InvalidIPv6Format(bytes.len()));
                }
                IpAddr::V6(
                    [
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6],
                        bytes[7], bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13],
                        bytes[14], bytes[15],
                    ]
                    .into(),
                )
            }
        };

        Ok(NodeAddress::Network { ip, port })
    }
}

fn get_byte(v: u32, idx: u8) -> u8 {
    (v >> (idx * 8)) as u8
}
