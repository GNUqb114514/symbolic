/// named `Read` and `Write` wrappers.

use std::io::{Read, Write};

/// Wrapper of `Read` with name.
pub struct NamedRead {
    /// The name of the file.
    pub name: String,
    /// The `Read` instance that we're wrapping.
    file: Box<dyn Read>,
}

impl Read for NamedRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.file.read(buf)
    }
}

impl NamedRead {
    /// Create from filename.
    ///
    /// ```
    /// assert!(NamedRead::new("exist.txt")?.name, "exist.txt")
    /// ```
    pub fn new(filename: &str) -> std::io::Result<Self> {
        Ok(NamedRead {
            name: filename.to_owned(),
            file: Box::new(std::fs::File::open(filename)?),
        })
    }
    /// Create from stdin.
    ///
    /// The filename is `-`.
    pub fn stdin() -> Self {
        NamedRead {
            name: "-".to_owned(),
            file: Box::new(std::io::stdin()),
        }
    }
}

/// Wrapper of `Write` with name.
pub struct NamedWrite {
    /// The name of the file.
    pub name: String,
    /// The `Write` instance that we're wrapping.
    file: Box<dyn Write>,
}

impl Write for NamedWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.file.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}

impl NamedWrite {
    /// Create from filename.
    ///
    /// ```
    /// assert!(NamedRead::new("exist.txt")?.name, "exist.txt")
    /// ```
    pub fn new(filename: &str) -> std::io::Result<Self> {
        Ok(NamedWrite {
            name: filename.to_owned(),
            file: Box::new(std::fs::File::create(filename)?),
        })
    }
    /// Create from stdout.
    ///
    /// The filename is `-`.
    pub fn stdout() -> Self {
        NamedWrite {
            name: "-".to_owned(),
            file: Box::new(std::io::stdout()),
        }
    }
}
