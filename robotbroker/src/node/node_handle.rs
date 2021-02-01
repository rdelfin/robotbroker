use anyhow::Result;
use broker_protos::broker::{broker_client::BrokerClient, HeartbeatRequest, RegisterNodeRequest};
use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};

/// This struct is used to manage a connection to the robot broker, and to provide an
/// interface for all service discovery needs that arise.
pub struct NodeHandle {
    client: Mutex<BrokerClient<Channel>>,
    name: String,
    uds_address: String,
}

impl NodeHandle {
    /// Returns the name of the node, as determined on startup by the return value of
    /// `ProgramNode.run`.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the filename used as a unix domain socket to expose the node's endpoint.
    pub fn uds_address(&self) -> &str {
        &self.uds_address
    }

    pub async fn new(name: &str) -> Result<NodeHandle> {
        let mut handle = NodeHandle {
            client: Mutex::new(BrokerClient::connect("http://[::1]:50051").await?),
            name: name.to_string(),
            uds_address: "".to_string(),
        };
        handle.register_node(name).await?;
        Ok(handle)
    }

    pub async fn heartbeat(&self) -> Result<()> {
        let request = Request::new(HeartbeatRequest {
            node_name: self.name().to_string(),
        });
        self.client.lock().await.heartbeat(request).await?;
        Ok(())
    }

    async fn register_node(&mut self, name: &str) -> Result<()> {
        let request = Request::new(RegisterNodeRequest {
            node_name: name.to_string(),
        });
        let response = self.client.lock().await.register_node(request).await?;
        println!(
            "Registered node with name {} successfully! UDS: {}",
            name,
            response.get_ref().uds_address
        );
        self.uds_address = response.get_ref().uds_address.to_string();

        Ok(())
    }
}
