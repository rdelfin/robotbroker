use crate::protos::broker::{broker_client::BrokerClient, RegisterNodeRequest};
use anyhow::Result;
use async_trait::async_trait;
use tokio::runtime::Builder;
use tonic::{transport::Channel, Request};

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
        let mut nh = NodeHandle::new().await?;
        nh.register_node(node.name()).await?;
        node.run().await
    });
    futures::executor::block_on(jh)?
}

// Internal struct for managing a connection to the broker.
struct NodeHandle {
    client: BrokerClient<Channel>,
}

impl NodeHandle {
    async fn new() -> Result<NodeHandle> {
        Ok(NodeHandle {
            client: BrokerClient::connect("http://[::1]:50051").await?,
        })
    }

    async fn register_node(&mut self, name: &str) -> Result<()> {
        let request = Request::new(RegisterNodeRequest {
            node_name: name.to_string(),
        });
        let response = self.client.register_node(request).await?;
        println!(
            "Registered node with name {} successfully! Ok: {}",
            name,
            response.get_ref().ok
        );
        Ok(())
    }
}
