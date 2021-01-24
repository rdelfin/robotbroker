use broker::broker_internal::{
    nodes::NodeManager,
    protos::{
        broker_server::{Broker, BrokerServer},
        ListNodesRequest, ListNodesResponse, RegisterNodeRequest, RegisterNodeResponse,
        RegisterPublisherRequest, RegisterPublisherResponse, RegisterSubscriberRequest,
        RegisterSubscriberResponse,
    },
    topics::TopicManager,
};
use log::info;
use log4rs;
use std::sync::{Arc, Mutex, MutexGuard};
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct MyBroker {
    nodes: Arc<Mutex<NodeManager>>,
    topics: Arc<Mutex<TopicManager>>,
}

impl MyBroker {
    fn get_node_manager(&self) -> Result<MutexGuard<NodeManager>, Status> {
        self.nodes
            .lock()
            .map_err(|_| Status::internal("Failed to borrow node manager"))
    }

    fn get_topic_manager(&self) -> Result<MutexGuard<TopicManager>, Status> {
        self.topics
            .lock()
            .map_err(|_| Status::internal("Failed to borrow topic manager"))
    }
}

#[tonic::async_trait]
impl Broker for MyBroker {
    async fn register_node(
        &self,
        request: Request<RegisterNodeRequest>,
    ) -> Result<Response<RegisterNodeResponse>, Status> {
        info!("called RegisterNode()");
        self.get_node_manager()?
            .register_node(request.get_ref())
            .map_err(Into::<Status>::into)?;

        Ok(Response::new(RegisterNodeResponse { ok: true }))
    }

    async fn list_nodes(
        &self,
        _: Request<ListNodesRequest>,
    ) -> Result<Response<ListNodesResponse>, Status> {
        info!("called ListNodes()");
        let nodes = self
            .get_node_manager()?
            .list_nodes()
            .map_err(Into::<Status>::into)?;
        Ok(Response::new(ListNodesResponse {
            nodes: nodes.into_iter().map(Into::into).collect(),
        }))
    }

    async fn register_publisher(
        &self,
        request: Request<RegisterPublisherRequest>,
    ) -> Result<Response<RegisterPublisherResponse>, Status> {
        info!("called RegisterPublisher()");
        let req_ref = request.get_ref();
        self.get_topic_manager()?
            .add_publisher(&req_ref.channel_name, &req_ref.node_name)
            .map_err(Into::<Status>::into)?;
        Ok(Response::new(RegisterPublisherResponse {}))
    }

    async fn register_subscriber(
        &self,
        request: Request<RegisterSubscriberRequest>,
    ) -> Result<Response<RegisterSubscriberResponse>, Status> {
        info!("called RegisterSubscriber()");
        let req_ref = request.get_ref();
        self.get_topic_manager()?
            .add_subscriber(&req_ref.channel_name, &req_ref.node_name)
            .map_err(Into::<Status>::into)?;
        Ok(Response::new(RegisterSubscriberResponse {}))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let addr = "[::1]:50051".parse()?;
    let broker = MyBroker::default();

    Server::builder()
        .add_service(BrokerServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}
