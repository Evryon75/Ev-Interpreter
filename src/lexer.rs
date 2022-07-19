use std::ffi::c_void;
use std::string::String;

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
    Unknown
}

pub enum LexerErrorType {
    InvalidCharacter,
}

pub(crate) struct Lexer {
    cursor: usize,
    vec_input: Vec<char>,
    tokens: Vec<TokenType>,
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

        let mut building_token: String = "".to_string();

        while self.cursor < self.vec_input.len() {
            building_token.push(self.vec_input[self.cursor]);
            let temp_len = self.vec_input.len();
            self.tokens.push(match building_token {
                "let" => TokenType::Let,
                "int" => TokenType::Int,
                "float" => TokenType::Float,
                "double" => TokenType::Double,
                "long" => TokenType::Long,
                "string" => TokenType::String,
                "char" => TokenType::Char,
                "bool" => TokenType::Bool,
                "(" => TokenType::LParen,
                ")" => TokenType::RParen,
                "[" => TokenType::LBracket,
                "]" => TokenType::RBracket,
                "{" => TokenType::LBrace,
                "}" => TokenType::RBrace,
                ";" => TokenType::Semicolon,
                "&&" => TokenType::And,
                _ => {
                    if " .?^'{[(+-/*!|;=".contains(self.vec_input[self.cursor + 1]) {
                        TokenType::Identifier{ identifier: building_token.to_string() };
                    }
                    if building_token == ">" && self.vec_input[self.cursor + 1] != '=' {
                        TokenType::GreaterThan;
                    } else if building_token == ">=" {
                        TokenType::GreaterThanEqual;
                    }
                    if building_token == "<" && self.vec_input[self.cursor + 1] != '=' {
                        TokenType::LessThan;
                    } else if building_token == "<=" {
                        TokenType::LessThanEqual;
                    }
                    if building_token == "!" && self.vec_input[self.cursor + 1] != '=' {
                        TokenType::Not;
                    } else if building_token == "!=" {
                        TokenType::NotEqual;
                    }
                    if building_token == "=" && self.vec_input[self.cursor + 1] != '=' {
                        TokenType::Equal;
                    } else if building_token == "==" {
                        TokenType::DoubleEqual;
                    }
                    if building_token == "|" && self.vec_input[self.cursor + 1] != '|' {
                        TokenType::TypeSeparator;
                    } else if building_token == "||" {
                        TokenType::Or;
                    }
                    TokenType::Unknown
                }
            });
            if temp_len != self.vec_input.len() {
                building_token = "";
            }
            self.cursor += 1;
        }
    }
}