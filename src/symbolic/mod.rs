use std::option::Option;

pub enum Step {
    Run,
    Compile,
    Parse,
    Tokenize,
}

pub enum File {
    StdIo,
    File(String),
}

pub struct Args {
    pub step : Step,
    pub input : File,
    pub output : Option<File>,
}

pub enum FailCase {
    ParseargMultiFilename,
    ParseargInvalidOption,
}

impl Args {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Args, FailCase>{
        args.next();
        let mut retval = Args {
            step:Step::Run,
            input:File::StdIo,
            output:None,
        };
        while let Some(arg) = args.next() {
            match &arg as &str {
                "parse" => {
                    retval.step=Step::Run;
                    retval.output=Some(File::StdIo);
                }
                "tokenize" => {
                    retval.step=Step::Tokenize;
                    retval.output=Some(File::StdIo);
                }
                "compile" => {
                    retval.step=Step::Compile;
                    retval.output=Some(File::StdIo);
                }
                "run" => {
                }
                "--output" => {
                    retval.output=Some(File::File(args.next().expect("6")));
                }
                filename => {
                    if filename.starts_with("--") {
                        return Err(FailCase::ParseargInvalidOption);
                    }
                    retval.input = if filename == "-" {
                        File::StdIo
                    } else {
                        match retval.input { File::File(_) => {
                            return Err(FailCase::ParseargMultiFilename);
                        }, _ => ()}
                        File::File(String::from(filename))
                    }
                }
            }
        }
        Ok(retval)
    }
}
