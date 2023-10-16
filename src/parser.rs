use crate::config::*;
use crate::consts::*;
use crate::token::{ Type, Token };
use crate::lexer::Lexer;
use crate::ast::AST;
use crate::evaluator::Evaluator;

/*

  program                 : PROGRAM variable SEMICOLON block DOT

  block                   : declarations compound_statement

  declarations            : CONST (constant_declaration SEMICOLON)+ | VAR (variable_declaration SEMICOLON)+ | empty

  constant_declaration    : ID EQUAL (INTEGER | REAL)
  variable_declaration    : ID (COMMA ID)* COLON type_spec

  compound_statement      : BEGIN statement (SEMICOLON statement)* END
  statement               : compound_statement | structured_statement | assignment_statement | function_statement | empty
  structured_statement    : if_statement | while_statement
  assignment_statement    : variable ASSIGN (simple_expression | (TRUE | FALSE))
  function_statement      : function LPAR simple_expression RPAR
  
  if_statement            : IF condition THEN statement (ELSE statement)?
  while_statement         : WHILE condition DO statement

  condition               : (TRUE | FALSE) | ODD simple_expression | simple_expression (EQUAL | GREATER_THAN | GREATER_EQUAL | LESS_THAN | LESS_EQUAL | NOT_EQUAL) simple_expression
  simple_expression       : term ((PLUS | MINUS) term)*

  term                    : factor ((MULTIPLY | DIVIDE | INTEGER_DIV | INTEGER_MOD) factor)*
  factor                  : PLUS factor | MINUS factor | INTEGER | REAL | LPAR simple_expression RPAR | variable

  type_spec               : INTEGER | REAL
  variable                : ID
  function                : WRITELN | [TODO: add more built-in functions]
  empty                   : 

*/

/*

  TODO: replace subset of pascal with PL/0 grammar

  program       = { statement };
  statement     = assign_stmt | input_stmt | output_stmt | if_stmt | while_stmt ;
  assign_stmt   = identifier ':=' expr ';';
  input_stmt    = '?' identifier ';';
  output_stmt   = '!' expr ';';

  if_stmt       = 'if' condition ('then' | '{') statement ('end' | '}');
  while_stmt    = 'while' condition ('do' | '{') statement ('end' | '}');

  condition     = cond_odd | cond_expr;
  cond_odd      = 'odd' expr;
  cond_expr     = expr ('=' | '<' | '>') expr;

  expr          = term {('+' | '-') term};
  term          = factor {('*' | '/') factor};
  factor        = identifier | number | '(' expr ')';

*/

pub struct Parser {
  lexer: Lexer,
  pub current_token: Option<Token>,
  pub symbol_table: Vec<(Token, Type)>,
  pub assign_table: Vec<(Token, AST)>,
}

