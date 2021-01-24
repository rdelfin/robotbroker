fn main() -> Result<(), Box<dyn std::error::Error>> {
    ::capnpc::CompilerCommand::new()
        .src_prefix("capnp")
        .file("capnp/core.capnp")
        .file("capnp/node.capnp")
        .run()?;
    tonic_build::compile_protos("proto/broker.proto")?;
    tonic_build::compile_protos("proto/node.proto")?;
    Ok(())
}
