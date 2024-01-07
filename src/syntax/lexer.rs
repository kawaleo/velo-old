use super::error::ERROR_INDICATOR;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Literal Types
    String,
    NumericLiteral,
    Identifier,
    Null,

    // Keywords
    Func,
    Let,
    If,
    Else,
    ElseIf,

    For,
    Loop,
    Break,
    Continue,
    Return,

    KwTrue,
    KwFalse,

    Import,
    Export,
    Struct,
    Enum,
    Type,

    // Grouping * Operators
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Not,
    Dollar,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    Comma,
    Semicolon,
    Dot,
    Colon,
    Arrow,
    Tilde,
    BitwiseOr,
    BitwiseAnd,
    ShiftLeft,
    ShiftRight,
    And,
    Or,

    Eq,
    EqEq,
    Ne,
    Lt,
    Gt,
    LtEq,
    GtEq,

    PlusEq,
    MinusEq,
    TimesEq,
    DivEq,

    EOF,
}

impl TokenType {
    pub fn to_string(token_type: TokenType) -> String {
        let string_repr = format!("{:#?}", token_type);
        format!("{}{}", &string_repr[..1].to_lowercase(), &string_repr[1..])
    }
}

pub struct KeywordMap {
    data: [(&'static str, TokenType); 18],
}

impl KeywordMap {
    const fn new() -> Self {
        Self {
            data: [
                ("let", TokenType::Let),
                ("func", TokenType::Func),
                ("if", TokenType::If),
                ("else", TokenType::Else),
                ("elif", TokenType::ElseIf),
                ("for", TokenType::For),
                ("loop", TokenType::Loop),
                ("break", TokenType::Break),
                ("continue", TokenType::Continue),
                ("return", TokenType::Return),
                ("true", TokenType::KwTrue),
                ("false", TokenType::KwFalse),
                ("import", TokenType::Import),
                ("export", TokenType::Export),
                ("struct", TokenType::Struct),
                ("enum", TokenType::Enum),
                ("type", TokenType::Type),
                ("null", TokenType::Null),
            ],
        }
    }
    pub fn get(&self, key: &str) -> Option<TokenType> {
        for (k, v) in &self.data {
            if *k == key {
                return Some(v.clone());
            }
        }
        None
    }
}

pub const KEYWORDS: KeywordMap = KeywordMap::new();

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line_num: usize,
    pub lexeme: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Bool,
    Int,
    Short,
    Large,
    Float,
    String,
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Void,
}

impl Type {
    pub fn from_string(string: String) -> Type {
        let type_str = &string;
        let type_result = match type_str.as_str() {
            "bool" => Some(Type::Bool),
            "int" => Some(Type::Int),
            "short" => Some(Type::Short),
            "large" => Some(Type::Large),
            "float" => Some(Type::Float),
            "string" => Some(Type::String),
            "array" => Some(Type::Array(Box::new(Type::Bool))),
            "tuple" => Some(Type::Tuple(vec![Type::Bool])),
            "void" => Some(Type::Void),
            _ => None,
        };

        if type_result.is_none() {
            let message = format!("{} Unknown type '{}'", ERROR_INDICATOR, type_str);
            println!("{}", message);
            // Todo: implement error handling kinda like parser
            std::process::exit(1);
        }

        type_result.unwrap()
    }
}

pub struct Lexer {
    pub source: Vec<char>,
    pub tokens: Vec<Token>,
    line: usize,
    start_pos: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Lexer {
            source: src.chars().collect(),
            tokens: Vec::new(),
            line: 1,
            start_pos: 0,
        }
    }

    fn get_string(&mut self) -> Token {
        let mut string = String::new();
        self.source.remove(0);

        while let Some(c) = self.source.get(0) {
            match c {
                '"' => {
                    self.source.remove(0);
                    return Token {
                        token_type: TokenType::String,
                        line_num: self.line,
                        lexeme: string,
                    };
                }
                _ => {
                    if *c == '\n' {
                        self.line += 1;
                    }
                    string.push(*c);
                    self.source.remove(0);
                }
            }
        }

        eprintln!("err: unclosed string");
        std::process::exit(1);
    }

    fn make_token(&mut self, c: char, tok_type: TokenType) -> Token {
        let c = c.to_string();

        self.source.remove(0);
        Token {
            token_type: tok_type,
            line_num: self.line,
            lexeme: c.to_string(),
        }
    }

    fn make_long_token(&mut self, s: &str, tok_type: TokenType) -> Token {
        let s = s.to_string();

        self.source.remove(0);
        self.source.remove(0);
        Token {
            token_type: tok_type,
            line_num: self.line,
            lexeme: s.to_string(),
        }
    }

