fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(
            &["../proto/log/v1/log.proto"],
            &["../proto"],
        )?;
    Ok(())
}
