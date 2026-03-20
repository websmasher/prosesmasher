use core::fmt;

#[derive(Debug, Clone)]
pub enum ReadError {
    NotFound(String),
    PermissionDenied(String),
    Io(String),
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::PermissionDenied(msg) => write!(f, "permission denied: {msg}"),
            Self::Io(msg) => write!(f, "io error: {msg}"),
        }
    }
}

impl std::error::Error for ReadError {}

#[derive(Debug, Clone)]
pub enum ParseError {
    InvalidMarkdown(String),
    SegmentationFailed(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMarkdown(msg) => write!(f, "invalid markdown: {msg}"),
            Self::SegmentationFailed(msg) => write!(f, "segmentation failed: {msg}"),
        }
    }
}

impl std::error::Error for ParseError {}

#[derive(Debug, Clone)]
pub enum ConfigError {
    NotFound(String),
    InvalidJson(String),
    ValidationFailed(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::InvalidJson(msg) => write!(f, "invalid json: {msg}"),
            Self::ValidationFailed(msg) => write!(f, "validation failed: {msg}"),
        }
    }
}

impl std::error::Error for ConfigError {}
