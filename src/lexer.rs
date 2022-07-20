use std::ffi::c_void;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Eof,
    Let,
    Int,
    Float,
    Double,
    Long,
    String,
    Char,
    Bool,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Equal,
    DoubleEqual,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Identifier{identifier: String},
    Semicolon,
    Or,
    And,
    Not,
    NumericLiteral{value: i32},
    StringLiteral{value: String},
    LineComment,
    OpenComment,
    CloseComment,
    TypeSeparator,
    AdditionSymbol,
    SubtractionSymbol,
    MultiplicationSymbol,
    DivisionSymbol,
    ArrowReturn,
    SingleQuote,
    DoubleQuote,
    Pointer,
    DereferencePointer,
    Caster,
    Class,
    Struct,
    Colon,
    Break,
    Continue,
    Return,
    If,
    Else,
    For,
    Switch,
    Try,
    Catch,
    Import,
    Dot,
    This,
    BackSlash,
    Ternary,
    Unknown{debug: String},
    Uninitialized
}

pub enum LexerErrorType {
    InvalidCharacter,
}

pub(crate) struct Lexer {
    cursor: usize,
    vec_input: Vec<char>,
    pub tokens: Vec<TokenType>,
    errors: Vec<LexerErrorType>
}

impl Lexer {
    pub(crate) fn new() -> Lexer {
        Lexer {
            cursor: 0,
            vec_input: vec![],
            tokens: vec![],
            errors: vec![]
        }
    }
    pub fn analyze(&mut self, raw_input: &str) {

        raw_input.chars().for_each(|c| self.vec_input.push(c));

        let mut building_token: String = "".parse().unwrap();

        while self.cursor < self.vec_input.len() {
            building_token.push(self.vec_input[self.cursor]);
            building_token = building_token.trim().parse().unwrap();
            let temp_len = self.tokens.len();
            match building_token.as_str() {
                "let"=> self.tokens.push(TokenType::Let),
                "int" => self.tokens.push(TokenType::Int),
                "float" => self.tokens.push(TokenType::Float),
                "double" => self.tokens.push(TokenType::Double),
                "long" => self.tokens.push(TokenType::Long),
                "string" => self.tokens.push(TokenType::String),
                "char" => self.tokens.push(TokenType::Char),
                "bool" => self.tokens.push(TokenType::Bool),
                "(" => self.tokens.push(TokenType::LParen),
                ")" => self.tokens.push(TokenType::RParen),
                "[" => self.tokens.push(TokenType::LBracket),
                "]" => self.tokens.push(TokenType::RBracket),
                "{" => self.tokens.push(TokenType::LBrace),
                "}" => self.tokens.push(TokenType::RBrace),
                ";" => self.tokens.push(TokenType::Semicolon),
                "&&" => self.tokens.push(TokenType::And),
                _ => {
                    if self.cursor < self.vec_input.len() - 1 {
                        let mut temp_token_type: TokenType = TokenType::Uninitialized;
                        if " .?^'{[()]}+-/*!|;=".contains(self.vec_input[self.cursor + 1])
                            && building_token != " " && building_token != "" {
                            temp_token_type = TokenType::Identifier { identifier: building_token.to_string() };
                        }
                        if building_token == ">" && self.vec_input[self.cursor + 1] != '=' {
                            temp_token_type = TokenType::GreaterThan;
                        } else if building_token == ">=" {
                            temp_token_type = TokenType::GreaterThanEqual;
                        }
                        if building_token == "<" && self.vec_input[self.cursor + 1] != '=' {
                            temp_token_type = TokenType::LessThan;
                        } else if building_token == "<=" {
                            temp_token_type = TokenType::LessThanEqual;
                        }
                        if building_token == "!" && self.vec_input[self.cursor + 1] != '=' {
                            temp_token_type = TokenType::Not;
                        } else if building_token == "!=" {
                            temp_token_type = TokenType::NotEqual;
                        }
                        if building_token == "=" && self.vec_input[self.cursor + 1] != '=' {
                            temp_token_type = TokenType::Equal;
                        } else if building_token == "==" {
                            temp_token_type = TokenType::DoubleEqual;
                        }
                        if building_token == "|" && self.vec_input[self.cursor + 1] != '|' {
                            temp_token_type = TokenType::TypeSeparator;
                        } else if building_token == "||" {
                            temp_token_type = TokenType::Or;
                        }
                        if temp_token_type != TokenType::Uninitialized {
                            self.tokens.push(temp_token_type);
                        }
                    }
                }
            };
            if temp_len != self.tokens.len() {
                building_token = "".parse().unwrap();
            }
            self.cursor += 1;
        }
    }
}