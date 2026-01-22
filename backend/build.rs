use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    connectrpc_axum_build::compile_dir("../proto")
        .with_prost_config(|config| {
            config.type_attribute("dt.v1.Event", "#[derive(sqlx::FromRow)]");
        })
        .compile()?;

    if cfg!(feature = "build_frontend") {
        let npm_install_output = Command::new("npm")
            .arg("install")
            .current_dir("../frontend")
            .output()?;

        if !npm_install_output.status.success() {
            panic!(
                "npm install failed:\nstdout: {}\nstderr: {}\n",
                String::from_utf8_lossy(&npm_install_output.stdout),
                String::from_utf8_lossy(&npm_install_output.stderr)
            );
        }

        let npm_generate_output = Command::new("npm")
            .arg("run")
            .arg("generate")
            .current_dir("../frontend")
            .output()?;

        if !npm_generate_output.status.success() {
            panic!(
                "npm run generate failed:\nstdout: {}\nstderr: {}\n",
                String::from_utf8_lossy(&npm_generate_output.stdout),
                String::from_utf8_lossy(&npm_generate_output.stderr)
            );
        }

        let output = Command::new("npm")
            .arg("run")
            .arg("build")
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
