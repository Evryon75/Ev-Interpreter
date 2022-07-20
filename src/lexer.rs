#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumericLiteral { numeric_type: NumericLiteralType, value: String },
    StringLiteral { string_type: StringLiteralType, value: String },
    DeclarationKeyword { keyword: DeclarationKeywords },
    Eof,
    LParen, // (
    RParen, // )
    LBracket, // [
    RBracket, // ]
    LBrace, // {
    RBrace, // }
    Equal, // =
    DoubleEqual, // ==
    NotEqual, // !=
    GreaterThan, // >
    GreaterThanEqual, // >=
    LessThan, // <
    LessThanEqual, // <=
    Identifier { identifier: String },
    Semicolon, // ;
    Or, // ||
    And, // &&
    Not, // !
    LineComment, // //
    OpenComment, // />
    CloseComment, // </
    TypeSeparator, // |
    AdditionOp, // +
    SubtractionOp, // -
    MultiplicationOp, // *
    DivisionOp, // /
    ArrowReturn, // >>
    SingleQuote, // '
    DoubleQuote, // "
    Pointer, // ^
    Dereference, // &
    Caster, // as
    Colon, // :
    Break, // break
    Continue, // continue
    Return, // return
    If, // if
    Else, // else
    For, // for
    Switch, // switch
    Try, // try
    Catch, // catch
    Import, // import
    Dot, // dot
    This, // this
    BackSlash, // \
    BackSlashN, // \n
    Ternary, // ?
    Debug { debug: String }, //TODO Remove this when lexer is done
}
#[derive(Debug, PartialEq)]
pub enum DeclarationKeywords {
    Class,
    Struct,
    Let,
    Bool,
    Int,
    Float,
    Double,
    Long,
    String,
    Char,
}
#[derive(Debug, PartialEq)]
pub enum NumericLiteralType {
    Int,
    Float,
    Double,
    Long,
}
#[derive(Debug, PartialEq)]
pub enum StringLiteralType {
    String,
    Char,
}
#[derive(Debug, PartialEq)]
pub enum LexerErrorType {
    InvalidCharacter,
}

pub fn parse(input: String) -> Result<Vec<TokenType>, Vec<LexerErrorType>> {
    let tokens: Vec<TokenType> = Vec::new();
    let parse_errors: Vec<LexerErrorType> = Vec::new();



    return if parse_errors.len() > 0 { Err(parse_errors) } else { Ok(Token) }
}
impl Lexer {
    pub(crate) fn new(input: String) -> Lexer {
        Lexer {
            tokens: vec![], input
        }
    }
}