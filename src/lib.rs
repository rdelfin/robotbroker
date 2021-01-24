pub mod broker_internal;
mod node;

pub mod protos {
    pub mod broker {
        tonic::include_proto!("broker");
    }
    pub mod node {
        tonic::include_proto!("node");
    }
}

pub use self::node::{start, ProgramNode};
