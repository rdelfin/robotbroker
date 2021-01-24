pub mod broker_internal;
mod node;

pub mod core_capnp {
    include!(concat!(env!("OUT_DIR"), "/core_capnp.rs"));
}
pub mod node_capnp {
    include!(concat!(env!("OUT_DIR"), "/node_capnp.rs"));
}

pub use self::node::{start, ProgramNode};
