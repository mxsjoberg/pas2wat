use crate::config::*;
use crate::r#const::*;
use crate::r#type::Type;
use crate::token::Token;

/*

  TODO: add symbols (implemented and not)

*/

// lexer
// ----------------------------------------------------
pub struct Lexer {
  text: String,
  position: i32,
  current_char: Option<char>,
  comment_multiline: bool,
}
impl Lexer {
  // new : Lexer
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
  // look_ahead : Option<char>
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
    if DEBUG && DEBUG_SHOW_CHAR { println!("{}{}{:?}", FORMAT_TAB, FORMAT_SPACE.repeat(2), self.current_char); }
    self.position += 1;
    // no more text input
    if self.position > self.text.len() as i32 - 1 {
      // end of file
      self.current_char = None;
    }
    // otherwise
    else {
      // usize : target machine specific
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
    if self.comment_multiline == true {
      // increment until closing curly
      while let Some(_char) = self.current_char {
        if _char != CHAR_RCUR {
          self.increment();
        } else {
          self.comment_multiline = false;
          // closing curly brace
          self.increment();
          break;
        }
      }
    } else {
      // increment until newline
      while let Some(_char) = self.current_char {
        if _char != CHAR_NEWLINE {
          self.increment();
        } else {
          // newline
          // self.increment();
          break;
        }
      }
    }
  }
  // number : Token
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
      // increment until not number
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
  // id : Token
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
      KEY_CONST => {
        return Token::CONST;
      },
      KEY_DIV => {
        return Token::INTEGER_DIV;
      },
      KEY_MOD => {
        return Token::INTEGER_MOD;
      },
      KEY_ODD => {
        return Token::ODD;
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
      KEY_SMALLINT => {
        return Token::TYPE_SPEC(Type::INTEGER);
      },
      KEY_REAL => {
        return Token::TYPE_SPEC(Type::REAL);
      },
      KEY_BOOLEAN => {
        return Token::TYPE_SPEC(Type::BOOLEAN);
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
  // get_next_token : Token
  pub fn get_next_token(&mut self) -> Token {
    while let Some(_char) = self.current_char {
      // whitespace
      if _char.is_whitespace() {
        self.skip_whitespace();
        continue;
      }
      // comment multiline
      if _char == CHAR_LCUR {
        self.comment_multiline = true;
        self.increment();
        self.skip_comment();
        continue;
      }
      // comment
      if _char == CHAR_DIVIDE && self.look_ahead() == Some(CHAR_DIVIDE) {
        self.increment();
        self.skip_comment();
        continue; 
      }
      // identifier (variable or keyword)
      if _char.is_alphabetic() {
        return self.id();
      }
      // number (INTEGER or REAL)
      // 10-base is decimal number (radix)
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
        // end of program
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
#[cfg(test)]
mod tests {
  use super::*;
  // unit tests
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
  fn increment() {
    let mut lexer = Lexer::new("42".to_string());
    assert_eq!(lexer.current_char, Some('4'));
    lexer.increment();
    assert_eq!(lexer.current_char, Some('2'));
    lexer.increment();
    assert_eq!(lexer.current_char, None);
  }
  #[test]
  fn skip_whitespace() {
    let mut lexer = Lexer::new("1  2".to_string());
    assert_eq!(lexer.current_char, Some('1'));
    lexer.increment();
    assert_eq!(lexer.current_char, Some(' '));
    lexer.skip_whitespace();
    assert_eq!(lexer.current_char, Some('2'));
  }
  #[test]
  fn skip_comment() {
    let mut lexer = Lexer::new("{ 2 }".to_string());
    assert_eq!(lexer.current_char, Some('{'));
    lexer.skip_comment();
    assert_eq!(lexer.current_char, None);
  }
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
    let mut lexer = Lexer::new("PROGRAM".to_string());
    assert_eq!(lexer.id(), Token::PROGRAM);
    let mut lexer = Lexer::new("VAR".to_string());
    assert_eq!(lexer.id(), Token::VAR);
    let mut lexer = Lexer::new("CONST".to_string());
    assert_eq!(lexer.id(), Token::CONST);
    let mut lexer = Lexer::new("DIV".to_string());
    assert_eq!(lexer.id(), Token::INTEGER_DIV);
    let mut lexer = Lexer::new("MOD".to_string());
    assert_eq!(lexer.id(), Token::INTEGER_MOD);
    let mut lexer = Lexer::new("BEGIN".to_string());
    assert_eq!(lexer.id(), Token::BEGIN);
    let mut lexer = Lexer::new("END".to_string());
    assert_eq!(lexer.id(), Token::END);
    let mut lexer = Lexer::new("INTEGER".to_string());
    assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::INTEGER));
    let mut lexer = Lexer::new("LONGINT".to_string());
    assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::INTEGER));
    let mut lexer = Lexer::new("SMALLINT".to_string());
    assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::INTEGER));
    let mut lexer = Lexer::new("REAL".to_string());
    assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::REAL));
    let mut lexer = Lexer::new("BOOLEAN".to_string());
    assert_eq!(lexer.id(), Token::TYPE_SPEC(Type::BOOLEAN));
    let mut lexer = Lexer::new("TRUE".to_string());
    assert_eq!(lexer.id(), Token::TRUE);
    let mut lexer = Lexer::new("FALSE".to_string());
    assert_eq!(lexer.id(), Token::FALSE);
    let mut lexer = Lexer::new("PACKED".to_string());
    assert_eq!(lexer.id(), Token::PACKED);
    let mut lexer = Lexer::new("ARRAY".to_string());
    assert_eq!(lexer.id(), Token::ARRAY);
    let mut lexer = Lexer::new("OF".to_string());
    assert_eq!(lexer.id(), Token::OF);
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
    let mut lexer = Lexer::new("WRITELN".to_string());
    assert_eq!(lexer.id(), Token::WRITELN);
    let mut lexer = Lexer::new("variable".to_string());
    assert_eq!(lexer.id(), Token::ID("variable".to_string()));
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
  // functional tests
  #[test]
  fn type_declaration() {
    let mut lexer = Lexer::new("first, second, third: INTEGER;".to_string());
    assert_eq!(lexer.get_next_token(), Token::ID("first".to_string()));
    assert_eq!(lexer.get_next_token(), Token::COMMA);
    assert_eq!(lexer.get_next_token(), Token::ID("second".to_string()));
    assert_eq!(lexer.get_next_token(), Token::COMMA);
    assert_eq!(lexer.get_next_token(), Token::ID("third".to_string()));
    assert_eq!(lexer.get_next_token(), Token::COLON);
    assert_eq!(lexer.get_next_token(), Token::TYPE_SPEC(Type::INTEGER));
    assert_eq!(lexer.get_next_token(), Token::SEMICOLON);
    let mut lexer = Lexer::new("first: BOOLEAN;".to_string());
    assert_eq!(lexer.get_next_token(), Token::ID("first".to_string()));
    assert_eq!(lexer.get_next_token(), Token::COLON);
    assert_eq!(lexer.get_next_token(), Token::TYPE_SPEC(Type::BOOLEAN));
    assert_eq!(lexer.get_next_token(), Token::SEMICOLON);
  }
  #[test]
  fn variable_assignment() {
    let mut lexer = Lexer::new("number := 42;".to_string());
    assert_eq!(lexer.get_next_token(), Token::ID("number".to_string()));
    assert_eq!(lexer.get_next_token(), Token::ASSIGN);
    assert_eq!(lexer.get_next_token(), Token::INTEGER(42));
    assert_eq!(lexer.get_next_token(), Token::SEMICOLON);
  }
}