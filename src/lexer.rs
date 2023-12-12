use std::fmt::Display;
use std::fs;

#[derive(Clone, Debug)]
pub enum Relop {
    EE,
    NE, 
    GT, 
    GE, 
    LT, 
    LE
}

#[derive(Clone, Debug)]
pub enum Token {
    // {{{
    Class,
    Else,
    False,
    Fi,
    If,
    In,
    Inherits,
    Isvoid,
    Let,
    Loop,
    Pool,
    Then,
    While,
    Case,
    Esac,
    New,
    Of,
    Not,
    True,
    Colon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RighParen,
    LeftBracket,
    RightBracket,
    Assignment,
    Relop(Relop),
    Id(usize),
    Literal(usize),
    // }}}
}

pub struct Lexer {
    filename: String,
    lexem_begin: usize,
    lookahead: usize,
    buffer: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn for_file(filename: &str) -> Lexer {
        Lexer {
            filename: String::from(filename),
            lexem_begin: 0,
            lookahead: 0,
            buffer: fs::read_to_string(filename).unwrap(),
            tokens: vec![],
        }
    }

    pub fn analyse(&mut self) {
        let mut new_token: Token;

        for letter in self.buffer.chars() {
            print!("{}", letter);
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // {{{
            Token::Class => write!(f, "<Class>"),
            Token::Else => write!(f, "<Else>"),
            Token::False => write!(f, "<False>"),
            Token::Fi => write!(f, "<Fi>"),
            Token::If => write!(f, "<If>"),
            Token::In => write!(f, "<In>"),
            Token::Inherits => write!(f, "<Inherits>"),
            Token::Isvoid => write!(f, "<Isvoid>"),
            Token::Let => write!(f, "<Let>"),
            Token::Loop => write!(f, "<Loop>"),
            Token::Pool => write!(f, "<Pool>"),
            Token::Then => write!(f, "<Then>"),
            Token::While => write!(f, "<While>"),
            Token::Case => write!(f, "<Case>"),
            Token::Esac => write!(f, "<Esac>"),
            Token::New => write!(f, "<New>"),
            Token::Of => write!(f, "<Of>"),
            Token::Not => write!(f, "<Not>"),
            Token::True => write!(f, "<True>"),
            Token::Colon => write!(f, "<Colon>"),
            Token::LeftBrace => write!(f, "<LeftBrace>"),
            Token::RightBrace => write!(f, "<RightBrace>"),
            Token::LeftParen => write!(f, "<LeftParen>"),
            Token::RighParen => write!(f, "<RighParen>"),
            Token::LeftBracket => write!(f, "<LeftBracket>"),
            Token::RightBracket => write!(f, "<RightBracket>"),
            Token::Assignment => write!(f, "<<->"),
            Token::Relop(op) => write!(f, "<{}>", op),
            Token::Id(num) => write!(f, "<Id, {}>", num),
            Token::Literal(num) => write!(f, "<Literal, {}>", num),
            // }}}
        }
    }
}

impl Display for Relop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relop::EE => write!(f, "="),
            Relop::NE => write!(f, "!="),
            Relop::GT => write!(f, ">"),
            Relop::GE => write!(f, ">="),
            Relop::LT => write!(f, "<"),
            Relop::LE => write!(f, "<="),
        }
    }
}
