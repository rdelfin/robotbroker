use crate::core_capnp::core;
use anyhow::anyhow;
use anyhow::Result;
use async_trait::async_trait;
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex},
};
use tokio::runtime::Builder;

/// Classes implementing this trait will define your node. From here, your node will
/// surface all the information needed to get registered, setup, and run.
#[async_trait]
pub trait ProgramNode {
    /// Should return the name of the node as a string. It should *not* change
    /// throughout execution.
    fn name(&self) -> &'static str;

    /// The "main" for your application. This is where most of your code should be
    /// added.
    async fn run(&mut self) -> Result<()>;

    /// Returns true if the node is still fine. This call should generally not block
    /// for any extended period of time. If this function doesn't respond within a
    /// configurable period of time, the broker will request this node to be killed.
    async fn ok(&self) -> Result<()>;
}

/// Here is where you pass in the node to execute in main. In theory, this should be
/// the only thing that gets run in main. The call will be blocking.
pub fn start<N: ProgramNode + std::marker::Send + Sync + 'static>(mut node: N) -> Result<()> {
    let rt = Builder::new_multi_thread()
        .thread_name("robot-node-worker")
        .enable_all()
        .build()?;

    let jh = rt.spawn(async move {
        let nh = NodeHandle::new().await?;
        nh.register_node(node.name()).await?;
        node.run().await
    });
    futures::executor::block_on(jh)?
}

// Internal struct for managing a connection to the broker.
struct NodeHandle {
    rpc_system: Arc<Mutex<RpcSystem<rpc_twoparty_capnp::Side>>>,
}

impl NodeHandle {
    async fn new() -> Result<NodeHandle> {
        let addr = "[::1]:50051"
            .to_socket_addrs()?
            .next()
            .ok_or(anyhow!("Could not parse address"))?;
        let stream = tokio::net::TcpStream::connect(&addr).await?;
        stream.set_nodelay(true)?;
        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network = Box::new(twoparty::VatNetwork::new(
            reader,
            writer,
            rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));
        let rpc_system = Arc::new(Mutex::new(RpcSystem::new(rpc_network, None)));

        tokio::task::spawn_local(rpc_system.lock().unwrap().map(|_| ()));

        Ok(NodeHandle { rpc_system })
    }

    async fn register_node(&self, name: &str) -> Result<()> {
        let core_client: core::Client = self
            .rpc_system
            .lock()?
            .bootstrap(rpc_twoparty_capnp::Side::Server);
        let mut request = core_client.create_node_request();
        request.get().init_req().set_name(&name);

        let reply = request.send().promise.await?;

        println!("Registered node with name {} successfully!", name);
        Ok(())
    }
}
