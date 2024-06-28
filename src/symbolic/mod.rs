use std::io::Read;
use std::io::Write;

/// Enum to indicate IRs in the progress.
#[derive(PartialEq)]
pub enum IR {
    Output,
    Bytecode,
    AST,
    TokenStream,
    Src,
}

/// Struct of argumentss.
pub struct Args {
    pub from: IR,
    pub to: IR,
    pub input: Box<dyn Read>,
    pub output: Box<dyn Write>,
    pub optimize_config:OptConfig,
}

pub struct OptConfig {
    pub constant_fold: bool,
    pub remove_dead_code: bool,
}

pub enum SymbolicError {
    ParseargTooManyFiles,
    ParseargInvalidOption,
}

impl PartialOrd for IR {
    fn partial_cmp(&self, rhs:&Self) -> Option<std::cmp::Ordering> {
        let a_val = match self {IR::Output=>4, IR::Bytecode=>3, IR::AST=>2, IR::TokenStream=>1, IR::Src=>0};
        let b_val = match rhs {IR::Output=>4, IR::Bytecode=>3, IR::AST=>2, IR::TokenStream=>1, IR::Src=>0};
        a_val.partial_cmp(&b_val)
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
            input: Box::new(std::io::stdin()),
            output: Box::new(std::io::stdout()),
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
                    retval.from = match &args.next().ok_or(SymbolicError::ParseargInvalidOption)? as &str {
                        "src" | "source" => IR::Src,
                        "tokens" => IR::TokenStream,
                        "ast" => IR::AST,
                        "bytecode" => IR::Bytecode,
&_ => {return Err(SymbolicError::ParseargInvalidOption)},
                    }
                }
                "--to" => {
                    retval.to = match &args.next().ok_or(SymbolicError::ParseargInvalidOption)? as &str {
                        "tokens" => IR::TokenStream,
                        "ast" => IR::AST,
                        "bytecode" => IR::Bytecode,
&_ => {return Err(SymbolicError::ParseargInvalidOption)},
                    }
                }
                filename => {
                    if retval_to_associated {
                        // All 2 files are associated => FAIL
                        return Err(SymbolicError::ParseargTooManyFiles);
                    }
                    if retval_from_associated {
                    let associated_file : Box<dyn Write > = match filename {
                        "-" => Box::new(std::io::stdout()),
                        _ => {
                            Box::new(std::fs::File::create(filename).unwrap())
                        }
                    };
                        retval.output = associated_file;
                        retval_to_associated = true;
                    } else {
                     let associated_file : Box <dyn Read> = match filename {
                       "-" => Box::new(std::io::stdin()),
                        _ => {
                            Box::new(std::fs::File::open(filename).unwrap())
                        }};
                        retval.input = associated_file;
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
