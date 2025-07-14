#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_project_has_required_dependencies() {
        // Teste que falha: verificar se axum está disponível
        let output = Command::new("cargo")
            .args(&["check", "--quiet"])
            .output()
            .expect("Failed to execute cargo check");

        // Se não tivermos axum, este teste deve falhar
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("unresolved import"), 
                "Project should have all required dependencies. Missing: {}", stderr);
    }

    #[test]
    fn test_project_compiles_without_errors() {
        let output = Command::new("cargo")
            .args(&["build", "--quiet"])
            .output()
            .expect("Failed to execute cargo build");

        assert!(output.status.success(), 
                "Project should compile without errors. Exit code: {}", output.status);
    }
} 