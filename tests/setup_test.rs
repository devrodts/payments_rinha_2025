#[cfg(test)]
mod tests {
    use std::process::Command;
    use rinha::modules::config::Config;

    #[test]
    fn test_project_has_required_dependencies() {
        let output = Command::new("cargo")
            .args(&["check", "--quiet"])
            .output()
            .expect("Failed to execute cargo check");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("unresolved import"), 
                "Project should have all required dependencies. Missing: {}",
                stderr);
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

    #[tokio::test]
    async fn test_t1_1_project_compiles() {
        // T1.1: Setup inicial do projeto Rust
        // Critério: Projeto deve compilar sem erros
        
        // RED PHASE: Este teste deve falhar inicialmente
        // pois vamos verificar se a estrutura básica está correta
        
        let config = Config::new();
        
            // Verificar se a configuração básica funciona
    let server_addr = config.server_addr();
    assert!(server_addr.to_string().len() > 0, "Server address should not be empty");
        
        // Verificar se o projeto pode ser importado
        assert!(std::path::Path::new("src/main.rs").exists(), "main.rs should exist");
        assert!(std::path::Path::new("src/lib.rs").exists(), "lib.rs should exist");
        assert!(std::path::Path::new("Cargo.toml").exists(), "Cargo.toml should exist");
        
        // Verificar se as dependências essenciais estão configuradas
        let cargo_content = std::fs::read_to_string("Cargo.toml").expect("Should read Cargo.toml");
        assert!(cargo_content.contains("axum"), "Cargo.toml should contain axum dependency");
        assert!(cargo_content.contains("tokio"), "Cargo.toml should contain tokio dependency");
        assert!(cargo_content.contains("serde"), "Cargo.toml should contain serde dependency");
        
        // Verificar se a estrutura de módulos está correta
        assert!(std::path::Path::new("src/modules").exists(), "modules directory should exist");
        assert!(std::path::Path::new("src/modules/config").exists(), "config module should exist");
        assert!(std::path::Path::new("src/modules/payment").exists(), "payment module should exist");
        
        // Verificar se o projeto compila
        let output = std::process::Command::new("cargo")
            .args(["check", "--quiet"])
            .output()
            .expect("Should run cargo check");
        
        assert!(
            output.status.success(),
            "Project should compile without errors. Stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[tokio::test]
    async fn test_t1_1_specific_issues() {
        // T1.1: Verificar problemas específicos no setup atual
        
        // Verificar se há warnings de compilação
        let output = std::process::Command::new("cargo")
            .args(["check", "--message-format=json"])
            .output()
            .expect("Should run cargo check with json output");
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        let warnings: Vec<&str> = output_str.lines()
            .filter(|line| line.contains("\"level\":\"warning\""))
            .collect();
        
        // Se há warnings, vamos falhar para documentar
        if !warnings.is_empty() {
            panic!("Found {} warnings in compilation: {:?}", warnings.len(), warnings);
        }
        
        // Verificar se o servidor pode ser iniciado (sem executar)
        let main_content = std::fs::read_to_string("src/main.rs").expect("Should read main.rs");
        assert!(main_content.contains("#[tokio::main]"), "main.rs should have tokio::main");
        assert!(main_content.contains("Router::new()"), "main.rs should have Router setup");
        assert!(main_content.contains("/payments"), "main.rs should have payments route");
        
        // Verificar se lib.rs expõe os módulos corretamente
        let lib_content = std::fs::read_to_string("src/lib.rs").expect("Should read lib.rs");
        assert!(lib_content.contains("pub mod modules"), "lib.rs should expose modules");
        
        // Verificar se modules/mod.rs existe e está correto
        let modules_mod_path = "src/modules/mod.rs";
        assert!(std::path::Path::new(modules_mod_path).exists(), "modules/mod.rs should exist");
        
        let modules_mod_content = std::fs::read_to_string(modules_mod_path).expect("Should read modules/mod.rs");
        assert!(modules_mod_content.contains("pub mod config"), "modules/mod.rs should expose config");
        assert!(modules_mod_content.contains("pub mod payment"), "modules/mod.rs should expose payment");
    }
} 