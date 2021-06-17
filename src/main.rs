/*

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

// config
mod config;
mod r#const;

use crate::config::*;
use crate::r#const::*;

// compiler parts
mod r#type;
mod token;
mod lexer;
mod ast;
mod evaluator;
mod parser;
mod emitter;

use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::emitter::Emitter;

// main
// ----------------------------------------------------
fn main() {
    // run compile on content in file
    if DEBUG && DEBUG_WITH_INPUT {
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
    // otherwise
    } else {
        let mut lexer = Lexer::new("40 MOD (10 - 8) * -2".to_string());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        println!("{:?}", lexer.get_next_token());
        // Some('4')
        // Some('0')
        // INTEGER(40)
        // Some(' ')
        // Some('M')
        // Some('O')
        // Some('D')
        // INTEGER_MOD
        // Some(' ')
        // Some('(')
        // LPAR
        // Some('1')
        // Some('0')
        // INTEGER(10)
        // Some(' ')
        // Some('-')
        // MINUS
        // Some(' ')
        // Some('8')
        // INTEGER(8)
        // Some(')')
        // RPAR
        // Some(' ')
        // Some('*')
        // MULTIPLY
        // Some(' ')
        // Some('-')
        // MINUS
        // Some('2')
        // INTEGER(2)
        // EOF

        // TODO: move to unit test
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
}
// ----------------------------------------------------
