use crate::token::Token;
use std::str::FromStr;
use std::num::ParseIntError;
use std::iter::Peekable;

pub fn lexer(src: &str) -> Vec<Token> {
    let mut tokens            = vec![];
    let mut chars = src.trim().chars().peekable();

    let mut comment           = String::new();

    let mut bracket_count     = 0; 
    let mut brace_count       = 0;
    let mut parenthesis_count = 0;

    while let Some(&char) = chars.peek() {

        match char {

            'a'..='z' | 'A'..='Z' => {
                let mut identifier = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                            identifier.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }

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
                    _           => Token::IDENTIFIER(identifier),
                };
                
                tokens.push(token);
            }
            '0'..='9' => {
                let mut number = String::new();
                while let Some(&c) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            number.push(c);
                            chars.next();
                        }
                        _ => break,
                    }
                }
                let value = i64::from_str(number.as_str()).unwrap();
                tokens.push(Token::NUMBER(value));
            }
            _ => ()
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_identifier() {
        let input = "12foo";
        let expected = vec![Token::IDENTIFIER(String::from("12foo"))];
        println!("{:#?}", lexer(input));
        assert_eq!(lexer(input), expected);
    }
    
    #[test]
    fn test_lex_number() {
        let input = "123";
        let expected = vec![Token::NUMBER(123)];
        assert_eq!(lexer(input), expected);
    }
    
    /*
    #[test]
    fn test_lex_operator() {
        let input = "+";
        let expected = vec![Token::PLUS];
        assert_eq!(lexer(input), expected);
    }
    */
}