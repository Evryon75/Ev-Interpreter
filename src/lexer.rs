use once_cell::sync::Lazy;
use crate::lexer::TokenType::NumericLiteral;
use regex::Regex;

pub fn tokenize(input: String) -> Result<Vec<TokenType>, Vec<LexerErrorType>> {
    let mut tokens: Vec<TokenType> = Vec::new();
    let parse_errors: Vec<LexerErrorType> = Vec::new();

    let mut raw_input_vec: Vec<char> = Vec::new();
    input.chars().for_each(|c| raw_input_vec.push(c));

    let mut building_token: String = "".parse().unwrap();
    let mut cursor = 0;
    while cursor < raw_input_vec.len() {
        building_token.push(raw_input_vec[cursor]);
        if building_token.as_str() == " " {
            building_token = "".parse().unwrap();
        }

        let analysis_result = analyze_token(
            &building_token,
            if cursor < raw_input_vec.len() - 1 {
                raw_input_vec[cursor + 1]
            } else {
                '⨂'
            },
        );

        if analysis_result.1 == LexerErrorType::None {
            if analysis_result.0 != TokenType::None {
                tokens.push(analysis_result.0);
                building_token = "".parse().unwrap();
            }
        } else {
            println!("Lexing Error: {:?}", analysis_result.1);
            break
        }
        cursor += 1;
    }

    if parse_errors.len() > 0 {
        Err(parse_errors)
    } else {
        Ok(tokens)
    }
}
fn analyze_token(token: &String, next_char: char) -> (TokenType, LexerErrorType) {
    let mut resulting_token: TokenType = TokenType::None;
    let mut error: LexerErrorType = LexerErrorType::None;

    //TODO Special cases like >=, ==, etc. Check the double ones first for efficiency
    //TODO Match that checks the rest

    // Numeric literals
    if resulting_token == TokenType::None
        && error == LexerErrorType::None
        && !token.starts_with('"')
    {
        let mut dot = false;
        let mut valid_num = true;
        token.trim().chars().for_each(|c| {
            if !".0123456789".contains(c) {
                valid_num = false;
            }
            if !dot {
                if c == '.' {
                    dot = true
                }
            } else {
                if c == '.' {
                    error = LexerErrorType::InvalidFloatingPoint;
                    valid_num = false
                }
            }
            if token.starts_with('.') {
                valid_num = false
            }
        });
        if !".0123456789".contains(next_char) && valid_num && !token.is_empty() {
            resulting_token = NumericLiteral {
                numeric_type: if token.contains('.') {
                    let mut post_dot = false;
                    let mut decimals = 0;
                    token.chars().for_each(|c| {
                        if post_dot {
                            decimals += 1;
                        }
                        if c == '.' && !post_dot {
                            post_dot = true
                        }
                    });
                    if decimals < 8 {
                        NumericLiteralType::Float
                    } else {
                        NumericLiteralType::Double
                    }
                } else if token.len() < 17 {
                    NumericLiteralType::Int
                } else {
                    NumericLiteralType::Long
                },
                value: token.trim().parse::<f64>().unwrap(),
            };
        }
    }
    // String literals
    if resulting_token == TokenType::None && error == LexerErrorType::None {
        // String literal
        if token.starts_with('"') && token.ends_with('"') && token.len() > 1 {
            resulting_token = TokenType::StringLiteral {
                string_type: StringLiteralType::String,
                value: token.replace("\"", "").to_string(),
            };
        } else if token.starts_with('"') && next_char == '⨂' {
            error = LexerErrorType::StringLiteralDoesNotEnd;
        }
        if token.starts_with("'") && token.ends_with("'") && token.len() > 1 {
            if token.len() < 4 {
                resulting_token = TokenType::StringLiteral {
                    string_type: StringLiteralType::Char,
                    value: token.replace("'", "").to_string(),
                };
            } else {
                error = LexerErrorType::CharIsTooLong
            }
        } else if token.starts_with("'") && next_char == '⨂' {
            error = LexerErrorType::StringLiteralDoesNotEnd;
        }
    }
    // Identifier
    if resulting_token == TokenType::None
        && error == LexerErrorType::None
    {
        if (" .?^'{[()]}+-/*!|;=\"".contains(next_char) || next_char == '⨂')
            && valid_identifier(token)
            && !token.starts_with('\"')
            && !token.is_empty()
            && token.is_ascii()
        {
            resulting_token = TokenType::Identifier {
                identifier: token.to_string(),
            }
        } else if !token.is_ascii() {
            error = LexerErrorType::NonAsciiCharactersInIdentifier
        }
    }

    (resulting_token, error)
}

fn valid_identifier(identifier: &str) -> bool {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new("[_a-zA-Z][_a-zA-Z0-9]*").unwrap());
    RE.is_match(identifier)
}


#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumericLiteral {
        numeric_type: NumericLiteralType,
        value: f64,
    },
    StringLiteral {
        string_type: StringLiteralType,
        value: String,
    },
    Identifier {
        identifier: String,
    },
    DeclarationKeyword {
        keyword: DeclarationKeywords,
    },
    Eof,
    LParen,           // (
    RParen,           // )
    LBracket,         // [
    RBracket,         // ]
    LBrace,           // {
    RBrace,           // }
    Equal,            // =
    GreaterThan,      // >
    LessThan,         // <
    Semicolon,        // ;
    Or,               // ||
    And,              // &&
    Not,              // !
    LineComment,      // //
    OpenComment,      // />
    CloseComment,     // </
    TypeSeparator,    // |
    AdditionOp,       // +
    SubtractionOp,    // -
    MultiplicationOp, // *
    DivisionOp,       // /
    ArrowReturn,      // >>
    SingleQuote,      // '
    DoubleQuote,      // "
    Pointer,          // ^
    Dereference,      // &
    Caster,           // as
    Colon,            // :
    Break,            // break
    Continue,         // continue
    Return,           // return
    If,               // if
    Else,             // else
    For,              // for
    Switch,           // switch
    Try,              // try
    Catch,            // catch
    Import,           // import
    Dot,              // dot
    This,             // this
    BackSlash,        // \
    BackSlashN,       // \n
    Ternary,          // ?
    None,             // No token found, gets removed later
    Debug {
        debug: String,
    }, //TODO Remove this when lexer is done
}
#[derive(Debug, PartialEq)]
pub enum DeclarationKeywords {
    Class,  // class
    Struct, // struct
    Let,    // let
    Bool,   // bool
    Int,    // int
    Float,  // float
    Double, // double
    Long,   // long
    String, // string
    Char,   // char
}
#[derive(Debug, PartialEq)]
pub enum NumericLiteralType {
    Int,    // i32
    Float,  // f32
    Double, // f64
    Long,   // i64
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
    StringLiteralDoesNotEnd,
    CharIsTooLong,
    NonAsciiCharactersInIdentifier,
    None,
}
