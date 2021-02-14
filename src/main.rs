/*

pawa: pascal-to-wat prototype compiler

----

program                 : PROGRAM variable SEMICOLON block DOT

block                   : declarations compound_statement
compound_statement      : BEGIN statement_list END
statement_list          : statement (SEMICOLON statement)*
statement               : compound_statement | structured_statement | assignment_statement | function_statement | empty
structured_statement    : if_statement | while_statement

if_statement            : IF expression THEN statement (ELSE statement)?
while_statement         : WHILE expression DO statement
function_statement      : (WRITELN) LPAR expression RPAR

declarations            : VAR (variable_declaration SEMICOLON)+ | empty
variable_declaration    : ID (COMMA ID)* COLON (type_spec | structured_type)
structured_type         : (PACKED)? (array_type)
array_type              : ARRAY LBRA RANGE (COMMA RANGE)* RBRA OF type_spec
type_spec               : INTEGER | REAL

assignment_statement    : variable ASSIGN simple_expression
variable                : ID (LBRA simple_expression RBRA)?

expression              : (TRUE | FALSE) | simple_expression ((EQUAL | GREATER_THAN | GREATER_EQUAL | LESS_THAN | LESS_EQUAL | NOT_EQUAL) simple_expression)?
simple_expression       : term ((PLUS | MINUS) term)*

term                    : factor ((MULTIPLY | DIVIDE | INTEGER_DIV | INTEGER_MOD) factor)*
factor                  : PLUS factor | MINUS factor | INTEGER | REAL | LPAR expression RPAR | variable
empty                   : 

*/

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq)]
enum Type {
    INTEGER,
    REAL,
}

#[derive(Clone, Debug, PartialEq)]
enum Token {
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

// types
static NTYPE_INTEGER            : &'static str = "i32";
static NTYPE_REAL               : &'static str = "f64";
// characters
const CHAR_DOT                  : char = '.';
const CHAR_UNDERSCORE           : char = '_';
const CHAR_COMMA                : char = ',';
const CHAR_COLON                : char = ':';
const CHAR_SEMICOLON            : char = ';';
const CHAR_EQUAL                : char = '=';
const CHAR_PLUS                 : char = '+';
const CHAR_MINUS                : char = '-';
const CHAR_MULTIPLY             : char = '*';
const CHAR_DIVIDE               : char = '/';
const CHAR_LPAR                 : char = '(';
const CHAR_RPAR                 : char = ')';
const CHAR_LBRA                 : char = '[';
const CHAR_RBRA                 : char = ']';
const CHAR_LCUR                 : char = '{';
const CHAR_RCUR                 : char = '}';
const CHAR_GREATER_THAN         : char = '>';
const CHAR_LESS_THAN            : char = '<';
// keywords
const KEY_PROGRAM               : &str = "PROGRAM";
const KEY_VAR                   : &str = "VAR";
const KEY_DIV                   : &str = "DIV";
const KEY_MOD                   : &str = "MOD";
const KEY_BEGIN                 : &str = "BEGIN";
const KEY_END                   : &str = "END";
const KEY_INTEGER               : &str = "INTEGER";
const KEY_LONGINT               : &str = "LONGINT";
const KEY_REAL                  : &str = "REAL";
const KEY_TRUE                  : &str = "TRUE";
const KEY_FALSE                 : &str = "FALSE";
const KEY_PACKED                : &str = "PACKED";
const KEY_ARRAY                 : &str = "ARRAY";
const KEY_OF                    : &str = "OF";
const KEY_WHILE                 : &str = "WHILE";
const KEY_DO                    : &str = "DO";
//const KEY_BREAK                 : &str = "BREAK";
const KEY_IF                    : &str = "IF";
const KEY_THEN                  : &str = "THEN";
const KEY_ELSE                  : &str = "ELSE";
// functions
const FUNC_WRITELN              : &str = "WRITELN";
// errors
const PANIC_SYNTAX              : &str = "Invalid syntax";
const PANIC_TYPE_DECLARATION    : &str = "Invalid type declaration";
const PANIC_ARRAY               : &str = "Invalid array type";
const PANIC_VAR_NOT_DECLARAED   : &str = "Variable not declared";
const PANIC_COMPILE             : &str = "Could not compile";
const PANIC_WRITE               : &str = "Could not write to file";
const PANIC_READ                : &str = "Could not read from file";
const PANIC_FILE                : &str = "No source file provided";
const PANIC_EVAL                : &str = "Could not evaluate";
// format
const FORMAT_SPACE              : &str = " ";
const FORMAT_TAB                : &str = "    ";
const FORMAT_NEWLINE            : &str = "\n";
// opcodes
const WASM_EQUAL                : &str = ".eq";
const WASM_NOT_EQUAL            : &str = ".ne";
const WASM_TRUNCATE             : &str = ".trunc";
const WASM_PLUS                 : &str = ".add";
const WASM_MINUS                : &str = ".sub";
const WASM_MULTIPLY             : &str = ".mul";
const WASM_DIVIDE               : &str = ".div";
const WASM_INTEGER_DIV          : &str = ".div_s";
const WASM_INTEGER_MOD          : &str = ".rem_s";
const WASM_CONVERT              : &str = ".convert";
const WASM_GREATER_THAN         : &str = ".gt";
const WASM_GREATER_EQUAL        : &str = ".ge";
const WASM_LESS_THAN            : &str = ".lt";
const WASM_LESS_EQUAL           : &str = ".le";
const WASM_NEGATION             : &str = ".neg";
const WASM_CONSTANT             : &str = ".const";
const WASM_VARIABLE             : &str = "get_local";
const WASM_ASSIGNMENT           : &str = "set_local";
const WASM_DECLARATION          : &str = "param";
const WASM_RESULT               : &str = "result";
const WASM_EXPORT               : &str = "export";
const WASM_FUNCTION             : &str = "func";
const WASM_MODULE               : &str = "module";
const WASM_BLOCK                : &str = "block";
const WASM_LOOP                 : &str = "loop";
const WASM_BREAK                : &str = "br";
const WASM_BREAK_IF             : &str = "br_if";
const WASM_IF                   : &str = "if";
const WASM_THEN                 : &str = "then";
const WASM_ELSE                 : &str = "else";
// files
const WASM_WAT                  : &str = ".wat";
const WASM_JS                   : &str = ".js";

// lexer
// ----------------------------------------------------
pub struct Lexer {
    text            : String,
    position        : i32,
    current_char    : Option<char>,
}

impl Lexer {
    // new
    // return : Lexer
    fn new(text: String) -> Lexer {
        let mut lexer = Lexer {
            text            : text,
            position        : 0,
            current_char    : None,
        };
        if lexer.text.len() > 0 {
            lexer.current_char = Some(lexer.text.as_bytes()[0] as char);
        }
        return lexer;
    }

