fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/broker.proto")?;
    tonic_build::compile_protos("proto/node.proto")?;
    Ok(())
}
