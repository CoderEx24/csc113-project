use std::fmt::Display;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Relop {
    EE,
    LT, 
    LE
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MathOp {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    Comma,
    SemiColon,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    FatArrow,
    Dot,
    At,
    Assignment,
    Relop(Relop),
    MathOp(MathOp),
    Id(String),
    Type(String),
    Integer(i128),
    StringLiteral(String),
    EndOfInput,
    // }}}
}

pub struct Lexer {
    filename: String,
    lexem_begin: usize,
    lookahead: usize,
    lexem_begin_letter: char,
    lookahead_letter: char,
    buffer: String,
    line_number: usize,
    finished: bool,
}

impl Lexer {
    pub fn for_file(filename: &str) -> Lexer {
        let buf = fs::read_to_string(filename).unwrap();
        let first_letter = buf.chars().nth(0).unwrap();
        let second_letter = buf.chars().nth(1).unwrap();

        Lexer {
            filename: String::from(filename),
            lexem_begin: 0,
            lookahead: 1,
            line_number: 1,
            buffer: buf,
            lexem_begin_letter: first_letter,
            lookahead_letter: second_letter,
            finished: false,
        }
    }

    fn advance(&mut self) {
        self.lexem_begin += 1;
        self.lexem_begin_letter = self.buffer.chars().nth(self.lexem_begin).unwrap_or('\0');

        if self.lexem_begin_letter == '\n' {
            self.line_number += 1;
        }

        self.advance_lookahead();
    }

    fn advance_lookahead(&mut self) {
        self.lookahead += 1;
        self.lookahead_letter = self.buffer.chars().nth(self.lookahead).unwrap_or('\0');
    }

    fn start_new_lexem(&mut self) {
        self.lexem_begin = self.lookahead;
        self.lookahead = self.lexem_begin + 1;

        self.lexem_begin_letter = self.buffer.chars().nth(self.lexem_begin).unwrap_or('\0');
        self.lookahead_letter = self.buffer.chars().nth(self.lookahead).unwrap_or('\0');
    }

}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        let keywords = HashMap::from([
            // {{{
            ("class", Token::Class),
            ("else", Token::Else),
            ("false", Token::False),
            ("fi", Token::Fi),
            ("if", Token::If),
            ("in", Token::In),
            ("inherits", Token::Inherits),
            ("isvoid", Token::Isvoid),
            ("let", Token::Let),
            ("loop", Token::Loop),
            ("pool", Token::Pool),
            ("then", Token::Then),
            ("while", Token::While),
            ("case", Token::Case),
            ("esac", Token::Esac),
            ("new", Token::New),
            ("of", Token::Of),
            ("not", Token::Not),
            ("true", Token::True),
            // }}}
        ]);

        let new_token;

        /*println!("Processing letters: {} = {:x}, {} = {:x}", 
            self.lexem_begin_letter,
            self.lexem_begin_letter as u16, 
            self.lookahead_letter,
            self.lookahead_letter as u16);*/

// {{{ Processing Comments
        while self.lexem_begin_letter == '(' ||
                self.lexem_begin_letter == '-' ||
                self.lexem_begin_letter.is_whitespace() {

            while self.lexem_begin_letter.is_whitespace() {
                self.advance();
            }

            if self.lexem_begin_letter == '(' && self.lookahead_letter == '*' {
                self.advance();
                self.advance();

                while self.lexem_begin_letter != '*' || self.lookahead_letter != ')' {
                    //println!("Ignoring Comment Content: {}", self.lexem_begin_letter);
                    self.advance();
                }

                self.advance();
                self.advance();
            }

            else if self.lexem_begin_letter == '-' && self.lookahead_letter == '-' {
                while self.lexem_begin_letter != '\n' {
                    //println!("Ignoring Comment Content: {}", self.lexem_begin_letter);
                    self.advance();
                }

                self.advance();
            }
            else {
                break;
            }

            while self.lexem_begin_letter.is_whitespace() {
                self.advance();
            }
        }
        // }}}
        
        if self.finished {
            return None;
        }
            
        if self.lexem_begin_letter == '\0' {
            self.finished = true;
            return Some(Token::EndOfInput);
        }
        

        match self.lexem_begin_letter {
            ':' => {new_token = Some(Token::Colon); self.advance()},
            ',' => {new_token = Some(Token::Comma); self.advance()},
            '(' => {new_token = Some(Token::LeftParen); self.advance()},
            ')' => {new_token = Some(Token::RightParen); self.advance()},
            '{' => {new_token = Some(Token::LeftBrace); self.advance()},
            '}' => {new_token = Some(Token::RightBrace); self.advance()},
            '[' => {new_token = Some(Token::LeftBracket); self.advance()},
            ']' => {new_token = Some(Token::RightBracket); self.advance()},
            ';' => {new_token = Some(Token::SemiColon); self.advance()},
            '~' => {new_token = Some(Token::Not); self.advance()},
            '+' => {new_token = Some(Token::MathOp(MathOp::Plus)); self.advance()},
            '-' => {new_token = Some(Token::MathOp(MathOp::Minus)); self.advance()},
            '*' => {new_token = Some(Token::MathOp(MathOp::Multiply)); self.advance()},
            '/' => {new_token = Some(Token::MathOp(MathOp::Divide)); self.advance()},
            '.' => {new_token = Some(Token::Dot); self.advance()},
            '@' => {new_token = Some(Token::At); self.advance()},
            '<' => {
                match self.lookahead_letter {
                    '-' => { new_token = Some(Token::Assignment); self.advance(); }
                    '=' => { new_token = Some(Token::Relop(Relop::LE)); self.advance(); } _ => new_token = Some(Token::Relop(Relop::LT)),
                }

                self.advance();
            },
            '=' => {
                match self.lookahead_letter {
                    '>' => { new_token = Some(Token::FatArrow); self.advance(); },
                    _ => new_token = Some(Token::Relop(Relop::EE)),
                }
                
                self.advance();
            },
            'a'..='z' | 'A'..='Z' | '_' => {
                loop {
                    if !self.lookahead_letter.is_alphanumeric() &&
                            self.lookahead_letter != '_' {
                        let lexem = &self.buffer[self.lexem_begin..self.lookahead];
                        let lexem = String::from(lexem); 

                        // TODO: This is stupid!
                        let type_token = Token::Type(lexem.clone());
                        let id_token = Token::Id(lexem.clone());

                        new_token = keywords.get(&lexem[..]).or_else(
                            || if lexem.chars().nth(0).unwrap().is_ascii_uppercase() { 
                                Some(&type_token) 
                            } else { 
                                Some(&id_token) 
                            }
                        ).map(|o| o.to_owned());

                        self.start_new_lexem();
                        break;
                    }

                    self.advance_lookahead();
                }
            },
            '0'..='9' => {
                loop {
                    if !self.lookahead_letter.is_alphanumeric() {
                        let lexem = &self.buffer[self.lexem_begin..self.lookahead];
                        let lexem = String::from(lexem);
                        let lexem = i128::from_str_radix(&lexem[..], 10)
                            .expect(format!("[LEXEICAL ERROR] failed to convert {} into integer", lexem).as_str());
                        new_token = Some(Token::Integer(lexem));


                        self.start_new_lexem();
                        break;
                    }
                    else if !self.lookahead_letter.is_numeric() {
                        while self.lookahead_letter.is_alphanumeric() {
                            self.advance_lookahead();
                        }

                        let lexem = &self.buffer[self.lexem_begin..self.lookahead];
                        panic!("[LEXICAL ERROR] {}:{} |  invalid id {}, ids can't start with a number", 
                                self.filename,
                                self.line_number,
                                lexem);

                    }
                    self.advance_lookahead();
                }

            },
            '\"' => {
                loop {
                    if self.lookahead_letter == '\"' {
                        let lexem = &self.buffer[self.lexem_begin..self.lookahead + 1];
                        let lexem = String::from(lexem);
                        new_token = Some(Token::StringLiteral(lexem));

                        self.start_new_lexem();
                        self.advance();
                        break;
                    }

                    self.advance_lookahead();
                }

            }
            _ => {
                panic!("[LEXICAL ERROR] {}:{} | unrecgonized lexem beginning ({}) probably not ASCII", 
                    self.filename,
                    self.line_number,
                    self.lexem_begin_letter);
            }
        }

        new_token
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
            Token::Colon => write!(f, "< : >"),
            Token::Comma => write!(f, "< , >"),
            Token::SemiColon => write!(f, "< ; >"),
            Token::LeftBrace => write!(f, "< {{ >"),
            Token::RightBrace => write!(f, "< }} >"),
            Token::LeftParen => write!(f, "< ( >"),
            Token::RightParen => write!(f, "< ) >"),
            Token::LeftBracket => write!(f, "< [ >"),
            Token::RightBracket => write!(f, "< ] >"),
            Token::FatArrow => write!(f, "< => >"),
            Token::Dot => write!(f, "< . >"),
            Token::At => write!(f, "< @ >"),
            Token::Assignment => write!(f, "< <- >"),
            Token::Relop(op) => write!(f, "< {} >", op),
            Token::MathOp(op) => write!(f, "< {} >", op),
            Token::Id(s) => write!(f, "<Id, {}>", s),
            Token::Type(s) => write!(f, "<Type, {}>", s),
            Token::Integer(i) => write!(f, "<Integer, {}>", i),
            Token::StringLiteral(s) => write!(f, "<StringLiteral, {}>", s),
            Token::EndOfInput => write!(f, "< $ >"),
            // }}}
        }
    }
}

impl Display for Relop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relop::EE => write!(f, "="),
            Relop::LT => write!(f, "<"),
            Relop::LE => write!(f, "<="),
        }
    }
}

impl Display for MathOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathOp::Plus => write!(f, "+"),
            MathOp::Minus => write!(f, "-"),
            MathOp::Multiply => write!(f, "*"),
            MathOp::Divide => write!(f, "/"),
        }
    }
}


