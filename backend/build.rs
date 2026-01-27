use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Note: We no longer compile protos here, as they are in `uq`.
    // We just handle the frontend build trigger.

    if cfg!(feature = "build_frontend") {
        let npm_install_output = Command::new("npm")
            .arg("install")
            .current_dir("../frontend")
            .output()?;

        if !npm_install_output.status.success() {
            panic!(
                "npm install failed:\nstdout: {}\nstderr: {}",
                String::from_utf8_lossy(&npm_install_output.stdout),
                String::from_utf8_lossy(&npm_install_output.stderr)
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
