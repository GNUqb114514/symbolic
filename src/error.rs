/// The error struct.

/// Generic error enum.
pub enum SymbolicError {
    /// Can't parse command line arguments
    /// because there're too many file arguments.
    ParseargTooManyFiles,
    /// Can't parse command line arguments
    /// because there're invalid options.
    ParseargInvalidOption,
}

impl From<std::io::Error> for SymbolicError {
    fn from(value: std::io::Error) -> Self {
        todo!()
    }
}
