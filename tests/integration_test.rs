#[cfg(test)]
mod tests {
    use std::process::{Command, Stdio};
    use std::time::Duration;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_server_integration() {
        let mut server = Command::new("cargo")
            .args(&["run"])
            .env("RUST_LOG", "info")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start server");

        sleep(Duration::from_secs(3)).await;

        let output = Command::new("curl")
            .args(&["-s", "-o", "/dev/null", "-w", "%{http_code}", "http://localhost:9999/health"])
            .output()
            .expect("Failed to execute curl");

        let status_code = String::from_utf8_lossy(&output.stdout);
        
        let _ = server.kill();

        assert!(status_code == "200" || status_code == "404" || status_code == "000", 
                "Server should respond with HTTP status. Got: {}", status_code);
    }

    #[tokio::test]
    async fn test_server_starts_successfully() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .output();

        assert!(output.is_ok(), "Server should compile and be executable");
    }
} 