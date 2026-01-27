#[cfg(test)]
mod tests {
    use std::process::{Child, Command, Stdio};
    use std::time::Duration;
    use tempfile::NamedTempFile;

    struct BackendProcess {
        child: Child,
        #[allow(dead_code)]
        db_file: NamedTempFile,
    }

    impl Drop for BackendProcess {
        fn drop(&mut self) {
            println!("Cleaning up backend process...");
            if let Err(e) = self.child.kill() {
                eprintln!("Failed to kill backend process: {}", e);
            }
            if let Err(e) = self.child.wait() {
                eprintln!("Failed to wait for backend process: {}", e);
            }
        }
    }

    fn start_backend() -> BackendProcess {
        // Create a temporary database
        let db_file = NamedTempFile::new().expect("Failed to create temp db file");
        let db_path = db_file.path().to_str().unwrap();
        let database_url = format!("sqlite://{}", db_path);

        println!("Building backend...");
        let build_status = Command::new("cargo")
            .arg("build")
            .status()
            .expect("Failed to execute cargo build");
        assert!(build_status.success(), "Backend build failed");

        println!("Starting backend...");
        let child = Command::new("cargo")
            .arg("run")
            .env("DATABASE_URL", &database_url)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start backend");

        println!("Waiting for backend to start...");
        for _ in 0..20 {
            match reqwest::blocking::get("http://localhost:8080") {
                Ok(res) if res.status().is_success() => {
                    println!("Backend started successfully!");
                    return BackendProcess { child, db_file };
                }
                _ => {
                    std::thread::sleep(Duration::from_secs(1));
                }
            }
        }

        panic!("Backend failed to start after 20 seconds.");
    }

    #[test]
    fn run_integration_tests() {
        let _backend_process = start_backend();

        println!("Verifying static file serving...");
        let res = reqwest::blocking::get("http://localhost:8080/").expect("Failed to get /");
        assert!(res.status().is_success(), "Failed to serve index.html");

        println!("Verifying UqService API...");
        let client = reqwest::blocking::Client::new();
        // Test Sync (Empty state)
        let res = client
            .post("http://localhost:8080/uq.v1.UqService/Sync")
            .header("Content-Type", "application/json")
            .body(r#"{ "sinceTimestampMs": "0" }"#)
            .send()
            .expect("Failed to call Sync");

        assert!(
            res.status().is_success(),
            "Sync request failed: {:?}",
            res.status()
        );
        let body = res.text().expect("Failed to read body");
        println!("Sync Response: {}", body);
        // Expect response with timestamp
        assert!(
            body.contains("serverTimestampMs"),
            "Response missing serverTimestampMs"
        );
    }
}
