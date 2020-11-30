use tonic::{transport::Server, Request, Response, Status};

use broker_proto::{
    broker_server::{Broker, BrokerServer},
    RegisterModuleRequest, RegisterModuleResponse, RegisterPublisherRequest,
    RegisterPublisherResponse, RegisterSubscriberRequest, RegisterSubscriberResponse,
};

pub mod broker_proto {
    tonic::include_proto!("broker");
}

#[derive(Debug, Default)]
pub struct MyBroker {}

#[tonic::async_trait]
impl Broker for MyBroker {
    async fn register_module(
        &self,
        request: Request<RegisterModuleRequest>,
    ) -> Result<Response<RegisterModuleResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = RegisterModuleResponse { ok: true };

        Ok(Response::new(reply))
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
