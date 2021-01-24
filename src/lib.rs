pub mod broker_internal;
mod node;

pub mod protos {
    tonic::include_proto!("broker");
    tonic::include_proto!("node");
}

pub use self::node::{start, ProgramNode};
