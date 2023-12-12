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
    RightParen,
    LeftBracket,
    RightBracket,
    Assignment,
    Relop(Relop),
    Id(String),
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
            lookahead: 1,
            buffer: fs::read_to_string(filename).unwrap(),
            tokens: vec![],
        }
    }

    pub fn analyse(&mut self) {
        println!("Analysis of \n\n{}\n\n", self.buffer);
        let mut new_token: Option<Token>;
    
        let bytes_slice = self.buffer.as_bytes();

        while self.lookahead < bytes_slice.len() {
            new_token = None;

            let mut letter = bytes_slice[self.lexem_begin] as char;

            if letter.is_whitespace() {
                self.lexem_begin += 1;
                self.lookahead += 1;
                continue;
            }
            print!("{}", letter);

            // match one-letter puncutation
            let mut found_puncutation = true;
            match letter {
                ':' => new_token = Some(Token::Colon),
                '(' => new_token = Some(Token::LeftParen),
                ')' => new_token = Some(Token::RightParen),
                '{' => new_token = Some(Token::LeftBrace),
                '}' => new_token = Some(Token::RightBrace),
                '[' => new_token = Some(Token::LeftBracket),
                ']' => new_token = Some(Token::RightBracket),
                _ => {
                    found_puncutation = false;
                }
            }

            if found_puncutation {
                self.lexem_begin += 1;
                self.lookahead += 1;

                self.tokens.push(new_token.unwrap());
                continue;
            }

            loop {
                letter = bytes_slice[self.lookahead] as char;
                
                if !letter.is_alphabetic() {
                    let lexem = &self.buffer[self.lexem_begin..self.lookahead];
                    let lexem = String::from(lexem);
                    new_token = Some(Token::Id(lexem));

                    self.lexem_begin = self.lookahead;
                    self.lookahead = self.lexem_begin + 1;
                    break;
                }

                self.lookahead += 1
            }

            if new_token.is_some() {
                let temp = new_token.unwrap();
                self.tokens.push(temp);
            }
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
            Token::RightParen => write!(f, "<RightParen>"),
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
