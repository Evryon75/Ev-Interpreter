use colour::*;
use once_cell::sync::Lazy;
use regex::Regex;

const EOF_SYMBOL: char = '⨂'; //Used for safer parsing in some cases
                              //STEP ONE: Tokenization
pub fn tokenize(input: String) -> Result<Vec<TokenType>, Vec<LexerErrorType>> {
    let mut tokens: Vec<TokenType> = Vec::new(); //Storing tokens
    let lexing_errors: Vec<LexerErrorType> = Vec::new(); //Storing errors
    let raw_input_vec: Vec<char> = input.chars().collect(); //Collecting raw input into characters
    let mut building_token: String = "".parse().unwrap(); //String reading the raw input
    let mut cursor = 0;
    while cursor < raw_input_vec.len() {
        building_token.push(raw_input_vec[cursor]); //Read
        if building_token.as_str() == " " //Ignore these sequences
            || building_token.as_str() == "\n"
            || building_token.as_str() == "\r"
        {
            building_token = "".parse().unwrap();
        }
        //Storing the result of the analysis
        let analysis_result = analyze_token(
            &building_token,
            if cursor < raw_input_vec.len() - 1 {
                raw_input_vec[cursor + 1]
            } else {
                EOF_SYMBOL
            },
        );
        //If everything is successful (not a comment, not "None", and no errors) save the token
        if analysis_result.1 == LexerErrorType::None {
            if analysis_result.0 != TokenType::None && analysis_result.0 != TokenType::LineComment {
                tokens.push(analysis_result.0);
                building_token = "".parse().unwrap();
            } else if analysis_result.0 == TokenType::LineComment {
                building_token = "".parse().unwrap();
            }
        } else {
            //Show possible errors
            red_ln!("Lexing Error: {:?}", analysis_result.1);
            panic!();
        }
        cursor += 1;
    }
    green_ln!("Lexing: finished successfully ✔");
    //Return the result of the tokenization
    if lexing_errors.len() > 0 {
        Err(lexing_errors)
    } else {
        Ok(tokens)
    }
}
fn analyze_token(token: &String, next_char: char) -> (TokenType, LexerErrorType) {
    use TokenType::*; //For cleaner looking code
                      //Default state
    let mut resulting_token: TokenType = None;
    let mut error: LexerErrorType = LexerErrorType::None;
    if !token.trim().starts_with("//") {
        //If there is a comment ignore all until the next line
        //Simple tokens
        resulting_token = match token.as_str() {
            //First basic check for simple tokens
            "fun" => Fun, //This means you cant have identifiers starting with "fun" or "let", etc
            "let" => Let,
            "(" => LParen,
            ")" => RParen,
            "{" => LBrace,
            "}" => RBrace,
            "==" => DoubleEqual,
            ">=" => GreaterThanEqual,
            "<=" => LessThanEqual,
            ";" => Semicolon,
            "||" => Or,
            "&&" => And,
            "!" => {
                red_ln!(
                    "The Ev programming language does not provide the \"Not\" [!] logical operator" //I tried
                );
                blue_ln!(
                    "fun not = (param) {
    let result = false;
    if param {
        result = false;
    } else {
        result = true;
    };
    return result;
};"
                );
                panic!()
            }
            "+" => AdditionOp,
            "-" => SubtractionOp,
            "*" => MultiplicationOp,
            ":" => Colon,
            "return" => Return,
            "if" => If,
            "else" => Else,
            "while" => While,
            "," => Comma,
            "true" => BooleanLiteral { value: true },
            "false" => BooleanLiteral { value: false },
            &_ => None,
        };
        //Slightly more complex tokens
        if token == "/" && next_char != '/' {
            resulting_token = DivisionOp;
        }
        if token == "|" && next_char != '|' {
            resulting_token = TypeSeparator;
        }
        if token == "=" && next_char != '=' {
            resulting_token = Equal;
        }
        if token == ">" && next_char != '=' {
            resulting_token = GreaterThan;
        }
        if token == "<" && next_char != '=' {
            resulting_token = LessThan;
        }
        if token == "as" && next_char == ' ' {
            resulting_token = Caster;
        }
        //Most complex tokens
        if resulting_token == None && error == LexerErrorType::None {
            //Numeric literals
            let mut dot = false;
            let mut valid_num = !token.is_empty(); // If its empty, default to false
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
        if resulting_token == None && error == LexerErrorType::None {
            //String literals
            // String literal
            if token.starts_with('"') && token.ends_with('"') && token.len() > 1 {
                resulting_token = StringLiteral {
                    string_type: StringLiteralType::String,
                    value: token.replace("\"", "").to_string(),
                };
            } else if token.starts_with('"') && next_char == EOF_SYMBOL {
                error = LexerErrorType::StringLiteralDoesNotEnd;
            }
            if token.starts_with("'") && token.ends_with("'") && token.len() > 1 {
                if token.len() < 4 {
                    resulting_token = StringLiteral {
                        string_type: StringLiteralType::Char,
                        value: token.replace("'", "").to_string(),
                    };
                } else {
                    error = LexerErrorType::CharIsTooLong
                }
            } else if token.starts_with("'") && next_char == EOF_SYMBOL {
                error = LexerErrorType::StringLiteralDoesNotEnd;
            }
        }
        if resulting_token == None && error == LexerErrorType::None {
            //Identifiers
            if (" ,.?^'{[()]}+-/*!|;=\"".contains(next_char) || next_char == EOF_SYMBOL)
                && valid_identifier(token) //For added safety
                && !token.starts_with('\"')
                && !token.is_empty()
                && token.is_ascii()
            {
                resulting_token = Identifier {
                    identifier: token.to_string(),
                }
            } else if !token.is_ascii() {
                //Im not going to have support for ascii characters
                error = LexerErrorType::NonAsciiCharactersInIdentifier
            }
        }
    } else if token.contains('\n') {
        //End the comment and proceed with the rest
        resulting_token = LineComment;
    }
    //Return
    (resulting_token, error)
}
//Simple regex function for helping with identifiers
fn valid_identifier(identifier: &str) -> bool {
    //The regex is taken from stackoverflow of course
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[_a-zA-Z]\w{0,30}").unwrap());
    RE.is_match(identifier)
}
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    //Main enum for tokens
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
    BooleanLiteral {
        value: bool,
    },
    Let,
    Fun,
    LParen,           // (
    RParen,           // )
    LBrace,           // {
    RBrace,           // }
    Equal,            // =
    DoubleEqual,      // ==
    GreaterThan,      // >
    GreaterThanEqual, // >=
    LessThan,         // <
    LessThanEqual,    // <=
    Semicolon,        // ;
    Or,               // ||
    And,              // &&
    LineComment,      // //
    TypeSeparator,    // |
    AdditionOp,       // +
    SubtractionOp,    // -
    MultiplicationOp, // *
    DivisionOp,       // /
    Caster,           // as
    Colon,            // :
    Return,           // return
    If,               // if
    Else,             // else
    While,            // while
    Comma,            // ,
    None,             // No token found, gets removed later
}
#[derive(Debug, PartialEq, Clone)]
pub enum NumericLiteralType {
    Int,    // i32
    Float,  // f32
    Double, // f64
    Long,   // i64
}
#[derive(Debug, PartialEq, Clone)]
pub enum StringLiteralType {
    String,
    Char,
}
#[derive(Debug, PartialEq, Clone)]
pub enum LexerErrorType {
    InvalidFloatingPoint,
    StringLiteralDoesNotEnd,
    CharIsTooLong,
    NonAsciiCharactersInIdentifier,
    None,
}
