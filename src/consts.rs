// types
pub static NTYPE_INTEGER            : &'static str = "i32";
pub static NTYPE_REAL               : &'static str = "f64";
// errors
pub const PANIC_SYNTAX              : &str = "Invalid syntax";
pub const PANIC_TYPE_DECLARATION    : &str = "Invalid type declaration";
pub const PANIC_ARRAY               : &str = "Invalid array type";
pub const PANIC_VAR_NOT_DECLARAED   : &str = "Variable not declared";
pub const PANIC_COMPILE             : &str = "Could not compile";
pub const PANIC_WRITE               : &str = "Could not write to file";
pub const PANIC_READ                : &str = "Could not read from file";
pub const PANIC_FILE                : &str = "No source file provided";
pub const PANIC_EVAL                : &str = "Could not evaluate";
// formatting
pub const FORMAT_SPACE              : &str = " ";
pub const FORMAT_TAB                : &str = "  ";
pub const FORMAT_NEWLINE            : &str = "\n";
// opcodes
pub const WASM_EQUAL                : &str = ".eq";
pub const WASM_NOT_EQUAL            : &str = ".ne";
pub const WASM_TRUNCATE             : &str = ".trunc";
pub const WASM_PLUS                 : &str = ".add";
pub const WASM_MINUS                : &str = ".sub";
pub const WASM_MULTIPLY             : &str = ".mul";
pub const WASM_DIVIDE               : &str = ".div";
pub const WASM_INTEGER_DIV          : &str = ".div_s";
pub const WASM_INTEGER_MOD          : &str = ".rem_s";
pub const WASM_CONVERT              : &str = ".convert";
pub const WASM_GREATER_THAN         : &str = ".gt";
pub const WASM_GREATER_EQUAL        : &str = ".ge";
pub const WASM_LESS_THAN            : &str = ".lt";
pub const WASM_LESS_EQUAL           : &str = ".le";
pub const WASM_NEGATION             : &str = ".neg";
pub const WASM_CONSTANT             : &str = ".const";
pub const WASM_VARIABLE             : &str = "get_local";
pub const WASM_ASSIGNMENT           : &str = "set_local";
pub const WASM_DECLARATION          : &str = "param";
pub const WASM_RESULT               : &str = "result";
pub const WASM_EXPORT               : &str = "export";
pub const WASM_FUNCTION             : &str = "func";
pub const WASM_MODULE               : &str = "module";
pub const WASM_WRITE				: &str = "call $log"; // TODO: is there a better way?
pub const WASM_BLOCK                : &str = "block";
pub const WASM_LOOP                 : &str = "loop";
pub const WASM_BREAK                : &str = "br";
pub const WASM_BREAK_IF             : &str = "br_if";
pub const WASM_IF                   : &str = "if";
pub const WASM_THEN                 : &str = "then";
pub const WASM_ELSE                 : &str = "else";
// misc
pub const WASM_WAT                  : &str = ".wat";
pub const WASM_JS                   : &str = ".js";