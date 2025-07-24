// backend/grpc-server/build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(
            &["../proto/log.proto"], // Path to your .proto file
            &["../proto"],                // Directory to search for includes
        )?;
    Ok(())
}