impl Parser {
  // new : Parser
  pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
      lexer: lexer,
      current_token: None,
      symbol_table: vec![],
      assign_table: vec![],
    };
    parser.current_token = Some(parser.lexer.get_next_token());
    return parser
  }
  // eat (consume token)
  fn eat(&mut self, token: Token) {
    if DEBUG && DEBUG_SHOW_TOKEN { println!("{}{}{:?}", FORMAT_TAB, FORMAT_SPACE.repeat(2), token); }
    // clone used to deep copy value
    if token == self.current_token.clone().unwrap() {
      self.current_token = Some(self.lexer.get_next_token());
    } else {
      panic!("{:?} : {}", token, PANIC_SYNTAX)
    }
  }
  // type_spec : AST
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
      _ => panic!("{:?} : {}", token, PANIC_TYPE_DECLARATION)
    }
  }
  // empty : AST
  fn empty(&mut self) -> AST {
    /*
      empty : 
    */
    return AST::new(Token::EMPTY, vec![])
  }
  // variable : AST
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
      _ => panic!("{:?} : {}", token, PANIC_SYNTAX)
    }
  }
  // array_type : AST
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
          _ => panic!("{:?} : {}", token, PANIC_ARRAY)
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
            _ => panic!("{:?} : {}", _variable, PANIC_SYNTAX)
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
                _ => panic!("{:?} : {}", _variable, PANIC_SYNTAX)
              }
            }
          }
        }
        return node
      },
      _ => panic!("{:?} : {}", token, PANIC_SYNTAX)
    }
  }
  // structured_type : AST
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
      _ => panic!("{:?} : {}", token, PANIC_SYNTAX)
    }
  }
  // constant_declaration : AST
  // fn constant_declaration(&mut self) -> AST {
  //   /*
  //     constant_declaration : ID EQUAL (INTEGER | REAL)
  //   */
  //   // ID
  //   let constant_node = self.variable();
  //   // EQUAL
  //   self.eat(Token::EQUAL);
  //   // (INTEGER | REAL)
  //   let token = self.current_token.clone().unwrap();
  //   match token {
  //     // INTEGER
  //     Token::INTEGER(_int) => {
  //       let node = Token::INTEGER(_int);
  //       // INTEGER
  //       self.eat(Token::INTEGER(_int));
  //       self.symbol_table.push((constant_node.token.clone(), Type::INTEGER.clone()));
  //       // self.result_type = true;
  //       return AST::new(node, vec![constant_node]);
  //     },
  //     // REAL
  //     Token::REAL(_float) => {
  //       let node = Token::REAL(_float);
  //       // INTEGER
  //       self.eat(Token::REAL(_float));
  //       self.symbol_table.push((constant_node.token.clone(), Type::REAL.clone()));
  //       // self.result_type = true;
  //       return AST::new(node, vec![constant_node]);
  //     },
  //     _ => panic!(format!("{:?} : {}", token, PANIC_TYPE_DECLARATION))
  //   }
  // }
  // variable_declaration : AST
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
      _ => panic!("{:?} : {}", token, PANIC_TYPE_DECLARATION)
    }
  }
  // constant_declarations : AST
  // fn constant_declarations(&mut self) -> AST {
  //   /*
  //     constant_declarations : CONST (constant_declaration SEMICOLON)+ | empty
  //   */
  //   let token = self.current_token.clone().unwrap();
  //   match token {
  //     Token::CONST => {
  //       let mut constant_declarations = vec![];
  //       // CONST
  //       self.eat(Token::CONST);
  //       // (constant_declaration SEMICOLON)+
  //       while let Some(_token) = &self.current_token {
  //         match _token {
  //           Token::ID(_string) => {
  //             constant_declarations.push(self.constant_declaration());
  //             self.eat(Token::SEMICOLON);
  //           },
  //           _ => break
  //         }
  //       }
  //       let node = AST::new(Token::CONST, constant_declarations);
  //       return node
  //     },
  //     _ => return self.empty()
  //   }
  // }
  // variable_declarations : AST
  fn variable_declarations(&mut self) -> AST {
    /*
      variable_declarations : VAR (variable_declaration SEMICOLON)+ | empty
    */
    let token = self.current_token.clone().unwrap();
    match token {
      Token::VAR => {
        let mut variable_declarations = vec![];
        // VAR
        self.eat(Token::VAR);
        // (variable_declaration SEMICOLON)+
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
  // assignment_statement : AST
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
            self.assign_table.push((node.token, simple_expression));
            // self.result_type = false;
            // new branch
            return AST::new(Token::ASSIGN, children);
          }
          _ => panic!("{:?} : {}", node.token, PANIC_SYNTAX)
        }
      }
    }
    panic!("{:?} : {}", node, PANIC_VAR_NOT_DECLARAED)
  }
  // factor : AST
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
        // self.result_type = true;
        return AST::new(token, vec![]);
      },
      Token::REAL(_float) => {
        // REAL
        self.eat(Token::REAL(_float));
        // self.result_type = true;
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
  // term : AST
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
        _ => panic!("{:?} : {}", self.current_token, PANIC_SYNTAX)
      }
    }
    return node;
  }
  // simple_expression : AST
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
        _ => panic!("{:?} : {}", self.current_token, PANIC_SYNTAX)
      }
    }
    return node;
  }
  // expression : AST
  fn expression(&mut self) -> AST {
    /*
      (TRUE | FALSE) | ODD LPAR simple_expression RPAR | simple_expression ((EQUAL | GREATER_THAN | GREATER_EQUAL | LESS_THAN | LESS_EQUAL | NOT_EQUAL) simple_expression)?
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
          // FALSE
          self.eat(Token::FALSE);
          node = AST::new(Token::FALSE, vec![]);
        },
        _ => {}
      }
    // odd
    } else if self.current_token == Some(Token::ODD) {
      // ODD
      self.eat(Token::ODD);
      // LPAR
      self.eat(Token::LPAR);
      let children: Vec<AST> = vec![self.simple_expression(), AST::new(Token::INTEGER(2), vec![])];
      node = AST::new(Token::INTEGER_MOD, children);
      // RPAR
      self.eat(Token::RPAR);
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
  // function_statement : AST
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
        // self.result_type = true;
        return node
      }
      _ => panic!("{:?} : {}", self.current_token, PANIC_SYNTAX)
    }
  }
  // while_statement : AST
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
  // if_statement : AST
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
  // structured_statement : AST
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
      _ => panic!("{:?} : {}", self.current_token, PANIC_SYNTAX)
    }
  }
  // statement : AST
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
  // statement_list : AST
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
  // compound_statement : AST
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
  // block : AST
  fn block(&mut self) -> AST {
    /*
      block : declarations compound_statement
    */
    // let constant_declarations_nodes = self.constant_declarations();
    let variable_declarations_nodes = self.variable_declarations();
    let compound_statement_nodes = self.compound_statement();
    // let node = AST::new(Token::BLOCK, vec![constant_declarations_nodes, variable_declarations_nodes, compound_statement_nodes]);
    let node = AST::new(Token::BLOCK, vec![variable_declarations_nodes, compound_statement_nodes]);
    return node
  }
  // program : AST
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
  // parse : AST
  pub fn parse(&mut self) -> AST {
    // start at program
    let node = self.program();
    if self.current_token != Some(Token::EOF) {
      panic!("{:?} : {}", self.current_token, PANIC_SYNTAX)
    }
    return node;
  }
}
// ----------------------------------------------------
#[cfg(test)]
mod test {
  // use super::*;
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