use std::env;
use std::process;
pub mod symbolic;

fn main() {
    println!("Symbolic, the language that has no keywords");
    let args = symbolic::Args::from_args(env::args()).unwrap_or_else(|_err| {
        println!("Invalid args");
        process::exit(1);
    });

    println!("{} {}{}", match args.step {
        symbolic::Step::Run => "Running",
        symbolic::Step::Parse => "Parsing",
        symbolic::Step::Compile => "Compiling",
        symbolic::Step::Tokenize => "Tokenizing",
    }, match args.input {
        symbolic::File::StdIo => String::from("stdin"),
        symbolic::File::File(name) => format!("file {}", name),
    }, match args.output {
        None => String::from(""),
        Some(file) => match file {
            symbolic::File::StdIo => String::from(" to stdio"),
            symbolic::File::File(name) => format!(" to file {}", name),
        }
    });
}
