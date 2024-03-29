#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::env;
// use std::fs::{ self, File };
// use std::io::{ BufReader, BufWriter, Write };
// use std::io::prelude::*;

mod ast;
mod config;
mod consts;
mod emitter;
mod evaluator;
mod lexer;
mod parser;

// use crate::config::*;
// use crate::consts::*;
// use crate::emitter::Emitter;
use crate::lexer::Lexer;
use crate::parser::Parser;

// only need this for testing
mod token;
// use crate::token::{ Token };

fn main() {
  let args: Vec<String> = env::args().collect();
  // println!("{:?}", args);

  let lexer = Lexer::new("
    test := 42;
  ".to_string());
  // let mut token = lexer.get_next_token();
  // while token != Token::EOF {
  //   println!("{}{}{:?}", FORMAT_TAB, FORMAT_SPACE.repeat(2), token);
  //   token = lexer.get_next_token();
  // };
  let tree = Parser::new(lexer).parse();

  println!("{:?}", tree);
  // AST { token: ASSIGN, children: [AST { token: ID("test"), children: [] }, AST { token: INTEGER(42), children: [] }] }

  // if DEBUG && DEBUG_WITH_INPUT {
  //   let mut source_file = String::new();

  //   // println!("{:?}", args);

  //   if args.len() as i32 > 1 {
  //     source_file = String::from(&args[1]);
  //     // file_name
  //     let input_string = String::from(&source_file);
  //     let file_name: Vec<&str> = input_string.split('.').collect();
  //     // read input file
  //     let mut input = String::new();
  //     match BufReader::new(File::open(source_file).expect(PANIC_READ)).read_to_string(&mut input) {
  //       Err(why) => panic!("{} : {}", PANIC_READ, why),
  //       Ok(_) => {
  //         // target file (optional)
  //         let mut target_file = String::new();
  //         if args.len() as i32 > 2 {
  //             target_file = String::from(&args[2]);
  //         } else {
  //             target_file = format!("{}{}", file_name[0].to_string(), WASM_WAT);
  //         }
  //         let target_js = format!("{}{}", file_name[0].to_string(), WASM_JS);
  //         // lexer
  //         let lexer = Lexer::new(input.to_string());
  //         // parser
  //         let parser = Parser::new(lexer);
  //         // emitter
  //         let file = File::create(target_file).expect(PANIC_WRITE);
  //         let file = BufWriter::new(file);
  //         let mut emitter = Emitter::new(parser, file);
  //         // compile
  //         emitter.compile();
  //         // js
  //         let js = File::create(target_js).expect(PANIC_WRITE);
  //         let mut js = BufWriter::new(js);
  //         js.write_all(format!("/* this file is generated */{}const wasmInstance = new WebAssembly.Instance(wasmModule, {{console}});{}const {{ {} }} = wasmInstance.exports;{}{}();", FORMAT_NEWLINE, FORMAT_NEWLINE, file_name[0].to_string(), FORMAT_NEWLINE, file_name[0].to_string()).as_bytes()).expect(PANIC_WRITE);
  //       },
  //     }
  //     // show result
  //     if DEBUG && DEBUG_SHOW_RESULT {
  //       let result = fs::read_to_string(format!("{}{}", file_name[0].to_string(), WASM_WAT)).expect(PANIC_READ);
  //       println!("{}{}START RESULT{}{}", "-".repeat(14), FORMAT_SPACE, FORMAT_SPACE, "-".repeat(14));
  //       for line in result.lines() {
  //         println!("{}", line);
  //       }
  //       println!("{}{}END RESULT{}{}", "-".repeat(15), FORMAT_SPACE, FORMAT_SPACE, "-".repeat(15));
  //     }
  //   } else {
  //     panic!("{} : {}", PANIC_READ, PANIC_FILE);
  //   }
  // // run parts of compiler using custom input
  // } else {
  //   let lexer = Lexer::new("
  //     test := 42;
  //   ".to_string());
  //   // let mut token = lexer.get_next_token();
  //   // while token != Token::EOF {
  //   //   println!("{}{}{:?}", FORMAT_TAB, FORMAT_SPACE.repeat(2), token);
  //   //   token = lexer.get_next_token();
  //   // };
  //   let tree = Parser::new(lexer, DEBUG_SHOW_TOKEN).parse();

  //   println!("{:?}", tree);
  //   // AST { token: ASSIGN, children: [AST { token: ID("test"), children: [] }, AST { token: INTEGER(42), children: [] }] }
  // }
}