    pub fn tokenize(&mut self) -> Self {
        let mut tokens = Vec::new();
        while !self.source.is_empty() {
            match self.source[0] {
                '(' => tokens.push(self.make_token(self.source[0], TokenType::LParen)),
                ')' => tokens.push(self.make_token(self.source[0], TokenType::RParen)),
                '[' => tokens.push(self.make_token(self.source[0], TokenType::LBracket)),
                ']' => tokens.push(self.make_token(self.source[0], TokenType::RBracket)),
                '{' => tokens.push(self.make_token(self.source[0], TokenType::LBrace)),
                '}' => tokens.push(self.make_token(self.source[0], TokenType::RBrace)),

                '+' => match self.source[1] {
                    '=' => tokens.push(self.make_long_token("+=", TokenType::PlusEq)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Add)),
                },
                '-' => match self.source[1] {
                    '=' => tokens.push(self.make_long_token("-=", TokenType::MinusEq)),
                    '>' => tokens.push(self.make_long_token("-=", TokenType::Arrow)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Sub)),
                },
                '*' => match self.source[1] {
                    '=' => tokens.push(self.make_long_token("*=", TokenType::TimesEq)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Mul)),
                },
                '/' => match self.source[1] {
                    '=' => tokens.push(self.make_long_token("/=", TokenType::DivEq)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Div)),
                },

                '=' => match self.source[1] {
                    '=' => tokens.push(self.make_long_token("==", TokenType::EqEq)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Eq)),
                },

                '!' => match self.source[1] {
                    '=' => tokens.push(self.make_long_token("!=", TokenType::Ne)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Not)),
                },

                '%' => tokens.push(self.make_token(self.source[0], TokenType::Mod)),
                '$' => tokens.push(self.make_token(self.source[0], TokenType::Dollar)),

                ',' => tokens.push(self.make_token(self.source[0], TokenType::Comma)),
                ';' => tokens.push(self.make_token(self.source[0], TokenType::Semicolon)),
                '.' => tokens.push(self.make_token(self.source[0], TokenType::Dot)),
                ':' => tokens.push(self.make_token(self.source[0], TokenType::Colon)),
                '~' => tokens.push(self.make_token(self.source[0], TokenType::Tilde)),

                '|' => match self.source[1] {
                    '|' => tokens.push(self.make_long_token("||", TokenType::Or)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::BitwiseOr)),
                },

                '&' => match self.source[1] {
                    '&' => tokens.push(self.make_long_token("&&", TokenType::And)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::BitwiseAnd)),
                },

                '<' => match self.source[1] {
                    '<' => tokens.push(self.make_long_token("<<", TokenType::ShiftLeft)),
                    '=' => tokens.push(self.make_long_token("<=", TokenType::LtEq)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Lt)),
                },

                '>' => match self.source[1] {
                    '>' => tokens.push(self.make_long_token(">>", TokenType::ShiftRight)),
                    '=' => tokens.push(self.make_long_token(">=", TokenType::GtEq)),
                    _ => tokens.push(self.make_token(self.source[0], TokenType::Gt)),
                },

                '"' => tokens.push(self.get_string()),
                _ => {
                    if self.source[0].is_ascii_whitespace() {
                        match self.source[0] {
                            '\n' => {
                                self.line += 1;
                                self.source.remove(0);
                            }
                            _ => {
                                self.source.remove(0);
                            }
                        }
                    } else if self.source[0].is_ascii_alphabetic() {
                        let mut ident = String::new();
                        while !self.source.is_empty() && self.source[0].is_ascii_alphabetic() {
                            ident.push(self.source.remove(0));
                        }
                        match KEYWORDS.get(&ident[..]) {
                            Some(token_type) => tokens.push(Token {
                                token_type,
                                line_num: self.line,
                                lexeme: ident,
                            }),
                            None => tokens.push(Token {
                                token_type: TokenType::Identifier,
                                line_num: self.line,
                                lexeme: ident,
                            }),
                        }
                    } else if self.source[0].is_digit(10) {
                        let mut num = String::new();
                        while !self.source.is_empty() && self.source[0].is_digit(10) {
                            num.push(self.source.remove(0));
                        }

                        tokens.push(Token {
                            token_type: TokenType::NumericLiteral,
                            line_num: self.line,
                            lexeme: num,
                        })
                    } else {
                        eprintln!(
                            "[err] unrecognized character found in source: {}",
                            self.source[0]
                        );
                        std::process::exit(1);
                    }
                }
            }
        }

        tokens.push(Token {
            token_type: TokenType::EOF,
            line_num: self.line,
            lexeme: "EOF".to_string(),
        });

        Self {
            source: vec![],
            tokens,
            line: self.line,
            start_pos: self.start_pos,
        }
    }
}
