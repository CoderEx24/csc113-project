use std::fmt::Display;
use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
pub enum Relop {
    EE,
    LT, 
    LE
}

#[derive(Clone, Debug)]
pub enum MathOp {
    Plus,
    Minus,
    Multiply,
    Divide,
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
    Literal(String),
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

    fn advance(&mut self) {
        self.lexem_begin += 1;
        self.lookahead += 1;
    }

    pub fn analyse(&mut self) {
        
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

        let mut new_token: Option<Token>;
    
        let temp = self.buffer.clone();
        let bytes_slice = temp.as_bytes();

        while self.lookahead < bytes_slice.len() {
            new_token = None;

            let mut letter = bytes_slice[self.lexem_begin] as char;

            if letter.is_whitespace() {
                self.advance();
                continue;
            }
            
            if letter == '(' {
                let mut lookahead = bytes_slice[self.lookahead] as char;
                if lookahead == '*' {
                    loop {
                        self.advance();
                        letter = bytes_slice[self.lexem_begin] as char;
                        lookahead = bytes_slice[self.lookahead] as char;


                        if letter == '*' && lookahead == ')' {
                            self.lexem_begin += 2;
                            self.lookahead += 2;

                            break;
                        }
                    }

                    continue;
                }
            }
            else if letter == '-' {
                let mut lookahead = bytes_slice[self.lookahead] as char;
                if lookahead == '-' {
                    while lookahead != '\n' {
                        self.advance();
                        lookahead = bytes_slice[self.lookahead] as char;
                    }
                    self.advance();
                    continue;
                }
            }

            match letter {
                ':' => {new_token = Some(Token::Colon); self.advance()},
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
                    letter = bytes_slice[self.lookahead] as char;
                    match letter {
                        '-' => { new_token = Some(Token::Assignment); self.advance(); }
                        '=' => { new_token = Some(Token::Relop(Relop::LE)); self.advance(); }
                        _ => new_token = Some(Token::Relop(Relop::LT)),
                    }

                    self.advance();
                },
                '=' => {
                    letter = bytes_slice[self.lookahead] as char;
                    match letter {
                        '>' => { new_token = Some(Token::FatArrow); self.advance(); },
                        _ => new_token = Some(Token::Relop(Relop::EE)),
                    }
                    
                    self.advance();
                },
                'a'..='z' | 'A'..='Z' => {
                    loop {
                        letter = bytes_slice[self.lookahead] as char;
                        
                        if !letter.is_alphabetic() {
                            let lexem = &self.buffer[self.lexem_begin..self.lookahead];
                            let lexem = String::from(lexem);
                            new_token = Some(if keywords.contains_key(&lexem[..]) {
                                keywords.get(&lexem[..]).unwrap().clone()
                            }
                            else {
                                Token::Id(lexem)
                            });

                            self.lexem_begin = self.lookahead;
                            self.lookahead = self.lexem_begin + 1;
                            break;
                        }

                        self.lookahead += 1
                    }
                },
                '0'..='9' => {
                    loop {
                        letter = bytes_slice[self.lookahead] as char;
                        
                        if !letter.is_alphanumeric() {
                            let lexem = &self.buffer[self.lexem_begin..self.lookahead];
                            let lexem = String::from(lexem);
                            new_token = Some(Token::Literal(lexem));

                            self.lexem_begin = self.lookahead;
                            self.lookahead = self.lexem_begin + 1;
                            break;
                        }
                        else if !letter.is_numeric() {
                            while letter.is_alphanumeric() {
                                letter = bytes_slice[self.lookahead] as char;
                                self.lookahead += 1;
                            }

                            let lexem = &self.buffer[self.lexem_begin..self.lookahead - 1];
                            panic!("[ERROR] invalid id {}", lexem);

                        }

                        self.lookahead += 1
                    }

                },
                '\"' => {
                    loop {
                        letter = bytes_slice[self.lookahead] as char;
                        
                        if letter == '\"' {
                            let lexem = &self.buffer[self.lexem_begin..self.lookahead + 1];
                            let lexem = String::from(lexem);
                            new_token = Some(Token::Literal(lexem));

                            self.lexem_begin = self.lookahead + 1;
                            self.lookahead = self.lexem_begin + 1;
                            break;
                        }

                        self.lookahead += 1
                    }

                }
                _ => {
                    panic!("[ERROR] unrecgonized lexem beginning {}\nprobably not ASCII", letter);
                }
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
            Token::Colon => write!(f, "< : >"),
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