    // look_ahead
    // return : Option<char>
    fn look_ahead(&mut self) -> Option<char> {
        // next position in text
        let next_position = self.position as i32 + 1;
        if next_position > self.text.len() as i32 - 1 {
            // end of file
            return None;
        } else {
            return Some(self.text.as_bytes()[next_position as usize] as char);
        }
    }

    // increment
    fn increment(&mut self) {
        self.position += 1;
        // no more text input
        if self.position > self.text.len() as i32 - 1 {
            // end of file
            self.current_char = None;
        }
        // otherwise
        else {
            // usize is target machine architecture specific
            self.current_char = Some(self.text.as_bytes()[self.position as usize] as char);
        }
    }

    // skip_whitespace
    fn skip_whitespace(&mut self) {
        while let Some(_char) = self.current_char {
            if _char.is_whitespace() {
                self.increment();
            } else {
                break;
            }
        }
    }

    // skip_comment
    fn skip_comment(&mut self) {
        while let Some(_char) = self.current_char {
            if _char != CHAR_RCUR {
                self.increment();
            } else {
                // closing curly brace
                self.increment();
                break;
            }
        }
    }

    // number
    // return : Token
    fn number(&mut self) -> Token {
        let mut string = String::new();
        while let Some(_char) = self.current_char {
            // check if char is digit
            if _char.is_digit(10) {
                // push to string
                string.push(_char);
                self.increment();
            } else {
                break;
            }
        }
        // two dots is range
        if self.current_char == Some(CHAR_DOT) && self.look_ahead() == Some(CHAR_DOT) {
            self.increment();
            self.increment();
            let mut end = String::new();
            while let Some(_char) = self.current_char {
                // check if char is digit
                if _char.is_digit(10) {
                    // push to string
                    end.push(_char);
                    self.increment();
                } else {
                    break;
                }
            }
            return Token::RANGE(string.parse::<i32>().unwrap(), end.parse::<i32>().unwrap());
        }
        // one dot is floating-point
        if self.current_char == Some(CHAR_DOT) {
            // push to string
            string.push(CHAR_DOT);
            // increment
            self.increment();
            
            while let Some(_char) = self.current_char {
                // check if char is digit
                if _char.is_digit(10) {
                    // push to string
                    string.push(_char);
                    // increment
                    self.increment();
                } else {
                    break;
                }
            }
            return Token::REAL(string.parse::<f64>().unwrap());
        }
        return Token::INTEGER(string.parse::<i32>().unwrap());
    }

    // id
    // return : Token
    fn id(&mut self) -> Token {
        let mut string = String::new();
        while let Some(_char) = self.current_char {
            // check if char is alphabetic
            if _char.is_alphabetic() || _char.is_digit(10) || _char == CHAR_UNDERSCORE {
                // push to string
                string.push(_char);
                self.increment();
            } else {
                break;
            }
        }
        // keywords
        match string.to_uppercase().as_str() {
            KEY_PROGRAM => {
                return Token::PROGRAM;
            },
            KEY_VAR => {
                return Token::VAR;
            },
            KEY_DIV => {
                return Token::INTEGER_DIV;
            },
            KEY_MOD => {
                return Token::INTEGER_MOD;
            },
            KEY_BEGIN => {
                return Token::BEGIN;
            },
            KEY_END => {
                return Token::END;
            },
            // types
            KEY_INTEGER => {
                return Token::TYPE_SPEC(Type::INTEGER);
            },
            KEY_LONGINT => {
                return Token::TYPE_SPEC(Type::INTEGER);
            },
            KEY_REAL => {
                return Token::TYPE_SPEC(Type::REAL);
            },
            // boolean
            KEY_TRUE => {
                return Token::TRUE;
            },
            KEY_FALSE => {
                return Token::FALSE;
            },
            // structured
            KEY_PACKED => {
                return Token::PACKED;
            },
            KEY_ARRAY => {
                return Token::ARRAY;
            },
            KEY_OF => {
                return Token::OF;
            },
            KEY_WHILE => {
                return Token::WHILE;
            },
            KEY_DO => {
                return Token::DO;
            },
            // KEY_BREAK => {
            //     return Token::BREAK;
            // },
            KEY_IF => {
                return Token::IF;
            },
            KEY_THEN => {
                return Token::THEN;
            },
            KEY_ELSE => {
                return Token::ELSE;
            },
            // functions
            FUNC_WRITELN => {
                return Token::WRITELN;
            },
            // otherwise
            _ => Token::ID(string)
        }
    }

    // get_next_token
    // return : Token
    fn get_next_token(&mut self) -> Token {
        while let Some(_char) = self.current_char {
            if DEBUG_SHOW_CHAR { println!("{:?}", _char); }
            // whitespace
            if _char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }
            // comment
            if _char == CHAR_LCUR {
                self.increment();
                self.skip_comment();
                continue;
            }
            // identifier (variable or keyword)
            if _char.is_alphabetic() {
                return self.id();
            }
            // number (INTEGER or REAL)
            // radix/ base of 10 is decimal number
            if _char.is_digit(10) {
                return self.number();
            }
            // assignment
            if _char == CHAR_COLON && self.look_ahead() == Some(CHAR_EQUAL) {
                self.increment();
                self.increment();
                return Token::ASSIGN;
            }
            // not equal
            if _char == CHAR_LESS_THAN && self.look_ahead() == Some(CHAR_GREATER_THAN) {
                self.increment();
                self.increment();
                return Token::NOT_EQUAL;
            }
            // less than or equal
            if _char == CHAR_LESS_THAN && self.look_ahead() == Some(CHAR_EQUAL) {
                self.increment();
                self.increment();
                return Token::LESS_EQUAL;
            }
            // greater than or equal
            if _char == CHAR_GREATER_THAN && self.look_ahead() == Some(CHAR_EQUAL) {
                self.increment();
                self.increment();
                return Token::GREATER_EQUAL;
            }
            // colon
            if _char == CHAR_COLON {
                self.increment();
                return Token::COLON;
            }
            // semicolon
            if _char == CHAR_SEMICOLON {
                self.increment();
                return Token::SEMICOLON;
            }
            // comma
            if _char == CHAR_COMMA {
                self.increment();
                return Token::COMMA;
            }
            // dot
            if _char == CHAR_DOT {
                // end of program (discard everything after)
                self.current_char = None;
                return Token::DOT;
            }
            // operators
            match _char {
                CHAR_PLUS => {
                    self.increment();
                    return Token::PLUS;
                },
                CHAR_MINUS => {
                    self.increment();
                    return Token::MINUS;
                },
                CHAR_MULTIPLY => {
                    self.increment();
                    return Token::MULTIPLY;
                },
                CHAR_DIVIDE => {
                    self.increment();
                    return Token::DIVIDE;
                },
                CHAR_LPAR => {
                    self.increment();
                    return Token::LPAR;
                },
                CHAR_RPAR => {
                    self.increment();
                    return Token::RPAR;
                },
                CHAR_LBRA => {
                    self.increment();
                    return Token::LBRA;
                },
                CHAR_RBRA => {
                    self.increment();
                    return Token::RBRA;
                },
                CHAR_EQUAL => {
                    self.increment();
                    return Token::EQUAL;
                },
                CHAR_GREATER_THAN => {
                    self.increment();
                    return Token::GREATER_THAN;
                },
                CHAR_LESS_THAN => {
                    self.increment();
                    return Token::LESS_THAN;
                },
                _ => panic!(format!("{} : {}", _char.to_string(), PANIC_SYNTAX))
            }
        }
        // otherwise
        Token::EOF
    }
}
// ----------------------------------------------------

