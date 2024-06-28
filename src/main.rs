use std::env;
use std::process;
pub mod symbolic;

fn main() {
    println!("Symbolic, the language that has no keywords");
    let mut args = symbolic::Args::from_args(env::args()).unwrap_or_else(|_err| {
        println!("Invalid args");
        process::exit(1);
    });

    println!("{}", match args.to {
        symbolic::IR::Output => "Running",
        symbolic::IR::AST => "Parsing",
        symbolic::IR::Bytecode => "Compiling",
        symbolic::IR::TokenStream => "Tokenizing",
        symbolic::IR::Src => {panic!("What's the fxxk")}
    });
    let mut data : String= Default::default();
    args.input.read_to_string(&mut data);
    println!("Input file data:\n{}", data);
}
