use anyhow::anyhow;
use broker::{
    broker_internal::{nodes::NodeManager, topics::TopicManager},
    core_capnp::core,
};
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt};
use log::info;
use log4rs;
use std::{
    net::ToSocketAddrs,
    sync::{Arc, Mutex, MutexGuard},
};
use tokio::net::TcpListener;

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

impl core::Server for BrokerImpl {
    fn create_node(
        &mut self,
        params: core::CreateNodeParams,
        mut _results: core::CreateNodeResults,
    ) -> Promise<(), capnp::Error> {
        info!(
            "Recieved request to create node {}",
            pry!(pry!(pry!(params.get()).get_req()).get_name())
        );
        Promise::ok(())
    }

    fn delete_node(
        &mut self,
        params: core::DeleteNodeParams,
        mut _results: core::DeleteNodeResults,
    ) -> Promise<(), capnp::Error> {
        info!(
            "Recieved request to delete node {}",
            pry!(pry!(pry!(params.get()).get_req()).get_name())
        );
        Promise::ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    let addr = "[::1]:50051"
        .to_socket_addrs()?
        .next()
        .ok_or(anyhow!("Could not parse address"))?;

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = TcpListener::bind(&addr).await?;
            let core_client: core::Client = capnp_rpc::new_client(BrokerImpl::default());

            loop {
                let (stream, _) = listener.accept().await?;
                stream.set_nodelay(true)?;
                let (reader, writer) =
                    tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                let network = twoparty::VatNetwork::new(
                    reader,
                    writer,
                    rpc_twoparty_capnp::Side::Server,
                    Default::default(),
                );

                let rpc_system =
                    RpcSystem::new(Box::new(network), Some(core_client.clone().client));

                tokio::task::spawn_local(Box::pin(rpc_system.map(|_| ())));
            }
        })
        .await
}
