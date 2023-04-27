use crate::token::Token;
use std::str::FromStr;

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
                let mut number_or_identifier = String::new();
                let mut identifier = false;
                while let Some(&c) = input_chars.peek() {
                    match c {
                        '0'..='9' => {
                            number_or_identifier.push(c);
                            input_chars.next();
                        }
                        'a'..='z' | 'A'..='Z' => {
                            identifier = true;
                            number_or_identifier.push(c);
                            input_chars.next();
                        }
                        _ => break,
                    }
                }
                if identifier {
                    tokens.push(Token::IDENTIFIER(number_or_identifier.to_string()));
                } else {
                    let value = i64::from_str(number_or_identifier.as_str()).unwrap();
                    tokens.push(Token::NUMBER(value));
                }   
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

#[cfg(test)]
mod lex_tests {
    use super::*;

    // basic tests
    #[test]
    fn test_lexer_identifiers() {
        let input = "if else elseif while for break int string bool true false function return null global input print and or not myIdentifier 123";
        let expected_output = vec![
            Token::IF, Token::ELSE, Token::ELSEIF, Token::WHILE, Token::FOR, Token::BREAK,
            Token::INTEGER, Token::STRING, Token::BOOLEAN, Token::TRUE, Token::FALSE,
            Token::FUNCTION, Token::RETURN, Token::NULL, Token::GLOBAL, Token::INPUT, Token::PRINT,
            Token::AND, Token::OR, Token::NOT,
            Token::IDENTIFIER("myIdentifier".to_string()), Token::NUMBER(123)
        ];
        assert_eq!(lexer(input), expected_output);
    }

    #[test]
    fn test_lexer_numbers() {
        let input = "123 808 34412";
        let expected_output = vec![
            Token::NUMBER(123), Token::NUMBER(808), Token::NUMBER(34412)
        ];
        assert_eq!(lexer(input), expected_output);
    }

    #[test]
    fn test_lexer_brackets() {
        let input = "(){}[]";
        let expected_output = vec![
            Token::LEFT_PARENTHESIS, Token::RIGHT_PARENTHESIS,
            Token::LEFT_BRACE, Token::RIGHT_BRACE,
            Token::LEFT_BRACKET, Token::RIGHT_BRACKET,
        ];
        assert_eq!(lexer(input), expected_output);
    }
    
    #[test]
    fn test_lexer_arithmetic() {
        let input = "+= -= *= /= %= ++ -- ** + - * / %";
        let expected_output = vec![
            Token::INCREMENT_ASSIGN, Token::DECREMENT_ASSIGN,
            Token::ASTERIX, Token::ASSIGN, Token::SLASH, Token::ASSIGN, Token::MODULO, Token::ASSIGN,
            Token::PLUS, Token::PLUS, Token::MINUS, Token::MINUS, Token::EXPONENT,
            Token::PLUS, Token::MINUS, Token::ASTERIX, Token::SLASH, Token::MODULO,
        ];
        assert_eq!(lexer(input), expected_output);
    }
    
    #[test]
    fn test_lexer_relational() {
        let input = "< <= > >= ==";
        let expected_output = vec![
            Token::LESS_THAN, Token::LESS_THAN_OR_EQUAL,
            Token::GREATER_THAN, Token::GREATER_THAN_OR_EQUAL,
            Token::EQUALITY
        ];
        assert_eq!(lexer(input), expected_output);
    }
    
    #[test]
    fn test_lexer_comma_semicolon() {
        let input = ",;";
        let expected_output = vec![
            Token::COMMA, Token::SEMICOLLON,
        ];
        assert_eq!(lexer(input), expected_output);
    }
    
    #[test]
    fn test_lexer_single_quote() {
        let input = "'";
        let expected_output = vec![
            Token::SINGLE_QUOTE,
        ];
        assert_eq!(lexer(input), expected_output);
    }
    
    #[test]
    fn test_lexer_comments() {
        let input = "this # is a comment";
        let expected_output = vec![
            Token::IDENTIFIER("this".to_string())
        ];
        assert_eq!(lexer(input), expected_output);
    }

    #[test]
    fn test_numbers_and_identifiers() {
        let input = "123 123var 1 var123 123";
        let expected_output = vec![
            Token::NUMBER(123),
            Token::IDENTIFIER("123var".to_string()),
            Token::NUMBER(1),
            Token::IDENTIFIER("var123".to_string()),
            Token::NUMBER(123)
        ];
        assert_eq!(lexer(input), expected_output);
    }

    // code simulation tests
    #[test]
    fn function_return() {
        let input = "
            function fun(param) {
                if a == b {
                    return true;
                } else {
                    return false;
                }    
            }
        ";
        let expected_output = vec![
            Token::FUNCTION, Token::IDENTIFIER("fun".to_string()), Token::LEFT_PARENTHESIS, Token::IDENTIFIER("param".to_string()),
            Token::RIGHT_PARENTHESIS, Token::LEFT_BRACE, Token::IF, Token::IDENTIFIER("a".to_string()), Token::EQUALITY, Token::IDENTIFIER("b".to_string()),
            Token::LEFT_BRACE, Token::RETURN, Token::TRUE, Token::SEMICOLLON, Token::RIGHT_BRACE, Token::ELSE, Token::LEFT_BRACE,
            Token::RETURN, Token::FALSE, Token::SEMICOLLON, Token::RIGHT_BRACE, Token::RIGHT_BRACE
        ];
        assert_eq!(lexer(input), expected_output);
    }
}