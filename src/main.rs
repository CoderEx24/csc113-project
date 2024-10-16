mod lexer;
mod parser;
mod symbol_table;

use lexer::Lexer;
use parser::Parser;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        panic!("[ERROR] too much arguments");
    }

    let filename = &args[1];

    let lexer = Lexer::for_file(filename);
    let mut parser = Parser::new(lexer);

    parser.parse();
}
