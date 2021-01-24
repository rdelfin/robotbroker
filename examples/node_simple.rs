use anyhow::Result;
use async_trait::async_trait;
use broker::{start, ProgramNode};
use tokio::time::{sleep, Duration};

struct SimpleNode;

#[async_trait]
impl ProgramNode for SimpleNode {
    fn name() -> &'static str {
        "simple_node"
    }

    async fn run(&mut self) -> Result<()> {
        println!("Hello, World!");
        println!("Sleeping 10 seconds...");
        sleep(Duration::from_secs(10)).await;
        println!("Done");
        Ok(())
    }

    async fn ok(&self) -> Result<()> {
        Ok(())
    }
}

fn main() -> Result<()> {
    start(SimpleNode)
}