// IR: AST
// ----------------------------------------------------
#[derive(Clone, Debug)]
struct AST {
    token           : Token,
    children        : Vec<AST>,
}

impl AST {
    // new
    // return : AST
    fn new(token: Token, children: Vec<AST>) -> AST {
        AST {
            token       : token,
            children    : children
        }
    }
}
// ----------------------------------------------------

// parser
// ----------------------------------------------------
pub struct Parser {
    lexer           : Lexer,
    current_token   : Option<Token>,
    symbol_table    : Vec<(Token, Type)>,
    assign_table    : Vec<(Token, AST)>,
    result_type     : bool,
}

impl Parser {
    // new
    // return : Parser
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer           : lexer,
            current_token   : None,
            symbol_table    : vec![],
            assign_table    : vec![],
            result_type     : false,
        };
        parser.current_token = Some(parser.lexer.get_next_token());
        return parser
    }

    // eat (consume token)
    fn eat(&mut self, token: Token) {
        if DEBUG_SHOW_TOKEN { println!("{:?}", token); }
        // clone used to deep copy value
        if token == self.current_token.clone().unwrap() {
            self.current_token = Some(self.lexer.get_next_token());
        } else {
            panic!(format!("{:?} : {}", token, PANIC_SYNTAX))
        }
    }

    // type_spec
    // return : AST
    fn type_spec(&mut self) -> AST {
        /*
            type_spec : INTEGER | REAL
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::TYPE_SPEC(Type::INTEGER) => {
                // INTEGER
                self.eat(Token::TYPE_SPEC(Type::INTEGER));
                return AST::new(token, vec![]);
            },
            Token::TYPE_SPEC(Type::REAL) => {
                // REAL
                self.eat(Token::TYPE_SPEC(Type::REAL));
                return AST::new(token, vec![]);
            },
            _ => panic!(format!("{:?} : {}", token, PANIC_TYPE_DECLARATION))
        }
    }

    // empty
    // return : AST
    fn empty(&mut self) -> AST {
        /*
            empty : 
        */
        return AST::new(Token::EMPTY, vec![])
    }

    // variable
    // return : AST
    fn variable(&mut self) -> AST {
        /*
            variable : ID (LBRA simple_expression RBRA)?
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::ID(_string) => {
                let string = _string.clone();
                // ID
                self.eat(Token::ID(_string));
                // indexed variable
                if self.current_token.clone().unwrap() == Token::LBRA {
                    // LBRA
                    self.eat(Token::LBRA);
                    // simple_expression
                    let simple_expression = self.simple_expression();
                    let mut evaluator = Evaluator::new(self.symbol_table.clone(), self.assign_table.clone());
                    let index = evaluator.evaluate(&simple_expression);
                    let children = AST::new(Token::INTEGER(index as i32), vec![]);
                    // RBRA
                    self.eat(Token::RBRA);
                    return AST::new(Token::ID(string), vec![children]);
                }
                // otherwise
                return AST::new(Token::ID(string), vec![]);
            },
            _ => panic!(format!("{:?} : {}", token, PANIC_SYNTAX))
        }
    }

    // array_type
    // return : AST
    fn array_type(&mut self, variable_nodes: Vec<AST>) -> AST {
        /*
            array_type : ARRAY LBRA RANGE (COMMA RANGE)* RBRA OF type_spec 
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::ARRAY => {
                // ARRAY
                self.eat(Token::ARRAY);
                // LBRA
                self.eat(Token::LBRA);
                // RANGE
                let mut ranges = vec![];
                let token = self.current_token.clone().unwrap();
                match token {
                    Token::RANGE(_start, _end) => {
                        self.eat(Token::RANGE(_start, _end));
                        ranges.push((_start, _end));
                    },
                    _ => panic!(format!("{:?} : {}", token, PANIC_ARRAY))
                }

                // (COMMA RANGE)*
                // while self.current_token == Some(Token::COMMA) {
                //     self.eat(Token::COMMA);
                //     token = self.current_token.clone().unwrap();
                //     match token {
                //         Token::RANGE(_start, _end) => {
                //             self.eat(Token::RANGE(_start, _end));
                //             ranges.push((_start, _end));
                //         },
                //         _ => panic!(format!("{:?} : {}", token, PANIC_ARRAY))
                //     }
                // }

                // RBRA
                self.eat(Token::RBRA);
                // OF
                self.eat(Token::OF);
                // type_spec
                let mut node = self.type_spec();
                // build AST
                for mut _variable in variable_nodes {
                    // add to symbol_table
                    match &node.token.clone() {
                        Token::TYPE_SPEC(_type) => {
                            self.symbol_table.push((_variable.token.clone(), _type.clone()));
                        },
                        _ => panic!(format!("{:?} : {}", _variable, PANIC_SYNTAX))
                    }
                    // TODO: consider changing to dimensions or remove
                    for range in &ranges {
                        for _ix in range.0 as i32 .. range.1 as i32 + 1 {
                            match &_variable.token {
                                Token::ID(_string) => {
                                    let id = _string.clone();
                                    let index = AST::new(Token::INTEGER(_ix), vec![]);
                                    let index_id = AST::new(Token::ID(id), vec![index]);
                                    node.children.push(index_id);
                                },
                                _ => panic!(format!("{:?} : {}", _variable, PANIC_SYNTAX))
                            }
                        }
                    }
                }
                return node
            },
            _ => panic!(format!("{:?} : {}", token, PANIC_SYNTAX))
        }
    }

    // structured_type
    // return : AST
    fn structured_type(&mut self, variable_nodes: Vec<AST>) -> AST {
        /*
            structured_type : (PACKED)? (array_type)
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::PACKED => {
                self.eat(Token::PACKED);
                return self.structured_type(variable_nodes);
            },
            Token::ARRAY => {
                return self.array_type(variable_nodes);
            },
            _ => panic!(format!("{:?} : {}", token, PANIC_SYNTAX))
        }
    }

    // variable_declaration
    // return : AST
    fn variable_declaration(&mut self) -> AST {
        /*
            variable_declaration : ID (COMMA ID)* COLON (type_spec | structured_type)
        */

        // ID
        let mut variable_nodes = vec![self.variable()];
        // (COMMA ID)*
        while self.current_token == Some(Token::COMMA) {
            self.eat(Token::COMMA);
            variable_nodes.push(self.variable());
        }
        // COLON
        self.eat(Token::COLON);
        // (type_spec | structured_type)
        let token = self.current_token.clone().unwrap();
        match token {
            // type_spec
            Token::TYPE_SPEC(_type) => {
                let mut node = self.type_spec();
                for _variable in variable_nodes {
                    self.symbol_table.push((_variable.token.clone(), _type.clone()));
                    node.children.push(_variable);
                }
                return node
            },
            // structured_type
            Token::PACKED | Token::ARRAY => {
                return self.structured_type(variable_nodes);
            },
            _ => panic!(format!("{:?} : {}", token, PANIC_TYPE_DECLARATION))
        }
    }

    // declarations
    // return : AST
    fn declarations(&mut self) -> AST {
        /*
            declarations : VAR (variable_declaration SEMICOLON)+ | empty
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::VAR => {
                let mut variable_declarations = vec![];
                // VAR
                self.eat(Token::VAR);
                // (variable_declaration SEMI)+
                while let Some(_token) = &self.current_token {
                    match _token {
                        Token::ID(_string) => {
                            variable_declarations.push(self.variable_declaration());
                            self.eat(Token::SEMICOLON);
                        },
                        _ => break
                    }
                }
                let node = AST::new(Token::VAR, variable_declarations);
                return node
            },
            _ => return self.empty()
        }
    }

    // assignment_statement
    // return : AST
    fn assignment_statement(&mut self) -> AST {
        /*
            assignment_statement : variable ASSIGN simple_expression
        */
        let node = self.variable();
        for symbol in &self.symbol_table {
            if node.token == symbol.0 {
                match self.current_token {
                    Some(Token::ASSIGN) => {
                        // ASSIGN
                        self.eat(Token::ASSIGN);
                        let simple_expression = self.simple_expression();
                        let children: Vec<AST> = vec![node.clone(), simple_expression.clone()];
                            
                        // TEST
                        self.assign_table.push((node.token, simple_expression));
                        
                        self.result_type = false;
                        // new branch
                        return AST::new(Token::ASSIGN, children);
                    }
                    _ => panic!(format!("{:?} : {}", node.token, PANIC_SYNTAX))
                }
            }
        }
        panic!(format!("{:?} : {}", node, PANIC_VAR_NOT_DECLARAED))
    }

    // factor
    // return : AST
    fn factor(&mut self) -> AST {
        /*
            factor : PLUS factor | MINUS factor | INTEGER | REAL | LPAR expression RPAR | variable
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::PLUS => {
                // PLUS
                self.eat(Token::PLUS);
                return AST::new(token, vec![self.factor()]);
            },
            Token::MINUS => {
                // MINUS
                self.eat(Token::MINUS);
                return AST::new(token, vec![self.factor()]);
            },
            Token::INTEGER(_int) => {
                // INTEGER
                self.eat(Token::INTEGER(_int));
                self.result_type = true;
                return AST::new(token, vec![]);
            },
            Token::REAL(_float) => {
                // REAL
                self.eat(Token::REAL(_float));
                self.result_type = true;
                return AST::new(token, vec![]);
            },
            Token::LPAR => {
                // LPAR
                self.eat(Token::LPAR);
                // expression
                let node = self.expression();
                // RPAR
                self.eat(Token::RPAR);
                return node;
            },
            _ => {
                let node = self.variable();
                return node;
            }
        }
    }

    // term
    // return : AST
    fn term(&mut self) -> AST {
        /*
            term : factor ((MULTIPLY | DIVIDE | INTEGER_DIV | INTEGER_MOD) factor)*
        */
        let mut node = self.factor();
        while self.current_token == Some(Token::MULTIPLY) || self.current_token == Some(Token::DIVIDE) || self.current_token == Some(Token::INTEGER_DIV) || self.current_token == Some(Token::INTEGER_MOD) {
            match self.current_token {
                Some(Token::MULTIPLY) => {
                    // MULTIPLY
                    self.eat(Token::MULTIPLY);
                    let children: Vec<AST> = vec![node, self.factor()];
                    node = AST::new(Token::MULTIPLY, children);
                },
                Some(Token::DIVIDE) => {
                    // DIVIDE
                    self.eat(Token::DIVIDE);
                    let children: Vec<AST> = vec![node, self.factor()];
                    node = AST::new(Token::DIVIDE, children);
                },
                Some(Token::INTEGER_DIV) => {
                    // INTEGER_DIV
                    self.eat(Token::INTEGER_DIV);
                    let children: Vec<AST> = vec![node, self.factor()];
                    node = AST::new(Token::INTEGER_DIV, children);
                },
                Some(Token::INTEGER_MOD) => {
                    // INTEGER_MOD
                    self.eat(Token::INTEGER_MOD);
                    let children: Vec<AST> = vec![node, self.factor()];
                    node = AST::new(Token::INTEGER_MOD, children);
                },
                _ => panic!(format!("{:?} : {}", self.current_token, PANIC_SYNTAX))
            }
        }
        return node;
    }

    // simple_expression
    // return : AST
    fn simple_expression(&mut self) -> AST {
        /*
            simple_expression : term ((PLUS | MINUS) term)*
        */
        let mut node = self.term();
        while self.current_token == Some(Token::PLUS) || self.current_token == Some(Token::MINUS) {
            match self.current_token {
                Some(Token::PLUS) => {
                    // PLUS
                    self.eat(Token::PLUS);
                    let children: Vec<AST> = vec![node, self.term()];
                    // new branch
                    node = AST::new(Token::PLUS, children);
                },
                Some(Token::MINUS) => {
                    // MINUS
                    self.eat(Token::MINUS);
                    let children: Vec<AST> = vec![node, self.term()];
                    // new branch
                    node = AST::new(Token::MINUS, children);
                },
                _ => panic!(format!("{:?} : {}", self.current_token, PANIC_SYNTAX))
            }
        }
        return node;
    }

    // expression
    // return : AST
    fn expression(&mut self) -> AST {
        /*
            expression : (TRUE | FALSE) | simple_expression ((EQUAL | GREATER_THAN | GREATER_EQUAL | LESS_THAN | LESS_EQUAL | NOT_EQUAL) simple_expression)?
        */

        // boolean
        let mut node = AST::new(Token::EMPTY, vec![]);
        if self.current_token == Some(Token::TRUE) || self.current_token == Some(Token::FALSE) {
            match self.current_token {
                Some(Token::TRUE) => {
                    // TRUE
                    self.eat(Token::TRUE);
                    node = AST::new(Token::TRUE, vec![]);
                },
                Some(Token::FALSE) => {
                    node = AST::new(Token::FALSE, vec![]);
                },
                _ => {}
            }
        // otherwise
        } else {
            node = self.simple_expression();
            //let mut node = self.simple_expression();
            match self.current_token {
                Some(Token::EQUAL) => {
                    // EQUAL
                    self.eat(Token::EQUAL);
                    let children: Vec<AST> = vec![node, self.simple_expression()];
                    // new branch
                    node = AST::new(Token::EQUAL, children);
                },
                Some(Token::GREATER_THAN) => {
                    // GREATER_THAN
                    self.eat(Token::GREATER_THAN);
                    let children: Vec<AST> = vec![node, self.simple_expression()];
                    // new branch
                    node = AST::new(Token::GREATER_THAN, children);
                },
                Some(Token::GREATER_EQUAL) => {
                    // GREATER_EQUAL
                    self.eat(Token::GREATER_EQUAL);
                    let children: Vec<AST> = vec![node, self.simple_expression()];
                    // new branch
                    node = AST::new(Token::GREATER_EQUAL, children);
                },
                Some(Token::LESS_THAN) => {
                    // LESS_THAN
                    self.eat(Token::LESS_THAN);
                    let children: Vec<AST> = vec![node, self.simple_expression()];
                    // new branch
                    node = AST::new(Token::LESS_THAN, children);
                },
                Some(Token::LESS_EQUAL) => {
                    // LESS_EQUAL
                    self.eat(Token::LESS_EQUAL);
                    let children: Vec<AST> = vec![node, self.simple_expression()];
                    // new branch
                    node = AST::new(Token::LESS_EQUAL, children);
                },
                Some(Token::NOT_EQUAL) => {
                    // NOT_EQUAL
                    self.eat(Token::NOT_EQUAL);
                    let children: Vec<AST> = vec![node, self.simple_expression()];
                    // new branch
                    node = AST::new(Token::NOT_EQUAL, children);
                },
                _ => {}
            }
        }
        return node;
    }

    // function_statement
    fn function_statement(&mut self) -> AST {
        /*
            function_statement : (WRITELN) LPAR expression RPAR
        */
        match self.current_token {
            Some(Token::WRITELN) => {
                // WRITELN
                self.eat(Token::WRITELN);
                // LPAR
                self.eat(Token::LPAR);
                let node = AST::new(Token::WRITELN, vec![self.expression()]);
                // RPAR
                self.eat(Token::RPAR);
                self.result_type = true;
                return node
            }
            _ => panic!(format!("{:?} : {}", self.current_token, PANIC_SYNTAX))
        }
    }

    // while_statement
    // return : AST
    fn while_statement(&mut self) -> AST {
        /*
            while_statement : WHILE expression DO statement
        */
        // WHILE
        self.eat(Token::WHILE);
        // expression
        let expression = self.expression();
        // DO
        self.eat(Token::DO);
        // statement
        let statement = self.statement();

        let node = AST::new(Token::WHILE, vec![expression, statement]);
        return node;
    }

    // if_statement
    // return : AST
    fn if_statement(&mut self) -> AST {
        /*
            if_statement : IF expression THEN statement (ELSE statement)?
        */
        // IF
        self.eat(Token::IF);
        // expression
        let expression = self.expression();
        // THEN
        self.eat(Token::THEN);
        // statement
        let statement = self.statement();

        // else statement
        if self.current_token == Some(Token::ELSE) {
            self.eat(Token::ELSE);
            let else_statement = self.statement();
            // node
            let node = AST::new(Token::IF, vec![expression, statement, else_statement]);
            return node;
        // otherwise
        } else {
            let node = AST::new(Token::IF, vec![expression, statement]);
            return node;
        }
    }

    // structured_statement
    // return : AST
    fn structured_statement(&mut self) -> AST {
        /*
            structured_statement : if_statement | while_statement 
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::IF => {
                return self.if_statement();
            },
            Token::WHILE => {
                return self.while_statement();
            },
            _ => panic!(format!("{:?} : {}", self.current_token, PANIC_SYNTAX))
        }
    }

    // statement
    // return : AST
    fn statement(&mut self) -> AST {
        /*
            statement : compound_statement | structured_statement | assignment_statement | function_statement | empty
        */
        let token = self.current_token.clone().unwrap();
        match token {
            Token::BEGIN => {
                return self.compound_statement();
            },
            Token::IF | Token::WHILE => {
                return self.structured_statement();
            },
            Token::ID(_string) => {
                return self.assignment_statement();
            },
            Token::WRITELN => {
                return self.function_statement();
            },
            _ => return self.empty()
        }
    }

    // statement_list
    // return : AST
    fn statement_list(&mut self) -> Vec<AST> {
        /*
            statement_list : statement (SEMICOLON statement)*
        */
        let mut statement_list_nodes = vec![];
        // statement
        let node = self.statement();
        // append to list with nodes
        statement_list_nodes.push(node);
        // (SEMICOLON statement)*
        while self.current_token == Some(Token::SEMICOLON) {
            self.eat(Token::SEMICOLON);
            statement_list_nodes.push(self.statement());
        } 
        return statement_list_nodes;
    }

    // compound_statement
    // return : AST
    fn compound_statement(&mut self) -> AST {
        /*
            compound_statement : BEGIN statement_list END
        */
        // BEGIN
        self.eat(Token::BEGIN);
        // statement_list
        let compound_statement_nodes = self.statement_list();
        // END
        self.eat(Token::END);
        return AST::new(Token::BLOCK, compound_statement_nodes)
    }

    // block
    // return : AST
    fn block(&mut self) -> AST {
        /*
            block : declarations compound_statement
        */
        let declarations_nodes = self.declarations();
        let compound_statement_nodes = self.compound_statement();
        let node = AST::new(Token::BLOCK, vec![declarations_nodes, compound_statement_nodes]);
        return node
    }

    // program
    // return : AST
    fn program(&mut self) -> AST {
        /*
            program : PROGRAM variable SEMICOLON block DOT
        */
        // PROGRAM
        self.eat(Token::PROGRAM);
        // variable (name of program)
        let variable_node = self.variable();
        // SEMICOLON
        self.eat(Token::SEMICOLON);
        // block
        let block_node = self.block();
        let program_node = AST::new(Token::PROGRAM, vec![variable_node, block_node]);
        // DOT
        self.eat(Token::DOT);
        return program_node
    }

    // parse
    // return : AST
    fn parse(&mut self) -> AST {
        //self.simple_expression()
        //self.variable_declaration()
        //self.declarations()
        //self.assignment_statement()
        //self.variable()
        //self.block()
        
        let node = self.program();
        if self.current_token != Some(Token::EOF) {
            panic!(format!("{:?} : {}", self.current_token, PANIC_SYNTAX))
        }
        return node;
    }
}
// ----------------------------------------------------

