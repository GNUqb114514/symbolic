use std::env;
use std::io::Read;
use std::process;
pub mod symbolic;

fn main() {
    println!("Symbolic, the language that has no keywords");
    let mut args = symbolic::Args::from_args(env::args()).unwrap_or_else(|_err| {
        println!("Invalid args");
        process::exit(1);
    });

    println!(
        "{} {}...",
        match args.to {
            symbolic::IR::Output => "Running",
            symbolic::IR::AST => "Parsing",
            symbolic::IR::Bytecode => "Compiling",
            symbolic::IR::TokenStream => "Tokenizing",
            symbolic::IR::Src => {
                panic!("What's the fxxk")
            }
        },
        args.input.name,
    );
    let mut data = Default::default();
    args.input.read_to_string(&mut data).unwrap_or_else(|err| {
        println!("Error reading file: {}", err);
        process::exit(1);
    });
    println!("Read:\n{}", data)
}
