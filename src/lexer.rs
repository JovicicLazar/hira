use crate::token::Token;
use std::str::FromStr;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    input_chars: Peekable<Chars<'a>>,
    comment: String,
    bracket_count: usize,
    brace_count: usize,
    parenthesis_count: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Lexer {
            tokens: vec![],
            input_chars: src.trim().chars().peekable(),
            comment: String::new(),
            bracket_count: 0,
            brace_count: 0,
            parenthesis_count: 0,
        }
    }
}

impl <'a> Lexer<'a> {
    pub fn tokenize(&mut self, src: &str) -> Vec<Token> {

        while let Some(&char) = self.input_chars.peek() {
    
            match char {
    
                'a'..='z' | 'A'..='Z' => {
                    let mut identifier = String::new();
                    while let Some(&c) = self.input_chars.peek() {
                        match c {
                            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => {
                                identifier.push(c);
                                self.input_chars.next();
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
                    
                    self.tokens.push(token);
                }
    
                // numbers match
                '0'..='9' => {
                    let mut number_or_identifier = String::new();
                    let mut identifier = false;
                    while let Some(&c) = self.input_chars.peek() {
                        match c {
                            '0'..='9' => {
                                number_or_identifier.push(c);
                                self.input_chars.next();
                            }
                            'a'..='z' | 'A'..='Z' => {
                                identifier = true;
                                number_or_identifier.push(c);
                                self.input_chars.next();
                            }
                            _ => break,
                        }
                    }
                    if identifier {
                        self.tokens.push(Token::IDENTIFIER(number_or_identifier.to_string()));
                    } else {
                        let value = i64::from_str(number_or_identifier.as_str()).unwrap();
                        self.tokens.push(Token::NUMBER(value));
                    }   
                }
    
                // brackets match
                '(' => {
                    self.tokens.push(Token::LEFT_PARENTHESIS);
                    self.input_chars.next();
                }
                ')' => {
                    self.tokens.push(Token::RIGHT_PARENTHESIS);
                    self.input_chars.next();
                }
                '{' => {
                    self.tokens.push(Token::LEFT_BRACE);
                    self.input_chars.next();
                }
                '}' => {
                    self.tokens.push(Token::RIGHT_BRACE);
                    self.input_chars.next();
                }
                '[' => {
                    self.tokens.push(Token::LEFT_BRACKET);
                    self.input_chars.next();
                }
                ']' => {
                    self.tokens.push(Token::RIGHT_BRACKET);
                    self.input_chars.next();
                }
    
                // arithmetic/relational match
                '-' => {
                    self.input_chars.next();
                    match self.input_chars.peek() {
                        Some(&'=') => {
                            self.tokens.push(Token::DECREMENT_ASSIGN);
                            self.input_chars.next();
                        } 
                        _          => {
                            self.tokens.push(Token::MINUS);
                        }
                    }
                }
                '+' => {
                    self.input_chars.next();
                    match self.input_chars.peek() {
                        Some(&'=') => {
                            self.tokens.push(Token::INCREMENT_ASSIGN);
                            self.input_chars.next();
                        }
                        _          => {
                            self.tokens.push(Token::PLUS);
                        }
                    }
                }
                '*' => {
                    self.input_chars.next();
                    match self.input_chars.peek() {
                        Some(&'*') => {
                            self.tokens.push(Token::EXPONENT);
                            self.input_chars.next();
                        }
                        _          => {
                            self.tokens.push(Token::ASTERIX);
                        }
                    }
                }
                '=' => {
                    self.input_chars.next();
                    match self.input_chars.peek() {
                        Some(&'=') => {
                            self.tokens.push(Token::EQUALITY);
                            self.input_chars.next();
                        }
                        _          => {
                            self.tokens.push(Token::ASSIGN);
                        }
                    }
                }
                '<' => {
                    self.input_chars.next();
                    match self.input_chars.peek() {
                        Some(&'=') =>{
                            self.tokens.push(Token::LESS_THAN_OR_EQUAL);
                            self.input_chars.next();
                        }
                        _          => {
                            self.tokens.push(Token::LESS_THAN);
                        }
                    }
                }
                '>' => {
                    self.input_chars.next();
                    match self.input_chars.peek() {
                        Some(&'=') => {
                            self.tokens.push(Token::GREATER_THAN_OR_EQUAL);
                            self.input_chars.next();
                        }
                        _          => {
                            self.tokens.push(Token::GREATER_THAN);
                        }
                    }
                }
                '/' => {
                    self.tokens.push(Token::SLASH);
                    self.input_chars.next();
                }
                '%' => {
                    self.tokens.push(Token::MODULO);
                    self.input_chars.next();
                }
    
                //  comma, semicollon match
                ',' => {
                    self.tokens.push(Token::COMMA);
                    self.input_chars.next();
                }
                ';' => {
                    self.tokens.push(Token::SEMICOLLON);
                    self.input_chars.next();
                }
    
                // single quote match
                '\'' => {
                    self.tokens.push(Token::SINGLE_QUOTE);
                    self.input_chars.next();
                }
    
                // comment match
                '#' => {
                    self.input_chars.next();
                    while let Some(&c) = self.input_chars.peek() {
                        match c {
                            '\n' => {
                                self.input_chars.next();
                                break;
                            }
                            _    => {
                                self.input_chars.next();
                            }
                        }
                    }
                }
    
                //  special match
                ' ' | '\n' | '\r' | '\t' 
                    => {
                        self.input_chars.next();
                }
    
                // other match
                //_ => ()
                // NOTE: will probbably remove panic! in next commits
                _   => {
                    panic!("Invalid character {}", char);
                }
            }
        }
        return self.tokens.clone();
    }
}

#[cfg(test)]
mod lex_tests {
    use super::*;

    // basic tests
    #[test]
    fn test_lexer_identifiers() {
        let input = "if else elseif while for break int string bool true false function return null global input print and or not myIdentifier 123";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::IF, Token::ELSE, Token::ELSEIF, Token::WHILE, Token::FOR, Token::BREAK,
            Token::INTEGER, Token::STRING, Token::BOOLEAN, Token::TRUE, Token::FALSE,
            Token::FUNCTION, Token::RETURN, Token::NULL, Token::GLOBAL, Token::INPUT, Token::PRINT,
            Token::AND, Token::OR, Token::NOT,
            Token::IDENTIFIER("myIdentifier".to_string()), Token::NUMBER(123)
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }

    #[test]
    fn test_lexer_numbers() {
        let input = "123 808 34412";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::NUMBER(123), Token::NUMBER(808), Token::NUMBER(34412)
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }

    #[test]
    fn test_lexer_brackets() {
        let input = "(){}[]";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::LEFT_PARENTHESIS, Token::RIGHT_PARENTHESIS,
            Token::LEFT_BRACE, Token::RIGHT_BRACE,
            Token::LEFT_BRACKET, Token::RIGHT_BRACKET,
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }
    
    #[test]
    fn test_lexer_arithmetic() {
        let input = "+= -= *= /= %= ++ -- ** + - * / %";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::INCREMENT_ASSIGN, Token::DECREMENT_ASSIGN,
            Token::ASTERIX, Token::ASSIGN, Token::SLASH, Token::ASSIGN, Token::MODULO, Token::ASSIGN,
            Token::PLUS, Token::PLUS, Token::MINUS, Token::MINUS, Token::EXPONENT,
            Token::PLUS, Token::MINUS, Token::ASTERIX, Token::SLASH, Token::MODULO,
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }
    
    #[test]
    fn test_lexer_relational() {
        let input = "< <= > >= ==";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::LESS_THAN, Token::LESS_THAN_OR_EQUAL,
            Token::GREATER_THAN, Token::GREATER_THAN_OR_EQUAL,
            Token::EQUALITY
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }
    
    #[test]
    fn test_lexer_comma_semicolon() {
        let input = ",;";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::COMMA, Token::SEMICOLLON,
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }
    
    #[test]
    fn test_lexer_single_quote() {
        let input = "'";
        let mut lexer_o = Lexer::new(input);;
        let expected_output = vec![
            Token::SINGLE_QUOTE,
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }
    
    #[test]
    fn test_lexer_comments() {
        let input = "this # is a comment";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::IDENTIFIER("this".to_string())
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }

    #[test]
    fn test_numbers_and_identifiers() {
        let input = "123 123var 1 var123 123";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::NUMBER(123),
            Token::IDENTIFIER("123var".to_string()),
            Token::NUMBER(1),
            Token::IDENTIFIER("var123".to_string()),
            Token::NUMBER(123)
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }

    // code simulation tests
    #[test]
    fn test_pseudo_code_function() {
        let input = "
            function fun( param ) {
                if param >= 12  {
                    return true;
                } else {
                    return false;
                }    
            }

            function fun2( param1, param2 ){
                if param1 == fun( param2 ) {
                    return true;
                } else {
                    return false;
                }
            }

            param1 = false;
            param2 = 13;

            fun2( param1, param2);
        ";
        let mut lexer_o = Lexer::new(input);
        let expected_output = vec![
            Token::FUNCTION, Token::IDENTIFIER("fun".to_string()), Token::LEFT_PARENTHESIS, Token::IDENTIFIER("param".to_string()),
            Token::RIGHT_PARENTHESIS, Token::LEFT_BRACE, Token::IF, Token::IDENTIFIER("param".to_string()), Token::GREATER_THAN_OR_EQUAL, Token::NUMBER(12),
            Token::LEFT_BRACE, Token::RETURN, Token::TRUE, Token::SEMICOLLON, Token::RIGHT_BRACE, Token::ELSE, Token::LEFT_BRACE,
            Token::RETURN, Token::FALSE, Token::SEMICOLLON, Token::RIGHT_BRACE, Token::RIGHT_BRACE,
            Token::FUNCTION, Token::IDENTIFIER("fun2".to_string()), Token::LEFT_PARENTHESIS, Token::IDENTIFIER("param1".to_string()), Token::COMMA, Token::IDENTIFIER("param2".to_string()),
            Token::RIGHT_PARENTHESIS, Token::LEFT_BRACE, Token::IF, Token::IDENTIFIER("param1".to_string()), Token::EQUALITY, Token::IDENTIFIER("fun".to_string()),
            Token::LEFT_PARENTHESIS, Token::IDENTIFIER("param2".to_string()), Token::RIGHT_PARENTHESIS, Token::LEFT_BRACE, Token::RETURN, Token::TRUE, Token::SEMICOLLON,
            Token::RIGHT_BRACE, Token::ELSE, Token::LEFT_BRACE, Token::RETURN, Token::FALSE, Token::SEMICOLLON, Token::RIGHT_BRACE, Token::RIGHT_BRACE,
            Token::IDENTIFIER("param1".to_string()), Token::ASSIGN, Token::FALSE, Token::SEMICOLLON, Token::IDENTIFIER("param2".to_string()), Token::ASSIGN, Token::NUMBER(13), Token::SEMICOLLON,
            Token::IDENTIFIER("fun2".to_string()), Token::LEFT_PARENTHESIS, Token::IDENTIFIER("param1".to_string()), Token::COMMA, Token::IDENTIFIER("param2".to_string()), Token::RIGHT_PARENTHESIS,
            Token::SEMICOLLON
        ];
        assert_eq!(lexer_o.tokenize(input), expected_output);
    }
}