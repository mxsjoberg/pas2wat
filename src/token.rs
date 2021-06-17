use crate::r#type::Type;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // types
    TYPE_SPEC(Type),
    // numbers
    INTEGER(i32),
    REAL(f64),
    RANGE(i32, i32),
    // booleans
    TRUE,
    FALSE,
    // operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    EQUAL,
    NOT_EQUAL,
    GREATER_THAN,
    GREATER_EQUAL,
    LESS_THAN,
    LESS_EQUAL,
    // expressions
    LPAR,
    RPAR,
    LBRA,
    RBRA,
    // variables
    ID(String),
    ASSIGN,
    // program
    BLOCK,
    SEMICOLON,
    COLON,
    COMMA,
    DOT,
    // keywords
    PROGRAM,
    //TYPE,
    VAR,
    PACKED,
    ARRAY,
    OF,
    INTEGER_DIV, // DIV
    INTEGER_MOD, // MOD
    BEGIN,
    END,
    EMPTY,
    WHILE,
    DO,
    //BREAK,
    IF,
    THEN,
    ELSE,
    // functions
    WRITELN,
    // end of file
    EOF
}