#[derive(Debug, PartialEq)]
pub enum Token {

    // identifier keywords
    IDENTIFIER(String),     // 'a..z', 'A..Z'
    NUMBER(i64),            // '0..9'

    // singlequote keyword
    SINGLE_QUOTE(String),   // '''

    // comment token
    COMMENT,                // '#'

    // brackets tokens
    EFT_PARENTHESIS,        // '('
    RIGHT_PARENTHESIS,      // ')'
    LEFT_BRACE,             // '{'
    RIGHT_BRACE,            // '}'
    LEFT_BRACKET,           // '['
    RIGHT_BRACKET,          // ']'

    //  commas, semicollon tokens
    COMMA,                  // ','
    SEMICOLLON,             // ';'  

    // assign operators
    ASSIGN,                 // '='
    INCREMENT_ASSIGN,       // '+='
    DECREMENT_ASSIGN,       // '-='

    // aritmethic operators
    PLUS,                   // '+'
    MINUS,                  // '-'
    ASTERIX,                // '*'
    SLASH,                  // '/'
    MODULO,                 // '%'
    EXPONENT,               // '**'

    // relational operators
    EQUALITY,               // '=='
    NON_EQUALITY,           // '!='
    LESS_THAN,              // '<'
    GREATER_THAN,           // '>'
    LESS_THAN_OR_EQUAL,     // '<='
    GREATER_THAN_OR_EQUAL,  // '>='

    // logical operators
    AND,                    // 'and'
    OR,                     // 'or'
    NOT,                    // '!'

    // condition keywords
    IF,                     // 'if'
    ELSE,                   // 'else'
    ELSEIF,                 // 'elseif
    
    // loop keywords 
    WHILE,                  // 'while'
    FOR,                    // 'for'
    BREAK,                  // 'break'

    // data type keywords
    INTEGER,                // 'int'
    STRING,                 // 'string'
    BOOLEAN,                // 'bool'

    // booleand value keywords
    TRUE,                   // 'true'
    FALSE,                  // 'false'

    // define function keyword
    FUNCTION,               // 'function'

    // return keyword
    RETURN,                 // 'return'

    // null keyword
    NULL,                   // 'null'

    // global variable keyword
    GLOBAL,                 // 'global'

    // input output
    INPUT,                  // 'input'
    PRINT,                  // 'print'
}
