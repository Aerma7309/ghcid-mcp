use rmcp::{
    ServerHandler,
    model::{ProtocolVersion, ServerCapabilities, ServerInfo},
    tool,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;
use tokio::fs;
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
