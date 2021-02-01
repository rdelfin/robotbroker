use anyhow::Result;
use async_trait::async_trait;
use robotbroker::{start, NodeHandle, ProgramNode};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

struct SimpleNode;

#[async_trait]
impl ProgramNode for SimpleNode {
    fn name(&self) -> &'static str {
        "simple_node"
    }

    async fn run(&mut self, _nh: Arc<NodeHandle>) -> Result<()> {
        println!("Hello, World!");
        println!("Looping...");

        loop {
            sleep(Duration::from_millis(25)).await;
        }
    }

    async fn ok(&self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    start(SimpleNode)
}
