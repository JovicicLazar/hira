use crate::token::Token;
use std::str::FromStr;
/*
use std::num::ParseIntError;
use std::iter::Peekable;
*/
pub fn lexer(src: &str) -> Vec<Token> {
    let mut tokens            = vec![];
    let mut input_chars = src.trim().chars().peekable();

    let mut comment           = String::new();

    let mut bracket_count     = 0; 
    let mut brace_count       = 0;
    let mut parenthesis_count = 0;

    while let Some(&char) = input_chars.peek() {

        match char {

            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&c) = input_chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            identifier.push(c);
                            input_chars.next();
                        }
                        _ => break,
                    }
                }
                
                // tokens match
                let token = match identifier.as_str() {
                    "if"        => Token::IF,
                    "else"      => Token::ELSE,
                    "elseif"    => Token::ELSEIF,
                    "while"     => Token::WHILE,
                    "for"       => Token::FOR,
                    "break"     => Token::BREAK,
                    "int"       => Token::INTEGER,
                    "string"    => Token::STRING,
                    "bool"      => Token::BOOLEAN,
                    "true"      => Token::TRUE,
                    "false"     => Token::FALSE,
                    "function"  => Token::FUNCTION,
                    "return"    => Token::RETURN,
                    "null"      => Token::NULL,
                    "global"    => Token::GLOBAL,
                    "input"     => Token::INPUT,
                    "print"     => Token::PRINT,
                    "and"       => Token::AND,
                    "or"        => Token::OR,
                    "not"       => Token::NOT,
                    _           => Token::IDENTIFIER(identifier),
                };
                
                tokens.push(token);
            }

            // numbers match
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = input_chars.peek() {
                    match c {
                        '0'..='9' => {
                            number.push(c);
                            input_chars.next();
                        }
                        _ => break,
                    }
                }
                let value = i64::from_str(number.as_str()).unwrap();
                tokens.push(Token::NUMBER(value));
            }

            // brackets match
            '(' => {
                tokens.push(Token::LEFT_PARENTHESIS);
                input_chars.next();
            }
            ')' => {
                tokens.push(Token::RIGHT_PARENTHESIS);
                input_chars.next();
            }
            '{' => {
                tokens.push(Token::LEFT_BRACE);
                input_chars.next();
            }
            '}' => {
                tokens.push(Token::RIGHT_BRACE);
                input_chars.next();
            }
            '[' => {
                tokens.push(Token::LEFT_BRACKET);
                input_chars.next();
            }
            ']' => {
                tokens.push(Token::RIGHT_BRACKET);
                input_chars.next();
            }

            // arithmetic/relational match
            '-' => {
                input_chars.next();
                match input_chars.peek() {
                    Some(&'=') => {
                        tokens.push(Token::DECREMENT_ASSIGN);
                        input_chars.next();
                    } 
                    _          => {
                        tokens.push(Token::MINUS);
                    }
                }
            }
            '+' => {
                input_chars.next();
                match input_chars.peek() {
                    Some(&'=') => {
                        tokens.push(Token::INCREMENT_ASSIGN);
                        input_chars.next();
                    }
                    _          => {
                        tokens.push(Token::PLUS);
                    }
                }
            }
            '*' => {
                input_chars.next();
                match input_chars.peek() {
                    Some(&'*') => {
                        tokens.push(Token::EXPONENT);
                        input_chars.next();
                    }
                    _          => {
                        tokens.push(Token::ASTERIX);
                    }
                }
            }
            '=' => {
                input_chars.next();
                match input_chars.peek() {
                    Some(&'=') => {
                        tokens.push(Token::EQUALITY);
                        input_chars.next();
                    }
                    _          => {
                        tokens.push(Token::ASSIGN);
                    }
                }
            }
            '<' => {
                input_chars.next();
                match input_chars.peek() {
                    Some(&'=') =>{
                        tokens.push(Token::LESS_THAN_OR_EQUAL);
                        input_chars.next();
                    }
                    _          => {
                        tokens.push(Token::LESS_THAN);
                    }
                }
            }
            '>' => {
                input_chars.next();
                match input_chars.peek() {
                    Some(&'=') => {
                        tokens.push(Token::GREATER_THAN_OR_EQUAL);
                        input_chars.next();
                    }
                    _          => {
                        tokens.push(Token::GREATER_THAN);
                    }
                }
            }
            '/' => {
                tokens.push(Token::SLASH);
                input_chars.next();
            }
            '%' => {
                tokens.push(Token::MODULO);
                input_chars.next();
            }

            //  comma, semicollon match
            ',' => {
                tokens.push(Token::COMMA);
                input_chars.next();
            }
            ';' => {
                tokens.push(Token::SEMICOLLON);
                input_chars.next();
            }

            // single quote match
            '\'' => {
                tokens.push(Token::SINGLE_QUOTE);
                input_chars.next();
            }

            // comment match
            '#' => {
                input_chars.next();
                while let Some(&c) = input_chars.peek() {
                    match c {
                        '\n' => {
                            input_chars.next();
                            break;
                        }
                        _    => {
                            input_chars.next();
                        }
                    }
                }
            }

            //  special match
            ' ' | '\n' | '\r' | '\t' 
                => {
                    input_chars.next();
            }

            // other match
            //_ => ()
            // NOTE: will probbably remove panic! in next commits
            _   => {
                panic!("Invalid character {}", char);
            }
        }
    }
    return tokens;
}


// just dummy tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_identifier() {
        let input = "foo";
        let expected = vec![Token::IDENTIFIER(String::from("foo"))];
        println!("{:#?}", lexer(input));
        assert_eq!(lexer(input), expected);
    }
    
    #[test]
    fn test_lex_number() {
        let input = "123";
        let expected = vec![Token::NUMBER(123)];
        assert_eq!(lexer(input), expected);
    }
    
 
    #[test]
    fn test_lex_operator() {
        let input = "+";
        let expected = vec![Token::PLUS];
        assert_eq!(lexer(input), expected);
    }

    #[test]
    fn test_single_quote() {
        let input = "'";
        let expected = vec![Token::SINGLE_QUOTE];
        assert_eq!(lexer(input), expected);
    }
}