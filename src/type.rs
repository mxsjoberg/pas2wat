/*

  Types

  INTEGER     : INTEGER(_)
  LONGINT     : INTEGER(_)
  REAL        : REAL(_)

  TODO

  SMALLINT    : INTEGER(_)
  BOOLEAN     : INTEGER(1) | INTEGER(0)
  STRING      : 
  CHAR        :
  BYTE        :

*/

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
  INTEGER,
  REAL,
}