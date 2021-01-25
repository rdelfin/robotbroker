use anyhow::anyhow;
use broker::{
    broker_internal::{nodes::NodeManager, topics::TopicManager},
    protos::broker::{
        broker_server::{Broker, BrokerServer},
        DeleteNodeRequest, DeleteNodeResponse, ListNodesRequest, ListNodesResponse,
        RegisterNodeRequest, RegisterNodeResponse,
    },
};
use log::info;
use log4rs;
use std::sync::{Arc, Mutex, MutexGuard};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct BrokerImpl {
    nodes: Arc<Mutex<NodeManager>>,
    topics: Arc<Mutex<TopicManager>>,
}

impl BrokerImpl {
    fn get_node_manager(&self) -> MutexGuard<NodeManager> {
        self.nodes.lock().unwrap()
    }

    fn get_topic_manager(&self) -> MutexGuard<TopicManager> {
        self.topics.lock().unwrap()
    }
}

#[tonic::async_trait]
impl Broker for BrokerImpl {
    async fn register_node(
        &self,
        request: Request<RegisterNodeRequest>,
    ) -> Result<Response<RegisterNodeResponse>, Status> {
        let node = self
            .get_node_manager()
            .register_node(&request.get_ref())
            .map_err(Into::<Status>::into)?;
        info!(
            "Registered a new node named {} using UDS {}",
            request.get_ref().node_name,
            node.uds
        );
        Ok(Response::new(RegisterNodeResponse {
            uds_address: node.uds.to_string(),
        }))
    }

    async fn list_nodes(
        &self,
        _: Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesResponse>, Status> {
        let nodes = self
            .get_node_manager()
            .list_nodes()
            .map_err(Into::<Status>::into)?;
        Ok(Response::new(ListNodesResponse {
            nodes: nodes.into_iter().map(Into::into).collect(),
        }))
    }

    async fn delete_node(
        &self,
        request: Request<DeleteNodeRequest>,
    ) -> Result<Response<DeleteNodeResponse>, Status> {
        info!(
            "Recieved request to delete node {}",
            request.get_ref().node_name
        );
        Ok(Response::new(DeleteNodeResponse {}))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let broker = BrokerImpl::default();

    Server::builder()
        .add_service(BrokerServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}