// evaluator
// ----------------------------------------------------
pub struct Evaluator {
    symbol_table    : Vec<(Token, Type)>,
    assign_table    : Vec<(Token, AST)>,
    // to track tokens
    tokens          : Vec<Token>,
}

impl Evaluator {
    // new
    // return : Evaluator
    fn new(symbol_table: Vec<(Token, Type)>, assign_table: Vec<(Token, AST)>) -> Evaluator {
        let evaluator = Evaluator {
            symbol_table    : symbol_table,
            assign_table    : assign_table,
            tokens          : vec![],
        };
        return evaluator;
    }

    // eval_number
    // return : f64
    fn eval_number(&mut self, node: &AST) -> f64 {
        match node.token {
            Token::INTEGER(_int) => {
                self.tokens.push(node.token.clone());
                return _int as f64;
            },
            Token::REAL(_float) => {
                self.tokens.push(node.token.clone());
                return _float;
            },
            _ => panic!("{} : {:?}", PANIC_EVAL, node)
        }
    }

    // eval_binary_operator
    // return : f64
    fn eval_binary_operator(&mut self, node: &AST) -> f64 {
        let left_value = self.evaluate(&node.children[0]);
        let right_value = self.evaluate(&node.children[1]);
        match node.token {
            Token::PLUS => {
                self.tokens.push(node.token.clone());
                return left_value + right_value;
            },
            Token::MINUS => {
                self.tokens.push(node.token.clone());
                return left_value - right_value;
            },
            Token::MULTIPLY => {
                self.tokens.push(node.token.clone());
                return left_value * right_value;
            },
            Token::DIVIDE | Token::INTEGER_DIV => {
                self.tokens.push(node.token.clone());
                return left_value / right_value;
            },
            Token::INTEGER_MOD => {
                self.tokens.push(node.token.clone());
                return left_value % right_value;

            },
            _ => panic!("{} : {:?}", PANIC_EVAL, node)
        }
    }

