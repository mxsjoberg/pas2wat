use std::fs::File;
use std::io::{BufWriter, Write};

use crate::config::*;
use crate::r#const::*;
use crate::r#type::Type;
use crate::token::Token;
use crate::ast::AST;
use crate::parser::Parser;

// emitter
// ----------------------------------------------------
pub struct Emitter {
  parser: Parser,
  file: BufWriter<File>,
  tab_pos: i32,
  require_i32: bool,
}
impl Emitter {
  // new : Emitter
  pub fn new(parser: Parser, file: BufWriter<File>) -> Emitter {
    let emitter = Emitter {
      parser: parser,
      file: file,
      tab_pos: 0,
      // control flow require condition to be i32
      require_i32: false,
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
      // Token::PLUS => {
      // },
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
          if !self.require_i32 {
            match self.file.write_all(format!("{}{}({}{}){}{}({}{}_{}_s)", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_INTEGER_DIV, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_CONVERT, NTYPE_INTEGER).as_bytes()) {
              Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
              Ok(_) => {},
            }
          } else {
            match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_INTEGER_DIV).as_bytes()) {
              Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
              Ok(_) => {},
            }
            self.require_i32 = false;
          }
        },
        Token::INTEGER_MOD => {
          if !self.require_i32 {
            match self.file.write_all(format!("{}{}({}{}){}{}({}{}_{}_s)", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_INTEGER_MOD, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_REAL, WASM_CONVERT, NTYPE_INTEGER).as_bytes()) {
              Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
              Ok(_) => {},
            }
          } else {
            match self.file.write_all(format!("{}{}({}{})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), NTYPE_INTEGER, WASM_INTEGER_MOD).as_bytes()) {
              Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
              Ok(_) => {},
            }
            self.require_i32 = false;
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
        self.require_i32 = true;
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
    // if self.parser.result_type {
    //     //println!("{:?}", self.parser.result_type);
    //     match self.file.write_all(format!("{}{}({} {}){}{};; body", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_RESULT, NTYPE_REAL, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()) {
    //         Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
    //         Ok(_) => {},
    //     }
    // }
  }
  // visit_program
  fn visit_program(&mut self, node: &AST) {
    match &node.children[0].token {
      Token::ID(_string) => {
        match self.file.write_all(format!("{}({}{}{}(import \"console\" \"log\" (func $log (param f64))){}{}({}{}{};; signature{}{}({} \"{}\")", FORMAT_NEWLINE, WASM_MODULE, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 1), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 1), WASM_FUNCTION, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 2), FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize + 2), WASM_EXPORT, _string).as_bytes()) {
          Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
          Ok(_) => { 
            self.tab_pos += 2;
            // check if next is variable declaration
            //println!("{:?}", &node.children[1].children[0].token);
            match &node.children[1].children[0].token {
              Token::VAR => {}
              _ => {
                  // declare result type (if any)
                  // if self.parser.result_type {
                  //     //println!("{:?}", self.parser.result_type);
                  //     match self.file.write_all(format!("{}{}({} {}){}{};; body", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_RESULT, NTYPE_REAL, FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()) {
                  //         Err(why) => panic!("{} : {:?} : {}", PANIC_COMPILE, node, why),
                  //         Ok(_) => {},
                  //     }
                  // }
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
        // write
        if OUTPUT_VERBOSE { self.file.write_all(format!("{}{};; write", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize)).as_bytes()).expect(PANIC_WRITE); };
        self.file.write_all(format!("{}{}({})", FORMAT_NEWLINE, FORMAT_TAB.repeat(self.tab_pos as usize), WASM_WRITE).as_bytes()).expect(PANIC_WRITE);
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
  pub fn compile(&mut self) {
    let tree = self.parser.parse();
    if DEBUG && DEBUG_SHOW_TREE {
        println!("{:?}", tree);
    };
    if DEBUG && DEBUG_SHOW_SYMBOL_TABLE { println!("{:?}", self.parser.symbol_table); };
    if DEBUG && DEBUG_SHOW_ASSIGNMENT_TABLE { println!("{:?}", self.parser.assign_table); };
    // write
    self.file.write_all(format!(";; this file is generated").as_bytes()).expect(PANIC_WRITE);
    self.visit(&tree);
  }
}
// ----------------------------------------------------