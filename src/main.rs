mod lexer;

use lexer::Lexer;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        panic!("[ERROR] too much arguments");
    }

    let filename = &args[1];

    let mut lexer = Lexer::for_file(filename);
    lexer.analyse();

    lexer.get_tokens().iter().for_each(|t| println!("{}", t));
}
