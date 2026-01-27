fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "../../uq/proto";
    println!("cargo:rerun-if-changed={}", proto_root);

    connectrpc_axum_build::compile_dir(proto_root)
        .with_prost_config(|config| {
            // We'll need FromRow to map from SQLite
            config.type_attribute("uq.v1.Event", "#[derive(sqlx::FromRow)]");
        })
        .compile()?;

    Ok(())
}
