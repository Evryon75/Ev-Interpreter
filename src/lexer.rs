use std::ffi::c_void;
use std::string::String;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumericLiteral { numeric_type: NumericLiteralType, value: i32 },
    StringLiteral { string_type: StringLiteralType, value: String },
    Eof,
    Let,
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
enum DeclarationKeywords {
    Int,
    Float,
    Double,
    Long,
    String,
    Char,
}
enum NumericLiteralType {
    Int,
    Float,
    Double,
    Long,
}
enum StringLiteralType {
    String,
    Char,
}
pub enum LexerErrorType {
    InvalidCharacter,
}

pub(crate) struct Lexer {
    pub tokens: Vec<TokenType>,
}

impl Lexer {
}