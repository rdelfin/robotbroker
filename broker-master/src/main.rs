mod nodes;
mod uds;

use crate::{nodes::NodeManager, uds::UdsGenerator};
use broker_protos::broker::{
    broker_server::{Broker, BrokerServer},
    DeleteNodeRequest, DeleteNodeResponse, HeartbeatRequest, HeartbeatResponse, ListNodesRequest,
    ListNodesResponse, RegisterNodeRequest, RegisterNodeResponse,
};
use log::info;
use std::{
    io,
    sync::{Arc, Mutex, MutexGuard},
};
use tonic::{transport::Server, Request, Response, Status};

struct BrokerImpl {
    nodes: Arc<Mutex<NodeManager>>,
    #[allow(dead_code)]
    uds_generator: Arc<UdsGenerator>,
}

impl BrokerImpl {
    fn new() -> io::Result<BrokerImpl> {
        let uds_generator = Arc::new(UdsGenerator::new()?);
        Ok(BrokerImpl {
            uds_generator: uds_generator.clone(),
            nodes: Arc::new(Mutex::new(NodeManager::new(uds_generator.clone()))),
        })
    }

    fn get_node_manager(&self) -> MutexGuard<NodeManager> {
        self.nodes.lock().unwrap()
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
            uds_address: node.uds,
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

    async fn heartbeat(
        &self,
        request: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        self.get_node_manager()
            .update_heartbeat(&request.get_ref().node_name)
            .map_err(Into::<Status>::into)?;
        Ok(Response::new(HeartbeatResponse {}))
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let broker = BrokerImpl::new()?;

    Server::builder()
        .add_service(BrokerServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}
