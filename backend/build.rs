use std::{env, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    connectrpc_axum_build::compile_dir("../proto")
        .with_prost_config(|config| {
            config.type_attribute("dt.v1.Event", "#[derive(sqlx::FromRow)]");
        })
        .compile()?;

    if cfg!(feature = "build_frontend") {
        let profile = env::var("PROFILE").unwrap();
        let npm_command = if profile == "release" {
            "build"
        } else {
            "build:debug"
        };

        let output = Command::new("npm")
            .arg("run")
            .arg(npm_command)
            .current_dir("../frontend")
            .output()?;

        if !output.status.success() {
            panic!(
                "npm build failed:\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        println!("cargo:rerun-if-changed=../frontend");
    }

    Ok(())
}
