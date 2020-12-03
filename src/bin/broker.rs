use broker::{
    nodes::NodeManager,
    protos::{
        broker_server::{Broker, BrokerServer},
        RegisterModuleRequest, RegisterModuleResponse, RegisterPublisherRequest,
        RegisterPublisherResponse, RegisterSubscriberRequest, RegisterSubscriberResponse,
    },
};
use std::sync::{Arc, Mutex};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct MyBroker {
    nodes: Arc<Mutex<NodeManager>>,
}

#[tonic::async_trait]
impl Broker for MyBroker {
    async fn register_module(
        &self,
        request: Request<RegisterModuleRequest>,
    ) -> Result<Response<RegisterModuleResponse>, Status> {
        self.nodes
            .lock()
            .map_err(|_| Status::internal("Failed to borrow node manager"))?
            .register_node(request.get_ref());

        Ok(Response::new(RegisterModuleResponse { ok: true }))
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
