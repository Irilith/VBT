use std::io;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum FileError {
    #[error("Failed to create parent directory for {path}: {source}")]
    CreateDir {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("Failed to create {path}: {source}")]
    CreateFile {
        path: String,
        #[source]
        source: io::Error,
    },
}
