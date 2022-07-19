use std::ffi::c_void;

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
    NumericLiteral,
    StringLiteral,
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
    PreIncrement,
    PreDecrement,
    PostIncrement,
    PostDecrement,
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

        let mut building_token = "";

        while self.cursor < self.vec_input.len() {
            building_token += self.vec_input[self.cursor];
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
                _ => {}
            });
            self.cursor += 1;
        }
    }
}