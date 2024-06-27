use std::env;
use std::fs;
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
        symbolic::File::File(ref name) => format!("file {}", name),
    }, match args.output {
        None => String::from(""),
        Some(file) => match file {
            symbolic::File::StdIo => String::from(" to stdio"),
            symbolic::File::File(name1) => format!(" to file {}", name1),
        }
    });
    let filename = match args.input {
        symbolic::File::StdIo => String::from("-"),
        symbolic::File::File(name) => name,
    };
    let data = String::from_utf8(fs::read::<String>(filename).unwrap_or_else(|_err| {
        println!("Cannot read");
        process::abort();
    })).expect("Cannot convert to utf8");
    println!("Input file data:\n{}", data);
}
