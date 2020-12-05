use broker::{
    nodes::NodeManager,
    protos::{
        broker_server::{Broker, BrokerServer},
        ListNodesRequest, ListNodesResponse, Node, RegisterNodeRequest, RegisterNodeResponse,
        RegisterPublisherRequest, RegisterPublisherResponse, RegisterSubscriberRequest,
        RegisterSubscriberResponse,
    },
};
use std::sync::{Arc, Mutex, MutexGuard};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct MyBroker {
    nodes: Arc<Mutex<NodeManager>>,
}

impl MyBroker {
    fn get_node_manager(&self) -> Result<MutexGuard<NodeManager>, Status> {
        self.nodes
            .lock()
            .map_err(|_| Status::internal("Failed to borrow node manager"))
    }
}

#[tonic::async_trait]
impl Broker for MyBroker {
    async fn register_node(
        &self,
        request: Request<RegisterNodeRequest>,
    ) -> Result<Response<RegisterNodeResponse>, Status> {
        self.get_node_manager()?
            .register_node(request.get_ref())
            .map_err(|e| Into::<Status>::into(e))?;

        Ok(Response::new(RegisterNodeResponse { ok: true }))
    }

    async fn list_nodes(
        &self,
        _: Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesResponse>, Status> {
        let nodes = self
            .get_node_manager()?
            .list_nodes()
            .map_err(|e| Into::<Status>::into(e))?;
        Ok(Response::new(ListNodesResponse {
            nodes: nodes.into_iter().map(|n| Into::<Node>::into(n)).collect(),
        }))
    }

    async fn register_publisher(
        &self,
        request: Request<RegisterPublisherRequest>,
    ) -> Result<Response<RegisterPublisherResponse>, Status> {
        Ok(Response::new(RegisterPublisherResponse {
            proxy_ip: "::".to_string(),
            port: 2,
        }))
    }

    async fn register_subscriber(
        &self,
        request: Request<RegisterSubscriberRequest>,
    ) -> Result<Response<RegisterSubscriberResponse>, Status> {
        Ok(Response::new(RegisterSubscriberResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let broker = MyBroker::default();

    Server::builder()
        .add_service(BrokerServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}
