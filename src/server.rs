use rmcp::{
    ServerHandler,
    model::{ProtocolVersion, ServerCapabilities, ServerInfo},
    tool,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Stdio;
use thiserror::Error;
use tokio::fs;
use tokio::process::Command;
use tokio::time::{Duration, timeout};
use tracing::{debug, error, info, instrument, warn};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CabalCheckRequest {
    #[schemars(description = "Path to the Haskell project directory")]
    pub path: String,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CabalCheckResponse {
    #[schemars(description = "Whether a cabal file was found")]
    pub found: bool,
    #[schemars(description = "Message describing the result")]
    pub message: String,
    #[schemars(description = "Path to the cabal file if found")]
    pub cabal_file: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CompileCheckRequest {
    #[schemars(description = "Path to the Haskell project directory")]
    pub path: String,
    #[schemars(description = "Timeout in seconds for compilation check (default: 30)")]
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CompileCheckResponse {
    #[schemars(description = "Whether the compilation was successful")]
    pub success: bool,
    #[schemars(description = "Status message")]
    pub message: String,
    #[schemars(description = "Compilation output (stdout)")]
    pub output: String,
    #[schemars(description = "Compilation errors (stderr)")]
    pub errors: String,
    #[schemars(description = "Exit code from ghcid process")]
    pub exit_code: Option<i32>,
}

#[derive(Error, Debug)]
pub enum CabalCheckError {
    #[error("Path does not exist: {path}")]
    PathNotFound { path: String },
    #[error("Path is not a directory: {path}")]
    NotADirectory { path: String },
    #[error("Permission denied accessing path: {path}")]
    PermissionDenied { path: String },
    #[error("I/O error reading directory: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
    #[error("No cabal files found in directory: {path}")]
    NoCabalFile { path: String },
}

#[derive(Error, Debug)]
pub enum CompileCheckError {
    #[error("Path does not exist: {path}")]
    PathNotFound { path: String },
    #[error("No cabal file found in directory: {path}")]
    NoCabalFile { path: String },
    #[error("Compilation timed out after {timeout_secs} seconds")]
    Timeout { timeout_secs: u64 },
    #[error("I/O error during compilation check: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },
}

#[derive(Debug, Clone)]
pub struct GhcidServer;

impl GhcidServer {
    pub fn new() -> Self {
        info!("Creating new GhcidServer instance");
        Self
    }
}

#[tool(tool_box)]
impl GhcidServer {
    #[tool(description = "Check if a cabal file exists in the specified Haskell project path")]
    async fn check_cabal(&self, #[tool(aggr)] request: CabalCheckRequest) -> String {
        info!("Starting cabal file check");
        debug!(path = %request.path, "Checking path for cabal files");

        match self.check_cabal_internal(&request.path).await {
            Ok(response) => {
                info!(
                    found = response.found,
                    cabal_file = response.cabal_file.as_deref(),
                    "Cabal check completed successfully"
                );
                serde_json::to_string(&response).unwrap_or_else(|e| {
                    error!(error = %e, "Failed to serialize response");
                    response.message
                })
            }
            Err(e) => {
                error!(error = %e, "Cabal check failed");
                let response = CabalCheckResponse {
                    found: false,
                    message: e.to_string(),
                    cabal_file: None,
                };
                serde_json::to_string(&response).unwrap_or_else(|_| e.to_string())
            }
        }
    }

    #[instrument]
    async fn check_cabal_internal(
        &self,
        path: &str,
    ) -> Result<CabalCheckResponse, CabalCheckError> {
        let project_path = Path::new(path);
        debug!(path = %path, "Validating project path");

        // Check if the path exists
        if !project_path.exists() {
            warn!(path = %path, "Path does not exist");
            return Err(CabalCheckError::PathNotFound {
                path: path.to_string(),
            });
        }

        // Check if it's a directory
        if !project_path.is_dir() {
            warn!(path = %path, "Path is not a directory");
            return Err(CabalCheckError::NotADirectory {
                path: path.to_string(),
            });
        }

        debug!(path = %path, "Reading directory contents");

        // Look for .cabal files in the directory
        let mut entries = fs::read_dir(project_path).await.map_err(|e| {
            error!(path = %path, error = %e, "Failed to read directory");
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                CabalCheckError::PermissionDenied {
                    path: path.to_string(),
                }
            } else {
                CabalCheckError::IoError { source: e }
            }
        })?;

        let mut cabal_files = Vec::new();

        while let Some(entry) = entries.next_entry().await? {
            let file_name = entry.file_name();
            let file_name_str = file_name.to_string_lossy();

            debug!(file_name = %file_name_str, "Examining file");

            if file_name_str.ends_with(".cabal") {
                let cabal_path = entry.path().to_string_lossy().to_string();
                info!(cabal_file = %cabal_path, "Found cabal file");
                cabal_files.push(cabal_path);
            }
        }

        if let Some(cabal_file) = cabal_files.first() {
            info!(cabal_files_count = cabal_files.len(), "Cabal files found");
            if cabal_files.len() > 1 {
                warn!(
                    cabal_files_count = cabal_files.len(),
                    "Multiple cabal files found, using first one"
                );
            }

            Ok(CabalCheckResponse {
                found: true,
                message: "Cabal file is located".to_string(),
                cabal_file: Some(cabal_file.clone()),
            })
        } else {
            warn!(path = %path, "No cabal files found in directory");
            Err(CabalCheckError::NoCabalFile {
                path: path.to_string(),
            })
        }
    }

    #[tool(description = "Run ghcid compilation check on a Haskell project using 'cabal repl'")]
    async fn check_compilation(&self, #[tool(aggr)] request: CompileCheckRequest) -> String {
        info!("Starting compilation check with ghcid");
        debug!(path = %request.path, timeout = ?request.timeout_secs, "Running ghcid compilation check");

        match self
            .check_compilation_internal(&request.path, request.timeout_secs.unwrap_or(300))
            .await
        {
            Ok(response) => {
                info!(
                    success = response.success,
                    exit_code = response.exit_code,
                    "Compilation check completed"
                );
                serde_json::to_string(&response).unwrap_or_else(|e| {
                    error!(error = %e, "Failed to serialize compilation response");
                    response.message
                })
            }
            Err(e) => {
                error!(error = %e, "Compilation check failed");
                let response = CompileCheckResponse {
                    success: false,
                    message: e.to_string(),
                    output: String::new(),
                    errors: e.to_string(),
                    exit_code: None,
                };
                serde_json::to_string(&response).unwrap_or_else(|_| e.to_string())
            }
        }
    }

    #[instrument]
    async fn check_compilation_internal(
        &self,
        path: &str,
        timeout_secs: u64,
    ) -> Result<CompileCheckResponse, CompileCheckError> {
        let project_path = Path::new(path);
        debug!(path = %path, "Validating project path for compilation");

        // Check if the path exists
        if !project_path.exists() {
            warn!(path = %path, "Path does not exist");
            return Err(CompileCheckError::PathNotFound {
                path: path.to_string(),
            });
        }

        // Check if there's a cabal file
        let has_cabal = match self.check_cabal_internal(path).await {
            Ok(_) => true,
            Err(CabalCheckError::NoCabalFile { .. }) => {
                warn!(path = %path, "No cabal file found in project directory");
                return Err(CompileCheckError::NoCabalFile {
                    path: path.to_string(),
                });
            }
            Err(e) => {
                error!(path = %path, error = %e, "Error checking for cabal file");
                return Err(CompileCheckError::NoCabalFile {
                    path: path.to_string(),
                });
            }
        };

        debug!(path = %path, has_cabal = %has_cabal, "Project validation passed, preparing ghcid command");

        // Use ghcid directly - let system handle if not found
        info!(path = %path, "Using ghcid command directly");

        let mut command = Command::new("ghcid");
        command
            .arg("-c")
            .arg("cabal repl")
            .current_dir(project_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        info!(timeout_secs = %timeout_secs, "Starting ghcid process with system environment");
        info!("Command to execute: ghcid -c 'cabal repl'");

        debug!(
            "About to execute command with current_dir: {:?}",
            project_path
        );
        let timeout_duration = Duration::from_secs(timeout_secs);
        let result = timeout(timeout_duration, command.output()).await;

        match result {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code();
                let success = output.status.success();

                debug!(
                    exit_code = ?exit_code,
                    success = %success,
                    stdout_len = stdout.len(),
                    stderr_len = stderr.len(),
                    "ghcid process completed"
                );

                let message = if success {
                    "Compilation successful - no errors found".to_string()
                } else {
                    "Compilation failed - errors detected".to_string()
                };

                Ok(CompileCheckResponse {
                    success,
                    message,
                    output: stdout,
                    errors: stderr,
                    exit_code,
                })
            }
            Ok(Err(e)) => {
                error!(error = %e, "I/O error executing ghcid command");
                Err(CompileCheckError::IoError { source: e })
            }
            Err(_) => {
                warn!(timeout_secs = %timeout_secs, "ghcid process timed out");
                Err(CompileCheckError::Timeout { timeout_secs })
            }
        }
    }
}

#[tool(tool_box)]
impl ServerHandler for GhcidServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "ghcid-server".into(),
                version: "0.1.0".into(),
            },
            instructions: Some("A Haskell development server that checks for cabal files and provides compilation feedback.".into()),
        }
    }
}