    // evaluate
    // return : f64
    fn evaluate(&mut self, node: &AST) -> f64 {
        match &node.token {
            Token::INTEGER(_) | Token::REAL(_) => {
                return self.eval_number(node);
            },
            Token::PLUS | Token::MINUS | Token::MULTIPLY | Token::DIVIDE | Token::INTEGER_DIV | Token::INTEGER_MOD => {
                return self.eval_binary_operator(node);
            },
            Token::ID(_string) => {
                if self.assign_table.len() as i32 > 0 {
                    for symbol in &self.assign_table {
                        if node.token == symbol.0 {
                            let node = symbol.1.clone();
                            let value = self.evaluate(&node);
                            return value;
                        }
                    }
                }
                panic!("{} : {:?}", PANIC_EVAL, node)
            }
            _ => panic!("{} : {:?}", PANIC_EVAL, node)
        }
    }
}
// ----------------------------------------------------

// emitter
// ----------------------------------------------------
pub struct Emitter {
    parser          : Parser,
    file            : BufWriter<File>,
    tab_pos         : i32,
}

impl Emitter {
    // new
    // return : Emitter
    fn new(parser: Parser, file: BufWriter<File>) -> Emitter {
        let emitter = Emitter {
            parser          : parser,
            file            : file,
            tab_pos         : 0,
        };
        return emitter;
    }

