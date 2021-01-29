use crate::protos::broker::{broker_client::BrokerClient, RegisterNodeRequest};
use anyhow::Result;
use tonic::{transport::Channel, Request};

/// This struct is used to manage a connection to the robot broker, and to provide an
/// interface for all service discovery needs that arise.
pub struct NodeHandle {
    client: BrokerClient<Channel>,
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
            client: BrokerClient::connect("http://[::1]:50051").await?,
            name: name.to_string(),
            uds_address: "".to_string(),
        };
        handle.register_node(name).await?;
        Ok(handle)
    }

    async fn register_node(&mut self, name: &str) -> Result<()> {
        let request = Request::new(RegisterNodeRequest {
            node_name: name.to_string(),
        });
        let response = self.client.register_node(request).await?;
        println!(
            "Registered node with name {} successfully! UDS: {}",
            name,
            response.get_ref().uds_address
        );
        self.uds_address = response.get_ref().uds_address.to_string();

        Ok(())
    }
}
