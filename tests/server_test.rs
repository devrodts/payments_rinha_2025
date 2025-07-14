#[cfg(test)]
mod tests {
    use std::process::Command;

    #[tokio::test]
    async fn test_server_compiles_with_axum() {
        let output = Command::new("cargo")
            .args(&["check", "--quiet"])
            .output()
            .expect("Failed to execute cargo check");

        assert!(output.status.success(), 
                "Project should compile with axum. Exit code: {}", output.status);
    }

    #[tokio::test]
    async fn test_server_builds_successfully() {
        let output = Command::new("cargo")
            .args(&["build", "--quiet"])
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success(), 
                "Project should build successfully. Exit code: {}", output.status);
    }

    #[tokio::test]
    async fn test_server_has_health_endpoint() {
        let main_content = std::fs::read_to_string("src/main.rs")
            .expect("Failed to read src/main.rs");
        
        assert!(main_content.contains("async fn health"), 
                "main.rs should contain health endpoint function");
        assert!(main_content.contains("/health"), 
                "main.rs should contain /health route");
    }
} 