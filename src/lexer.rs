use crate::config::*;
use crate::consts::*;
use crate::token::{ Token };

// characters
const CHAR_DOT                  : char = '.';
const CHAR_UNDERSCORE           : char = '_';
// const CHAR_COMMA                : char = ',';
const CHAR_COLON                : char = ':';
const CHAR_SEMICOLON            : char = ';';
const CHAR_EQUAL                : char = '=';
const CHAR_PLUS                 : char = '+';
const CHAR_MINUS                : char = '-';
const CHAR_MULTIPLY             : char = '*';
const CHAR_DIVIDE               : char = '/';
const CHAR_LPAR                 : char = '(';
const CHAR_RPAR                 : char = ')';
// const CHAR_LBRA                 : char = '[';
// const CHAR_RBRA                 : char = ']';
const CHAR_LCUR                 : char = '{';
const CHAR_RCUR                 : char = '}';
const CHAR_GREATER_THAN         : char = '>';
const CHAR_LESS_THAN            : char = '<';
const CHAR_NEWLINE              : char = '\n';
// keywords
// const KEY_PROGRAM               : &str = "PROGRAM";
// const KEY_CONST                 : &str = "CONST";
// const KEY_VAR                   : &str = "VAR";
// const KEY_DIV                   : &str = "DIV";
// const KEY_MOD                   : &str = "MOD";
const KEY_ODD                   : &str = "ODD";
const KEY_BEGIN                 : &str = "BEGIN";
const KEY_END                   : &str = "END";
// const KEY_INTEGER               : &str = "INTEGER";
// const KEY_LONGINT               : &str = "LONGINT";
// const KEY_SMALLINT              : &str = "SMALLINT";
// const KEY_REAL                  : &str = "REAL";
// const KEY_BOOLEAN               : &str = "BOOLEAN";
// const KEY_TRUE                  : &str = "TRUE";
// const KEY_FALSE                 : &str = "FALSE";
// const KEY_PACKED                : &str = "PACKED";
// const KEY_ARRAY                 : &str = "ARRAY";
// const KEY_OF                    : &str = "OF";
const KEY_WHILE                 : &str = "WHILE";
const KEY_DO                    : &str = "DO";
const KEY_IF                    : &str = "IF";
const KEY_THEN                  : &str = "THEN";
const KEY_ELSE                  : &str = "ELSE";
// functions
// const FUNC_WRITELN              : &str = "WRITELN";

pub struct Lexer {
  text: String,
  position: i32,
  current_char: Option<char>,
  comment_multiline: bool,
}

