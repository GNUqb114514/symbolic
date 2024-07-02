use crate::error::SymbolicError;
use crate::namedrw::{NamedRead, NamedWrite};
use crate::error::SymbolicErrorKind;

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

impl Args {
    /// Parse arguments from a iterator.
    ///
    /// The argument format is:
    /// ```bash
    /// symbolic <OPTS> [<INPUT> [<OUTPUT>]]
    /// ```
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Args, SymbolicError> {
        args.next().ok_or(SymbolicError::from(SymbolicErrorKind::ParseargInvalidOption))?;
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
                        match &args.next().ok_or(SymbolicErrorKind::ParseargInvalidOption)? as &str {
                            "src" | "source" => IR::Src,
                            "tokens" => IR::TokenStream,
                            "ast" => IR::AST,
                            "bytecode" => IR::Bytecode,
                            &_ => return Err(SymbolicErrorKind::ParseargInvalidOption.into()),
                        }
                }
                "--to" => {
                    retval.to =
                        match &args.next().ok_or(SymbolicErrorKind::ParseargInvalidOption)? as &str {
                            "tokens" => IR::TokenStream,
                            "ast" => IR::AST,
                            "bytecode" => IR::Bytecode,
                            &_ => return Err(SymbolicErrorKind::ParseargInvalidOption.into()),
                        }
                }
                filename => {
                    if retval_to_associated {
                        // All 2 files are associated => FAIL
                        return Err(SymbolicErrorKind::ParseargTooManyFiles.into());
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
            return Err(SymbolicErrorKind::ParseargInvalidOption.into());
        }
        Ok(retval)
    }
}
