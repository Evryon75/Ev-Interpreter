use std::any::Any;
use crate::main;

pub fn tokenize(input: String) -> Result<Vec<TokenType>, Vec<LexerErrorType>> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let parse_errors: Vec<LexerErrorType> = Vec::new();

    let mut raw_input_vec: Vec<char> = Vec::new();
    input.chars().for_each(|c| raw_input_vec.push(c));

    let mut building_token: String = "".parse().unwrap();

    for cursor in raw_input_vec.len() {
        building_token.push(raw_input_vec[cursor]);
        let next_token = analyze_token(&building_token);
        if next_token != TokenType::None {
            tokens.push(next_token)
        }
    }

    return if parse_errors.len() > 0 { Err(parse_errors) } else { Ok(tokens) };
}
fn analyze_token(token: &String) -> TokenType {

    let mut resulting_token: TokenType = TokenType::None;

    //TODO Before all of those check for each one whether the result is still None
    //TODO Numeric literal finder
    //TODO String literal finder
    //TODO Special cases like >=, ==, etc. Check the double ones first for efficiency
    //TODO Match that checks the rest

    resulting_token
}

fn parser() {

}

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
    None, // No token found, gets removed later
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
    Int, // i32
    Float, // f32
    Double, // f64
    Long, // i64
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