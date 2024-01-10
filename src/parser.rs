use std::collections::HashMap;
use crate::lexer::{Lexer, Token};

enum Action {
    Reduce(u8),
    Shift(u8),
    Accept,
    Error,
}

struct Parser {
    lex: Lexer,
    stack: Vec<u8>,

}

impl Parser {
    fn new(lex: Lexer) -> Parser {
        Parser {
            lex,
            stack: vec![0],
        }
    }

    fn parse(&mut self) {

        let PARSE_TABLE = HashMap::from([
            (Token::Class, Action::Shift(1)),
            (Token::Id(_), Action::Shift(2)),
            (Token::LeftBrace, Action::Shift(3)),
            (Token::Inherits, Action::Shift(4)),
            (Token::RightBrace, Action::Shift(5)),

        ]);

        let REDUCED_STATES_COUNT = HashMap::from([
            (1, 2)
        ]);

        let lex = &mut self.lex;
        let tokens: Vec<Token> = lex.collect();

        for token in tokens {
            let action = PARSE_TABLE.get(&token).unwrap_or(&Action::Error);

            match action {
                Action::Reduce(state) => { 
                    println!("Reduce {}", state); 
                    for _ in 0..REDUCED_STATES_COUNT.get(self.stack.pop().unwrap()) {
                        self.stack.pop()
                    }
                    self.stack.push(state);
                }
                Action::Shift(state) => { 
                    println!("Shift {}", state); 
                    self.stack.push(state);
                }
                Action::Accept => { println!("Accept"); }
                Action::Error => { panic!("[SYNTAX ERROR]");
            }
        }

    }
}

