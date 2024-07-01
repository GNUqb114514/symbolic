use std::io::Read;
use std::io::Write;

/// Enum to indicate IRs in the progress.
#[derive(PartialEq)]
pub enum IR {
    /// Runned code (can't be `from`).
    Output,
    /// Compiled bytecode.
    Bytecode,
    /// Parsed AST.
    AST,
    /// Tokenized token stream.
    TokenStream,
    /// Source code (can't be `to`)
    Src,
}

/// Struct of argumentss.
pub struct Args {
    /// The status we has.
    pub from: IR,
    /// The status we're going to.
    pub to: IR,
    /// The input file.
    pub input: NamedRead,
    /// The output file.
    pub output: NamedWrite,
    /// The optimize config.
    pub optimize_config: OptConfig,
}

/// Optimization config.
pub struct OptConfig {
    /// Pre-calculate values of constant expressions.
    ///
    /// e.g.
    /// ```
    /// a = 114 + 514
    /// ```
    /// will be:
    /// ```
    /// PUSH_CONST 628 // Folded from '114 + 514'
    /// STORE_NAME a
    /// ```
    pub constant_fold: bool,
    /// Remove unreachable code (or 'dead code').
    ///
    /// e.g.
    /// ```
    /// <114
    /// a = 514
    /// ```
    /// will be:
    /// ```
    /// PUSH_CONST 114
    /// POP_FRAME
    /// // PUSH_CONST 512 // This line and above are dead codes
    ///                   // and are going to be removed.
    /// // POP_NAME a
    /// ```
    pub remove_dead_code: bool,
}

/// Generic error type.
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

impl PartialOrd for IR {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        let a_val = match self {
            IR::Output => 4,
            IR::Bytecode => 3,
            IR::AST => 2,
            IR::TokenStream => 1,
            IR::Src => 0,
        };
        let b_val = match rhs {
            IR::Output => 4,
            IR::Bytecode => 3,
            IR::AST => 2,
            IR::TokenStream => 1,
            IR::Src => 0,
        };
        a_val.partial_cmp(&b_val)
    }
}

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
    fn new(filename: &str) -> std::io::Result<Self> {
        Ok(NamedRead {
            name: filename.to_owned(),
            file: Box::new(std::fs::File::open(filename)?),
        })
    }
    /// Create from stdin.
    ///
    /// The filename is `-`.
    fn stdin() -> Self {
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
    fn new(filename: &str) -> std::io::Result<Self> {
        Ok(NamedWrite {
            name: filename.to_owned(),
            file: Box::new(std::fs::File::create(filename)?),
        })
    }
    /// Create from stdout.
    ///
    /// The filename is `-`.
    fn stdout() -> Self {
        NamedWrite {
            name: "-".to_owned(),
            file: Box::new(std::io::stdout()),
        }
    }
}

impl Args {
    /// Parse arguments from a iterator.
    ///
    /// The argument format is:
    /// ```bash
    /// symbolic <OPTS> [<INPUT> [<OUTPUT>]]
    /// ```
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Args, SymbolicError> {
        args.next().ok_or(SymbolicError::ParseargInvalidOption)?;
        let mut retval = Args {
            from: IR::Src,
            to: IR::Output,
            input: NamedRead::stdin(),
            output: NamedWrite::stdout(),
            optimize_config: OptConfig {
                constant_fold: false,
                remove_dead_code: false,
            },
        };
        let mut retval_to_associated = false;
        let mut retval_from_associated = false;
        while let Some(arg) = args.next() {
            match &arg as &str {
                "--from" => {
                    retval.from =
                        match &args.next().ok_or(SymbolicError::ParseargInvalidOption)? as &str {
                            "src" | "source" => IR::Src,
                            "tokens" => IR::TokenStream,
                            "ast" => IR::AST,
                            "bytecode" => IR::Bytecode,
                            &_ => return Err(SymbolicError::ParseargInvalidOption),
                        }
                }
                "--to" => {
                    retval.to =
                        match &args.next().ok_or(SymbolicError::ParseargInvalidOption)? as &str {
                            "tokens" => IR::TokenStream,
                            "ast" => IR::AST,
                            "bytecode" => IR::Bytecode,
                            &_ => return Err(SymbolicError::ParseargInvalidOption),
                        }
                }
                filename => {
                    if retval_to_associated {
                        // All 2 files are associated => FAIL
                        return Err(SymbolicError::ParseargTooManyFiles);
                    }
                    if retval_from_associated {
                        // Only from associated -> ASSOCIATE TO
                        retval.output = match filename {
                            "-" => NamedWrite::stdout(),
                            filename => NamedWrite::new(filename)?,
                        };
                        retval_to_associated = true;
                    } else {
                        // other -> ASSOCIATE FROM
                        retval.input = match filename {
                            "-" => NamedRead::stdin(),
                            filename => NamedRead::new(filename)?,
                        };
                        retval_from_associated = true;
                    }
                }
            }
        }
        if retval.from > retval.to {
            return Err(SymbolicError::ParseargInvalidOption);
        }
        Ok(retval)
    }
}
