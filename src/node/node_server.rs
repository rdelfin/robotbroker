use crate::{
    node::NodeHandle,
    protos::node::{
        node_server::{Node, NodeServer},
        HeartbeatRequest, HeartbeatResponse,
    },
};
use futures::TryFutureExt;
use std::sync::Arc;
use tokio::net::UnixListener;
use tonic::{transport::Server, Request, Response, Status};

pub struct NodeServerImpl {
    nh: Arc<NodeHandle>,
}

impl NodeServerImpl {
    pub fn new(nh: Arc<NodeHandle>) -> NodeServerImpl {
        NodeServerImpl { nh }
    }
}

#[tonic::async_trait]
impl Node for NodeServerImpl {
    async fn heartbeat(
        &self,
        _: Request<HeartbeatRequest>,
    ) -> Result<Response<HeartbeatResponse>, Status> {
        Ok(Response::new(HeartbeatResponse {
            name: self.nh.name().into(),
        }))
    }
}

pub async fn start_server(nh: Arc<NodeHandle>, uds_address: &str) -> anyhow::Result<()> {
    let node = NodeServerImpl::new(nh);
    let incoming = {
        let uds = UnixListener::bind(uds_address)?;

        async_stream::stream! {
            while let item = uds.accept().map_ok(|(st, _)| unix::UnixStream(st)).await {
                yield item;
            }
        }
    };

    println!("Creating server on {}", uds_address);
    tokio::spawn(async move {
        Server::builder()
            .add_service(NodeServer::new(node))
            .serve_with_incoming(incoming)
            .await
            .unwrap();
    });

    Ok(())
}

mod unix {
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };

    use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
    use tonic::transport::server::Connected;

    #[derive(Debug)]
    pub struct UnixStream(pub tokio::net::UnixStream);

    impl Connected for UnixStream {}

    impl AsyncRead for UnixStream {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_read(cx, buf)
        }
    }

    impl AsyncWrite for UnixStream {
        fn poll_write(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &[u8],
        ) -> Poll<std::io::Result<usize>> {
            Pin::new(&mut self.0).poll_write(cx, buf)
        }

        fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_flush(cx)
        }

        fn poll_shutdown(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<std::io::Result<()>> {
            Pin::new(&mut self.0).poll_shutdown(cx)
        }
    }
}
