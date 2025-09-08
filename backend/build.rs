use std::{env, process::Command};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .compile(
            &["../proto/log/v1/log.proto", "../proto/dt/v1/dt.proto"],
            &["../proto"],
        )?;

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

    Ok(())
}