impl Lexer {
  // new(String) -> Lexer
  pub fn new(text: String) -> Lexer {
    let mut lexer = Lexer {
      text: text,
      position: 0,
      current_char: None,
      comment_multiline: false,
    };
    if lexer.text.len() > 0 {
      lexer.current_char = Some(lexer.text.as_bytes()[0] as char);
    }
    return lexer;
  }
  // look_ahead() -> Option<char>
  fn look_ahead(&mut self) -> Option<char> {
    // next position in text
    let next_position = self.position as i32 + 1;
    if next_position > self.text.len() as i32 - 1 {
      return None;
    } else {
      return Some(self.text.as_bytes()[next_position as usize] as char);
    }
  }
  // next_token()
  fn next_token(&mut self) {
    if DEBUG && DEBUG_SHOW_CHAR { println!("{}{}{:?}", FORMAT_TAB, FORMAT_SPACE.repeat(2), self.current_char); }
    self.position += 1;
    // EOF
    if self.position > self.text.len() as i32 - 1 {
      self.current_char = None;
    }
    else {
      // usize is specific to target machine
      self.current_char = Some(self.text.as_bytes()[self.position as usize] as char);
    }
  }
  // skip_whitespace() TODO: rename to whitespace()
  fn skip_whitespace(&mut self) {
    while let Some(_char) = self.current_char {
      if _char.is_whitespace() {
        self.next_token();
      } else {
        break;
      }
    }
  }
  // skip_comment() TODO: rename to comment()
  fn skip_comment(&mut self) {
    if self.comment_multiline == true {
      // next_token until closing curly
      while let Some(_char) = self.current_char {
        if _char != CHAR_RCUR {
          self.next_token();
        } else {
          self.comment_multiline = false;
          // closing curly brace
          self.next_token();
          break;
        }
      }
    } else {
      // next_token until newline
      while let Some(_char) = self.current_char {
        if _char != CHAR_NEWLINE {
          self.next_token();
        } else {
          // newline
          // self.next_token();
          break;
        }
      }
    }
  }
  // number() -> Token
  fn number(&mut self) -> Token {
    let mut number = String::new();
    while let Some(_char) = self.current_char {
      if _char.is_digit(10) {
        number.push(_char);
        self.next_token();
      } else {
        break;
      }
    }
    // two dots is range
    if self.current_char == Some(CHAR_DOT) && self.look_ahead() == Some(CHAR_DOT) {
      self.next_token();
      self.next_token();
      let mut end = String::new();
      while let Some(_char) = self.current_char {
        // check if char is digit
        if _char.is_digit(10) {
          // push to number
          end.push(_char);
          self.next_token();
        } else {
          break;
        }
      }
      return Token::RANGE(number.parse::<i32>().unwrap(), end.parse::<i32>().unwrap());
    }
    // one dot is floating-point
    if self.current_char == Some(CHAR_DOT) {
      // push to number
      number.push(CHAR_DOT);
      // next_token
      self.next_token();
      // next_token until not number
      while let Some(_char) = self.current_char {
        // check if char is digit
        if _char.is_digit(10) {
          // push to number
          number.push(_char);
          // next_token
          self.next_token();
        } else {
          break;
        }
      }
      return Token::REAL(number.parse::<f64>().unwrap());
    }
    return Token::INTEGER(number.parse::<i32>().unwrap());
  }
  // id() -> Token
  fn id(&mut self) -> Token {
    let mut string = String::new();
    while let Some(_char) = self.current_char {
      if _char.is_alphabetic() || _char.is_digit(10) || _char == CHAR_UNDERSCORE {
        string.push(_char);
        self.next_token();
      } else {
        break;
      }
    }
    // keywords
    match string.to_uppercase().as_str() {
      // KEY_PROGRAM => {
      //   return Token::PROGRAM;
      // },
      // KEY_VAR => {
      //   return Token::VAR;
      // },
      // KEY_CONST => {
      //   return Token::CONST;
      // },
      // KEY_DIV => {
      //   return Token::INTEGER_DIV;
      // },
      // KEY_MOD => {
      //   return Token::INTEGER_MOD;
      // },
      KEY_ODD => {
        return Token::ODD;
      },
      KEY_BEGIN => {
        return Token::BEGIN;
      },
      KEY_END => {
        return Token::END;
      },
      // KEY_INTEGER => {
      //   return Token::TYPE_SPEC(Type::INTEGER);
      // },
      // KEY_LONGINT => {
      //   return Token::TYPE_SPEC(Type::INTEGER);
      // },
      // KEY_SMALLINT => {
      //   return Token::TYPE_SPEC(Type::INTEGER);
      // },
      // KEY_REAL => {
      //   return Token::TYPE_SPEC(Type::REAL);
      // },
      // KEY_BOOLEAN => {
      //   return Token::TYPE_SPEC(Type::BOOLEAN);
      // },
      // KEY_TRUE => {
      //   return Token::TRUE;
      // },
      // KEY_FALSE => {
      //   return Token::FALSE;
      // },
      // KEY_PACKED => {
      //   return Token::PACKED;
      // },
      // KEY_ARRAY => {
      //   return Token::ARRAY;
      // },
      // KEY_OF => {
      //   return Token::OF;
      // },
      KEY_WHILE => {
        return Token::WHILE;
      },
      KEY_DO => {
        return Token::DO;
      },
      KEY_IF => {
        return Token::IF;
      },
      KEY_THEN => {
        return Token::THEN;
      },
      KEY_ELSE => {
        return Token::ELSE;
      },
      // FUNC_WRITELN => {
      //   return Token::WRITELN;
      // },
      _ => Token::ID(string)
    }
  }
  // get_next_token() -> Token
  pub fn get_next_token(&mut self) -> Token {
    while let Some(_char) = self.current_char {
      // whitespace
      if _char.is_whitespace() {
        self.skip_whitespace();
        continue;
      }
      // multiline comment
      // if _char == CHAR_LCUR {
      //   self.comment_multiline = true;
      //   self.next_token();
      //   self.skip_comment();
      //   continue;
      // }
      // comment -> //
      if _char == CHAR_DIVIDE && self.look_ahead() == Some(CHAR_DIVIDE) {
        self.next_token();
        self.skip_comment();
        continue; 
      }
      // identifier -> [a-zA-Z]
      if _char.is_alphabetic() {
        return self.id();
      }
      // number -> [0-9]
      // base-10 is decimal number
      if _char.is_digit(10) {
        return self.number();
      }
      // assignment -> :=
      if _char == CHAR_COLON && self.look_ahead() == Some(CHAR_EQUAL) {
        self.next_token();
        self.next_token();
        return Token::ASSIGN;
      }
      // not equal -> <>
      // if _char == CHAR_LESS_THAN && self.look_ahead() == Some(CHAR_GREATER_THAN) {
      //   self.next_token();
      //   self.next_token();
      //   return Token::NOT_EQUAL;
      // }
      // less than or equal -> <=
      // if _char == CHAR_LESS_THAN && self.look_ahead() == Some(CHAR_EQUAL) {
      //   self.next_token();
      //   self.next_token();
      //   return Token::LESS_EQUAL;
      // }
      // greater than or equal -> >=
      // if _char == CHAR_GREATER_THAN && self.look_ahead() == Some(CHAR_EQUAL) {
      //   self.next_token();
      //   self.next_token();
      //   return Token::GREATER_EQUAL;
      // }
      // colon -> :
      // if _char == CHAR_COLON {
      //   self.next_token();
      //   return Token::COLON;
      // }
      // semicolon -> ;
      if _char == CHAR_SEMICOLON {
        self.next_token();
        return Token::SEMICOLON;
      }
      // comma -> ,
      // if _char == CHAR_COMMA {
      //   self.next_token();
      //   return Token::COMMA;
      // }
      // dot -> .
      // if _char == CHAR_DOT {
      //   self.current_char = None;
      //   return Token::DOT;
      // }
      // operators
      match _char {
        CHAR_PLUS => {
          self.next_token();
          return Token::PLUS;
        },
        CHAR_MINUS => {
          self.next_token();
          return Token::MINUS;
        },
        CHAR_MULTIPLY => {
          self.next_token();
          return Token::MULTIPLY;
        },
        CHAR_DIVIDE => {
          self.next_token();
          return Token::DIVIDE;
        },
        CHAR_LPAR => {
          self.next_token();
          return Token::LPAR;
        },
        CHAR_RPAR => {
          self.next_token();
          return Token::RPAR;
        },
        // CHAR_LBRA => {
        //   self.next_token();
        //   return Token::LBRA;
        // },
        // CHAR_RBRA => {
        //   self.next_token();
        //   return Token::RBRA;
        // },
        CHAR_EQUAL => {
          self.next_token();
          return Token::EQUAL;
        },
        CHAR_GREATER_THAN => {
          self.next_token();
          return Token::GREATER_THAN;
        },
        CHAR_LESS_THAN => {
          self.next_token();
          return Token::LESS_THAN;
        },
        _ => panic!("{} : {}", _char.to_string(), PANIC_SYNTAX)
      }
    }
    Token::EOF
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn new() {
    let lexer = Lexer::new("42".to_string());
    assert_eq!(lexer.text, "42");
    assert_eq!(lexer.position, 0);
    assert_eq!(lexer.current_char, Some('4'));
  }

  #[test]
  fn look_ahead() {
    let mut lexer = Lexer::new("42".to_string());
    assert_eq!(lexer.look_ahead(), Some('2'));
    let mut lexer = Lexer::new("1".to_string());
    assert_eq!(lexer.look_ahead(), None);
  }

  #[test]
  fn next_token() {
    let mut lexer = Lexer::new("42".to_string());
    assert_eq!(lexer.current_char, Some('4'));
    lexer.next_token();
    assert_eq!(lexer.current_char, Some('2'));
    lexer.next_token();
    assert_eq!(lexer.current_char, None);
  }

  #[test]
  fn skip_whitespace() {
    let mut lexer = Lexer::new("1  2".to_string());
    assert_eq!(lexer.current_char, Some('1'));
    lexer.next_token();
    assert_eq!(lexer.current_char, Some(' '));
    lexer.skip_whitespace();
    assert_eq!(lexer.current_char, Some('2'));
  }

  // #[test]
  // fn skip_comment() {
  //   let mut lexer = Lexer::new("{ 2 }".to_string());
  //   assert_eq!(lexer.current_char, Some('{'));
  //   lexer.skip_comment();
  //   assert_eq!(lexer.current_char, None);
  // }

  #[test]
  fn number() {
    let mut lexer = Lexer::new("42".to_string());
    assert_eq!(lexer.number(), Token::INTEGER(42));
    let mut lexer = Lexer::new("4.2".to_string());
    assert_eq!(lexer.number(), Token::REAL(4.2));
    let mut lexer = Lexer::new("4..2".to_string());
    assert_eq!(lexer.number(), Token::RANGE(4, 2));
  }

  #[test]
  fn id() {
    // let mut lexer = Lexer::new("PROGRAM".to_string());
    // assert_eq!(lexer.id(), Token::PROGRAM);
    // let mut lexer = Lexer::new("VAR".to_string());
    // assert_eq!(lexer.id(), Token::VAR);
    // let mut lexer = Lexer::new("CONST".to_string());
    // assert_eq!(lexer.id(), Token::CONST);
    // let mut lexer = Lexer::new("DIV".to_string());
    // assert_eq!(lexer.id(), Token::INTEGER_DIV);
    // let mut lexer = Lexer::new("MOD".to_string());
    // assert_eq!(lexer.id(), Token::INTEGER_MOD);
    let mut lexer = Lexer::new("BEGIN".to_string());
    assert_eq!(lexer.id(), Token::BEGIN);
    let mut lexer = Lexer::new("END".to_string());
    assert_eq!(lexer.id(), Token::END);
    // let mut lexer = Lexer::new("INTEGER".to_string());
    // assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::INTEGER));
    // let mut lexer = Lexer::new("LONGINT".to_string());
    // assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::INTEGER));
    // let mut lexer = Lexer::new("SMALLINT".to_string());
    // assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::INTEGER));
    // let mut lexer = Lexer::new("REAL".to_string());
    // assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::REAL));
    // let mut lexer = Lexer::new("BOOLEAN".to_string());
    // assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::BOOLEAN));
    // let mut lexer = Lexer::new("TRUE".to_string());
    // assert_eq!(lexer.id(), Token::TRUE);
    // let mut lexer = Lexer::new("FALSE".to_string());
    // assert_eq!(lexer.id(), Token::FALSE);
    // let mut lexer = Lexer::new("PACKED".to_string());
    // assert_eq!(lexer.id(), Token::PACKED);
    // let mut lexer = Lexer::new("ARRAY".to_string());
    // assert_eq!(lexer.id(), Token::ARRAY);
    // let mut lexer = Lexer::new("OF".to_string());
    // assert_eq!(lexer.id(), Token::OF);
    let mut lexer = Lexer::new("WHILE".to_string());
    assert_eq!(lexer.id(), Token::WHILE);
    let mut lexer = Lexer::new("DO".to_string());
    assert_eq!(lexer.id(), Token::DO);
    let mut lexer = Lexer::new("IF".to_string());
    assert_eq!(lexer.id(), Token::IF);
    let mut lexer = Lexer::new("THEN".to_string());
    assert_eq!(lexer.id(), Token::THEN);
    let mut lexer = Lexer::new("ELSE".to_string());
    assert_eq!(lexer.id(), Token::ELSE);
    // let mut lexer = Lexer::new("WRITELN".to_string());
    // assert_eq!(lexer.id(), Token::WRITELN);
    let mut lexer = Lexer::new("identifier".to_string());
    assert_eq!(lexer.id(), Token::ID("identifier".to_string()));
  }
  
  #[test]
  fn get_next_token() {
    let mut lexer = Lexer::new("-(10 + (2 * 3))".to_string());
    assert_eq!(lexer.get_next_token(), Token::MINUS);
    assert_eq!(lexer.get_next_token(), Token::LPAR);
    assert_eq!(lexer.get_next_token(), Token::INTEGER(10));
    assert_eq!(lexer.get_next_token(), Token::PLUS);
    assert_eq!(lexer.get_next_token(), Token::LPAR);
    assert_eq!(lexer.get_next_token(), Token::INTEGER(2));
    assert_eq!(lexer.get_next_token(), Token::MULTIPLY);
    assert_eq!(lexer.get_next_token(), Token::INTEGER(3));
    assert_eq!(lexer.get_next_token(), Token::RPAR);
    assert_eq!(lexer.get_next_token(), Token::RPAR);
  }

  // #[test]
  // fn type_declaration() {
  //   let mut lexer = Lexer::new("first, second, third: INTEGER;".to_string());
  //   assert_eq!(lexer.get_next_token(), Token::ID("first".to_string()));
  //   assert_eq!(lexer.get_next_token(), Token::COMMA);
  //   assert_eq!(lexer.get_next_token(), Token::ID("second".to_string()));
  //   assert_eq!(lexer.get_next_token(), Token::COMMA);
  //   assert_eq!(lexer.get_next_token(), Token::ID("third".to_string()));
  //   assert_eq!(lexer.get_next_token(), Token::COLON);
  //   assert_eq!(lexer.get_next_token(), Token::TYPE_SPEC(Type::INTEGER));
  //   assert_eq!(lexer.get_next_token(), Token::SEMICOLON);
  //   let mut lexer = Lexer::new("first: BOOLEAN;".to_string());
  //   assert_eq!(lexer.get_next_token(), Token::ID("first".to_string()));
  //   assert_eq!(lexer.get_next_token(), Token::COLON);
  //   assert_eq!(lexer.get_next_token(), Token::TYPE_SPEC(Type::BOOLEAN));
  //   assert_eq!(lexer.get_next_token(), Token::SEMICOLON);
  // }

  #[test]
  fn variable_assignment() {
    let mut lexer = Lexer::new("number := 42;".to_string());
    assert_eq!(lexer.get_next_token(), Token::ID("number".to_string()));
    assert_eq!(lexer.get_next_token(), Token::ASSIGN);
    assert_eq!(lexer.get_next_token(), Token::INTEGER(42));
    assert_eq!(lexer.get_next_token(), Token::SEMICOLON);
  }
}
