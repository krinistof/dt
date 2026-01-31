use std::process::Command;

fn run_npm(args: &[&str], dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("npm").args(args).current_dir(dir).output()?;

    if !output.status.success() {
        panic!(
            "npm {} failed in {}:\nstdout: {}\nstderr: {}",
            args.join(" "),
            dir,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Note: We no longer compile protos here, as they are in `uq`.
    // We just handle the frontend build trigger.

    if cfg!(feature = "build_frontend") {
        // Build uq-client first
        run_npm(&["install"], "../uq/client")?;
        run_npm(&["run", "generate"], "../uq/client")?;
        run_npm(&["run", "build"], "../uq/client")?;

        // Build frontend
        run_npm(&["install"], "../frontend")?;
        run_npm(&["run", "build"], "../frontend")?;

        println!("cargo:rerun-if-changed=../frontend");
        println!("cargo:rerun-if-changed=../uq/client");
        println!("cargo:rerun-if-changed=../uq/proto");
    }

    Ok(())
}
