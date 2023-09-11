use config::ConfigError;
use thiserror::Error;

#[derive(Error, Debug)]
/// All applications errors are to be defined here.
pub enum AppError {
    /// For starter, to remove as code matures
    #[error("generic error: {0}")]
    Generic(String),
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error("video clip parsing error: {0}")]
    ParseVideoClipError(String),
    #[error("failed to find attribute {0} for video {1}")]
    MissingAttribute(String, String),
    #[error("failed to parse {0} str {1} for video {2}")]
    ParseAttributeError(String, String, String),
    #[error("unable to detect clip program format for video {0}")]
    UnknownClipProgram(String),
}
