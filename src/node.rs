use anyhow::Result;
use async_trait::async_trait;
use tokio::runtime::Builder;

/// Classes implementing this trait will define your node. From here, your node will
/// surface all the information needed to get registered, setup, and run.
#[async_trait]
pub trait ProgramNode {
    /// Should return the name of the node as a string. It should *not* change
    /// throughout execution.
    fn name() -> &'static str;

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
pub fn start<N: ProgramNode + std::marker::Send + 'static>(mut node: N) -> Result<()> {
    let rt = Builder::new_multi_thread()
        .thread_name("robot-node-worker")
        .enable_all()
        .build()?;

    let jh = rt.spawn(async move { node.run().await });
    futures::executor::block_on(jh)?
}