    // visit_number
    fn visit_number(&mut self, node: &AST) {
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; number", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match node.token {
            Token::INTEGER(_int) => {
                match self.file.write_all(format!("{}{}({}{} {:?})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_CONSTANT, _int).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {},
                }
            },
            Token::REAL(_float) => {
                match self.file.write_all(format!("{}{}({}{} {:?})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_CONSTANT, _float).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {},
                }
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_boolean
    fn visit_boolean(&mut self, node: &AST) {
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; boolean", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match node.token {
            Token::TRUE => {
                match self.file.write_all(format!("{}{}({}{} {:?})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_CONSTANT, 1).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {},
                }
            },
            Token::FALSE => {
                match self.file.write_all(format!("{}{}({}{} {:?})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_CONSTANT, 0).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {},
                }
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_unary_operator
    fn visit_unary_operator(&mut self, node: &AST) {
        self.visit(&node.children[0]);
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; unary operator", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match node.token {
            Token::PLUS => {
            },
            Token::MINUS => {
                match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_NEGATION).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {},
                }
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_binary_operator
    fn visit_binary_operator(&mut self, node: &AST) {
        if node.children.len() as i32 != 2 {
            self.visit_unary_operator(node);
        } else {
            // type checker for integer operators
            match node.token {
                Token::INTEGER_DIV | Token::INTEGER_MOD => {
                    // first child, convert f64 to i32
                    self.visit(&node.children[0]);
                    match self.file.write_all(format!("{}{}({}{}_{}_s)", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_TRUNCATE, NTYPE_REAL).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                    // second child, convert f64 to i32
                    self.visit(&node.children[1]);
                    match self.file.write_all(format!("{}{}({}{}_{}_s)", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_TRUNCATE, NTYPE_REAL).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                }
                // otherwise, visit children
                _ => {
                    self.visit(&node.children[0]);
                    self.visit(&node.children[1]);
                }
            }
            if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; binary operator", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
            // match operator
            match node.token {
                Token::PLUS => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_PLUS).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::MINUS => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_MINUS).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::MULTIPLY => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_MULTIPLY).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::DIVIDE => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_DIVIDE).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::INTEGER_DIV => {
                    match self.file.write_all(format!("{}{}({}{}){}{}({}{}_{}_s)", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_INTEGER_DIV, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_CONVERT, NTYPE_INTEGER).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::INTEGER_MOD => {
                    match self.file.write_all(format!("{}{}({}{}){}{}({}{}_{}_s)", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_INTEGER_MOD, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_CONVERT, NTYPE_INTEGER).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::EQUAL => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_EQUAL).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::GREATER_THAN => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_GREATER_THAN).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::GREATER_EQUAL => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_GREATER_EQUAL).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::LESS_THAN => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_LESS_THAN).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::LESS_EQUAL => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_LESS_EQUAL).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                Token::NOT_EQUAL => {
                    match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_NOT_EQUAL).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                },
                _ => panic!("{} : {:?}", PANIC_COMPILE, node)
            }
        }
    }

    // visit_variable
    fn visit_variable(&mut self, node: &AST) {
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; variable reference", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match &node.token {
            Token::ID(_string) => {
                // indexed variables
                if node.children.len() as i32 > 0 {
                    match node.children[0].token {
                        Token::INTEGER(_int) => {
                            match self.file.write_all(format!("{}{}({} ${}_{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_VARIABLE, _string, _int).as_bytes()) {
                                Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                                Ok(_) => {},
                            }
                        },
                        _ => panic!("{} : {:?}", PANIC_COMPILE, node)
                    }
                // otherwise
                } else {
                    match self.file.write_all(format!("{}{}({} ${})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_VARIABLE, _string).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {},
                    }
                }
            }
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_while
    fn visit_while(&mut self, node: &AST) {
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; while statement", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match &node.token {
            Token::WHILE => {
                match self.file.write_all(format!("{}{}({}{}{}({}", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_BLOCK, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 1), WASM_LOOP).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {
                        // statement
                        self.tab_pos += 2;
                        self.visit(&node.children[1]);
                        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; conditional statement", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
                        match self.file.write_all(format!("{}{}({} 1", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_BREAK_IF).as_bytes()) {
                            Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                            Ok(_) => {
                                self.tab_pos += 1;
                                // swap booleans to work as condition in while loop
                                let token = node.children[0].token.clone();
                                let children = node.children[0].children.clone();
                                // expression
                                match token {
                                    Token::EQUAL => {
                                        let expression = AST::new(Token::NOT_EQUAL, children);
                                        self.visit(&expression);
                                    },
                                    Token::GREATER_THAN => {
                                        let expression = AST::new(Token::LESS_EQUAL, children);
                                        self.visit(&expression);
                                    },
                                    Token::GREATER_EQUAL => {
                                        let expression = AST::new(Token::LESS_THAN, children);
                                        self.visit(&expression);
                                    },
                                    Token::LESS_THAN => {
                                        let expression = AST::new(Token::GREATER_EQUAL, children);
                                        self.visit(&expression);
                                    },
                                    Token::LESS_EQUAL => {
                                        let expression = AST::new(Token::GREATER_THAN, children);
                                        self.visit(&expression);
                                    },
                                    Token::NOT_EQUAL => {
                                        let expression = AST::new(Token::EQUAL, children);
                                        self.visit(&expression);
                                    },
                                    Token::TRUE => {
                                        let expression = AST::new(Token::FALSE, vec![]);
                                        self.visit(&expression);
                                    },
                                    Token::FALSE => {
                                        let expression = AST::new(Token::TRUE, vec![]);
                                        self.visit(&expression);
                                    },
                                    _ => {}
                                }
                                self.tab_pos -= 1;
                                self.file.write_all(format!("{}{}){}{}({} 0){}{}){}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_BREAK, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 2)).as_bytes()).expect(PANIC_WRITE);
                                self.tab_pos -= 2;
                            }
                        }
                    }
                }
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_if
    fn visit_if(&mut self, node: &AST) {
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; if statement", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match &node.token {
            Token::IF => {
                match self.file.write_all(format!("{}{}({}{}{}({}{}{}({} {})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_IF, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 1), WASM_BLOCK, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 2), WASM_RESULT, NTYPE_INTEGER).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => {
                        // statement
                        self.tab_pos += 2;
                        self.visit(&node.children[0]);
                        //if OUTPUT_VERBOSE { self.file.write_all(format!("{}{}){}{};; conditional statement", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1)).as_bytes()).expect(PANIC_WRITE); };
                        // expression
                        match self.file.write_all(format!("{}{}){}{}({}", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1), WASM_THEN).as_bytes()) {
                            Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                            Ok(_) => {
                                //self.tab_pos += 1;
                                let token = node.children[1].token.clone();
                                let children = node.children[1].children.clone();
                                // expression
                                let expression = AST::new(token, children);
                                self.visit(&expression);
                                //self.tab_pos -= 1;
                                self.file.write_all(format!("{}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1)).as_bytes()).expect(PANIC_WRITE);
                                //self.tab_pos -= 2;
                            }
                        }
                        // else expression
                        if node.children.len() as i32 == 3 {
                            match self.file.write_all(format!("{}{}({}", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1), WASM_ELSE).as_bytes()) {
                                Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                                Ok(_) => {
                                    //self.tab_pos += 1;
                                    let token = node.children[2].token.clone();
                                    let children = node.children[2].children.clone();
                                    // expression
                                    let expression = AST::new(token, children);
                                    self.visit(&expression);
                                    self.tab_pos -= 1;
                                    self.file.write_all(format!("{}{}){}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1)).as_bytes()).expect(PANIC_WRITE);
                                    self.tab_pos -= 1;
                                }
                            }
                        }
                    }
                }
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_assign
    fn visit_assign(&mut self, node: &AST) {
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; assignment statement", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        match &node.children[0].token {
            Token::ID(_string) => {
                // indexed variables
                if node.children[0].children.len() as i32 > 0 {
                    match node.children[0].children[0].token {
                        Token::INTEGER(_int) => {
                            match self.file.write_all(format!("{}{}({} ${}_{}", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_ASSIGNMENT, _string, _int).as_bytes()) {
                                Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                                Ok(_) => {
                                    self.tab_pos += 1;
                                    self.visit(&node.children[1]);
                                    self.tab_pos -= 1;
                                    self.file.write_all(format!("{}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE);
                                }
                            }
                        },
                        _ => panic!("{} : {:?}", PANIC_COMPILE, node)
                    }
                // otherwise
                } else {
                    match self.file.write_all(format!("{}{}({} ${}", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_ASSIGNMENT, _string).as_bytes()) {
                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                        Ok(_) => {
                            self.tab_pos += 1;
                            self.visit(&node.children[1]);
                            self.tab_pos -= 1;
                            self.file.write_all(format!("{}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE);
                        }
                    }
                }
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit_variable_declaration
    fn visit_variable_declaration(&mut self, node: &AST) {
        for _child in &node.children {
            let token = &_child.token;
            match token {
                Token::TYPE_SPEC(Type::INTEGER) => {
                    for _id in &_child.children {
                        match &_id.token {
                            Token::ID(_string) => {
                                // indexed variables
                                if _id.children.len() as i32 > 0 {
                                    for _child in &_id.children {
                                        match _child.token {
                                            Token::INTEGER(_int) => {
                                                match self.file.write_all(format!("{}{}({} ${}_{} {})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_DECLARATION, _string, _int, NTYPE_REAL).as_bytes()) {
                                                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, token, why),
                                                    Ok(_) => {},
                                                }
                                            },
                                            _ => panic!("{} : {:?}", PANIC_COMPILE, token),
                                        }

                                    }
                                // otherwise
                                } else {
                                    match self.file.write_all(format!("{}{}({} ${} {})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_DECLARATION, _string, NTYPE_REAL).as_bytes()) {
                                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                                        Ok(_) => {},
                                    }
                                }
                            },
                           _ => panic!("{} : {:?}", PANIC_COMPILE, node)
                        }
                    }
                },
                Token::TYPE_SPEC(Type::REAL) => {
                    for _id in &_child.children {
                        match &_id.token {
                            Token::ID(_string) => {
                                match self.file.write_all(format!("{}{}({} ${} {})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_DECLARATION, _string, NTYPE_REAL).as_bytes()) {
                                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                                    Ok(_) => {},
                                }
                            },
                           _ => panic!("{} : {:?}", PANIC_COMPILE, _id)
                        }
                    }
                },
                _ => {}
            }
        }
        // declare result type (if any)
        if self.parser.result_type {
            //println!("{:?}", self.parser.result_type);
            match self.file.write_all(format!("{}{}({} {}){}{};; body", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_RESULT, NTYPE_REAL, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()) {
                Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                Ok(_) => {},
            }
        }
    }

    // visit_program
    fn visit_program(&mut self, node: &AST) {
        match &node.children[0].token {
            Token::ID(_string) => {
                match self.file.write_all(format!("{}({}{}{}({}{}{};; signature{}{}({} \"{}\")", FORMAT_NEWLINE, WASM_MODULE, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 1), WASM_FUNCTION, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 2), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 2), WASM_EXPORT, _string).as_bytes()) {
                    Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                    Ok(_) => { 
                        self.tab_pos += 2;

                        // check if next is variable declaration
                        //println!("{:?}", &node.children[1].children[0].token);
                        match &node.children[1].children[0].token {
                            Token::VAR => {}
                            _ => {
                                // declare result type (if any)
                                if self.parser.result_type {
                                    //println!("{:?}", self.parser.result_type);
                                    match self.file.write_all(format!("{}{}({} {}){}{};; body", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_RESULT, NTYPE_REAL, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()) {
                                        Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                                        Ok(_) => {},
                                    }
                                }
                            }
                        }
                        // visit next
                        self.visit(&node.children[1]);
                        self.tab_pos -= 1;
                        self.file.write_all(format!("{}{}){}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize - 1)).as_bytes()).expect(PANIC_WRITE);
                        self.tab_pos -= 1;
                    }
                }
            },
           _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // visit
    fn visit(&mut self, node: &AST) {
        match &node.token {
            Token::INTEGER(_int) => {
                self.visit_number(node);
            },
            Token::REAL(_float) => {
                self.visit_number(node);
            },
            Token::TRUE | Token::FALSE => {
                self.visit_boolean(node);
            },
            Token::PLUS 
                | Token::MINUS 
                | Token::MULTIPLY 
                | Token::DIVIDE 
                | Token::INTEGER_DIV 
                | Token::INTEGER_MOD 
                | Token::EQUAL 
                | Token::GREATER_THAN
                | Token::GREATER_EQUAL 
                | Token::LESS_THAN 
                | Token::LESS_EQUAL => {
                
                self.visit_binary_operator(node);
            },
            Token::ASSIGN => {
                self.visit_assign(node);
            },
            Token::WHILE => {
                self.visit_while(node);
            },
            Token::IF => {
                self.visit_if(node);
            },
            Token::ID(_string) => {
                self.visit_variable(node);
            },
            Token::WRITELN => {
                self.visit(&node.children[0]);
            },
            Token::VAR => {
                self.visit_variable_declaration(node);
            },
            Token::BLOCK => {
                // block can have multiple children
                for _child in &node.children {
                    self.visit(_child);
                }
            },
            Token::PROGRAM => {
                self.visit_program(node);
            },
            Token::EMPTY => {
            },
            _ => panic!("{} : {:?}", PANIC_COMPILE, node)
        }
    }

    // compile
    fn compile(&mut self) {
        let tree = self.parser.parse();
        if DEBUG_SHOW_TREE { println!("{:?}", tree); };
        if DEBUG_SHOW_SYMBOL_TABLE { println!("{:?}", self.parser.symbol_table); };
        if DEBUG_SHOW_ASSIGNMENT_TABLE { println!("{:?}", self.parser.assign_table); };

        self.file.write_all(format!(";; this file is generated").as_bytes()).expect(PANIC_WRITE);
        self.visit(&tree);
    }
}
// ----------------------------------------------------
// debugging
static DEBUG_SHOW_CHAR: bool = false;
static DEBUG_SHOW_TOKEN: bool = false;
static DEBUG_SHOW_TREE: bool = false;
static DEBUG_SHOW_SYMBOL_TABLE: bool = false;
static DEBUG_SHOW_ASSIGNMENT_TABLE: bool = false;
// output
static OUTPUT_VERBOSE: bool = true;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut source_file = String::new();
    
    if args.len() as i32 > 1 {
        source_file = String::from(&args[1]);
        
        let input_string = String::from(&source_file);
        let file_name: Vec<&str> = input_string.split('.').collect();

        // read input file
        let mut input = String::new();
        match BufReader::new(File::open(source_file).expect(PANIC_READ)).read_to_string(&mut input) {
            Err(why) => panic!("{} : {}", PANIC_READ, why),
            Ok(_) => {
                // target file (not required)
                let mut target_file = String::new();
                if args.len() as i32 > 2 {
                    target_file = String::from(&args[2]);
                } else {
                    target_file = format!("{}{}", file_name[0].to_string(), WASM_WAT);
                }
                //println!("{:?}", target_file);
                let target_js = format!("{}{}", file_name[0].to_string(), WASM_JS);

                // lexer
                let lexer = Lexer::new(input.to_string());
                // parser
                let parser = Parser::new(lexer);
                // emitter
                let file = File::create(target_file).expect(PANIC_WRITE);
                let file = BufWriter::new(file);
                let mut emitter = Emitter::new(parser, file);
                // compile
                emitter.compile();
                // js
                let js = File::create(target_js).expect(PANIC_WRITE);
                let mut js = BufWriter::new(js);
                js.write_all(format!("/* this file is generated */{}const wasmInstance = new WebAssembly.Instance(wasmModule, {{}});{}const {{ {} }} = wasmInstance.exports;{}console.log({}());", FORMAT_NEWLINE, FORMAT_NEWLINE, file_name[0].to_string(), FORMAT_NEWLINE, file_name[0].to_string()).as_bytes()).expect(PANIC_WRITE);
            },
        }
    } else {
        panic!("{} : {}", PANIC_READ, PANIC_FILE);
    }
    
    // let lexer = Lexer::new("42".to_string());                                 // 42       : OK
    // let lexer = Lexer::new("-42".to_string());                                // -42      : OK
    // let lexer = Lexer::new("10 + 2".to_string());                             // 12       : OK
    // let lexer = Lexer::new("-(10 + (2 * 3))".to_string());                    // -16      : OK
    // let lexer = Lexer::new("-(10.5 + (2 * 4) / 5)".to_string());              // -12.1    : OK
    // let lexer = Lexer::new("10 MOD 4".to_string());                           // 2        : OK
    // let lexer = Lexer::new("10 DIV 4".to_string());                           // 2        : OK
    // let lexer = Lexer::new("--10+-2".to_string());                            // 8        : OK
    // let lexer = Lexer::new("((22.5 MOD 8) * (21 DIV 3))".to_string());        // 42       : OK
    // let lexer = Lexer::new("((22 MOD (2 * 4)) * (21 DIV 3))".to_string());    // 42       : OK

    // println!("{:?}", "first, second: INTEGER;");
    // let lexer = Lexer::new("first, second, third: INTEGER;".to_string());

    // let lexer = Lexer::new("number := 42;".to_string());
    // let lexer = Lexer::new("number := (40 + 2);".to_string());
    // let lexer = Lexer::new("number := (40 MOD (10 - 8)) - (-2);".to_string());

    // println!("{:?}", "numbers[0] := (2 + 2);");
    // let lexer = Lexer::new("numbers[0] := (2 + 2);".to_string());
    // let tree = Parser::new(lexer).assignment_statement();
    
    // println!("{:?}", "numbers[(1 - 1)]");
    // let lexer = Lexer::new("numbers[(1 - 1)]".to_string());
    // let tree = Parser::new(lexer).variable();
    
    // println!("{:?}", "numbers: ARRAY[2..4] OF INTEGER;");
    // let lexer = Lexer::new("numbers: ARRAY[2..4] OF INTEGER;".to_string());
    // let tree = Parser::new(lexer).variable_declaration();
}
