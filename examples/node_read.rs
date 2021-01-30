use broker::protos::node::{node_client::NodeClient, HeartbeatRequest};
use std::convert::TryFrom;
use structopt::StructOpt;
use tokio::net::UnixStream;
use tonic::transport::{Endpoint, Uri};
use tower::service_fn;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long)]
    socket: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // We will ignore this uri because uds do not use it
    // if your connector does use the uri it will be provided
    // as the request to the `MakeConnection`.
    let channel = Endpoint::try_from("http://[::]:50051")?
        .connect_with_connector(service_fn(|_: Uri| {
            let opt = Opt::from_args();
            // Connect to a Uds socket
            UnixStream::connect(opt.socket)
        }))
        .await?;

    let mut client = NodeClient::new(channel);

    let request = tonic::Request::new(HeartbeatRequest {});

    let response = client.heartbeat(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
