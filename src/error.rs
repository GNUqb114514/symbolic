/// The error struct.
use std::error::Error;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct SymbolicError {
    kind: SymbolicErrorKind,
    description: String,
}

impl Error for SymbolicError {}

impl Display for SymbolicError {
    fn fmt(&self, f:&mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ERROR: {:?}\n{}", self.kind, self.description)
    }
}

impl From<SymbolicErrorKind> for SymbolicError {
    fn from(kind: SymbolicErrorKind) -> Self {
        SymbolicError {
            kind: kind,
            description: format!(""),
        }
    }
}

impl From<std::io::Error> for SymbolicError {
    fn from(source: std::io::Error) -> SymbolicError {
        SymbolicError {
            kind: source.kind().into(),
            description: format!("{}", source),
        }
    }
}

/// Generic error enum.
#[derive(Debug)]
pub enum SymbolicErrorKind {
    /// Can't parse command line arguments
    /// because there're too many file arguments.
    ParseargTooManyFiles,
    /// Can't parse command line arguments
    /// because there're invalid options.
    ParseargInvalidOption,
    /// File not found.
    FileNotFound,
    /// Permission denied.
    PermissionDenied,
    /// Operation interrupted.
    OperationInterrupted,
    /// Operation is unsupported.
    OperationUnsupported,
    /// Unexpected EOF.
    UnexceptedEOF,
    /// Unknown error.
    Unknown,
}

impl From<std::io::ErrorKind> for SymbolicErrorKind {
    fn from(value: std::io::ErrorKind) -> Self {
        match value {
            std::io::ErrorKind::NotFound => SymbolicErrorKind::FileNotFound,
            std::io::ErrorKind::PermissionDenied => SymbolicErrorKind::PermissionDenied,
            std::io::ErrorKind::Interrupted => SymbolicErrorKind::OperationInterrupted,
            std::io::ErrorKind::Unsupported => SymbolicErrorKind::OperationUnsupported,
            std::io::ErrorKind::UnexpectedEof => SymbolicErrorKind::UnexceptedEOF,
            _ => SymbolicErrorKind::Unknown,
        }
    }
}
