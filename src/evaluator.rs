use crate::r#const::*;
use crate::r#type::Type;
use crate::token::Token;
use crate::ast::AST;

// evaluator
// ----------------------------------------------------
pub struct Evaluator {
  symbol_table: Vec<(Token, Type)>,
  assign_table: Vec<(Token, AST)>,
  // to track tokens
  tokens: Vec<Token>,
}

impl Evaluator {
  // new : Evaluator
  pub fn new(symbol_table: Vec<(Token, Type)>, assign_table: Vec<(Token, AST)>) -> Evaluator {
    let evaluator = Evaluator {
      symbol_table: symbol_table,
      assign_table: assign_table,
      tokens: vec![],
    };
    return evaluator;
  }
  // eval_number : f64
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
  // eval_binary_operator : f64
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
  // evaluate : f64
  pub fn evaluate(&mut self, node: &AST) -> f64 {
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