use std::alloc::handle_alloc_error;
use std::ptr::addr_of;
use crate::lexer::TokenType::NumericLiteral;

pub fn tokenize(input: String) -> Result<Vec<TokenType>, Vec<LexerErrorType>> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let parse_errors: Vec<LexerErrorType> = Vec::new();

    let mut raw_input_vec: Vec<char> = Vec::new();
    input.chars().for_each(|c| raw_input_vec.push(c));

    let mut building_token: String = "".parse().unwrap();
    let mut cursor = 0;

    while cursor < raw_input_vec.len() {
        building_token.push(raw_input_vec[cursor]);

        let analysis_result= analyze_token( &building_token,
if cursor < raw_input_vec.len() - 1 { raw_input_vec[cursor + 1] } else { ' ' });

        if analysis_result.1 == LexerErrorType::None {
            if analysis_result.0 != TokenType::None {
                tokens.push(analysis_result.0);
                building_token = "".parse().unwrap();
            }
        } else {
            println!("Error: {:?}", analysis_result.1)
        }
        cursor += 1;
    }
    println!("Everything is working properly up to here");
    if parse_errors.len() > 0 { Err(parse_errors) } else { Ok(tokens) }
}
fn analyze_token(token: &String, next_char: char) -> (TokenType, LexerErrorType) {

    let mut resulting_token: TokenType = TokenType::None;
    let mut error: LexerErrorType = LexerErrorType::None;

    //TODO Before all of those check for each one whether the result is still None
    //TODO String literal finder
    //TODO Special cases like >=, ==, etc. Check the double ones first for efficiency
    //TODO Match that checks the rest

    if resulting_token == TokenType::None && error == LexerErrorType::None { // Numeric literal
        let mut dot = false;
        let mut valid_num = true;
        token.trim().chars().for_each(|c|{
            if !".0123456789".contains(c) {
                valid_num = false;
            }
            if !dot { if c == '.' { dot = true }
            } else {
                if c == '.' { error = LexerErrorType::InvalidFloatingPoint; valid_num = false }
            }
            if token.starts_with('.') { valid_num = false }
        });
        if !".0123456789".contains(next_char) && valid_num {
            resulting_token = TokenType::NumericLiteral
            { numeric_type:
            if token.contains('.') {
                let mut post_dot = false;
                let mut decimals = 0;
                token.chars().for_each(|c| {
                    if post_dot {
                        decimals += 1;
                    }
                    if c == '.' && !post_dot { post_dot = true}
                });
                if decimals < 8 { NumericLiteralType::Float } else { NumericLiteralType::Double }
            } else if token.len() < 17 { NumericLiteralType::Int }
            else {
                NumericLiteralType::Long }, value: token.trim().parse::<f64>().unwrap()
            };
        }
    }
    if resulting_token == TokenType::None && error == LexerErrorType::None {

    }

    (resulting_token, error)
}

fn parser() {

}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumericLiteral { numeric_type: NumericLiteralType, value: f64 },
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
    InvalidFloatingPoint,
    None
}