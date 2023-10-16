// https://codebeautify.org/jsviewer

use std::fmt;

use crate::token::{ Token };

#[derive(Clone, Debug, PartialEq)]
pub struct AST {
  pub token: Token,
  pub children: Vec<AST>,
}

impl AST {
  // new() -> AST
  pub fn new(token: Token, children: Vec<AST>) -> AST {
    AST {
      token: token,
      children: children
    }
  }
}

impl std::fmt::Display for AST {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}