# pas2wat: pascal-to-wat

This is a work-in-progress compiler (lexer, parser, and code generation) for a subset of Pascal targeting WebAssembly text-format. The end-goal is to build a versatile framework for implementing compilers or interpreters in Rust.

## folder structure

- src
    - main : [start here](https://github.com/michaelsjoeberg/pas2wat/blob/main/src/main.rs)
    - config : configuration file (settings for debugging etc.)
    - const : constants used in other files (keywords, strings, opcodes, etc.)
    - type : source code for type used in other files
    - token : source code for token type used in other files
    - lexer : source code for lexer
    - ast : source code for AST data structure (IR)
    - parser : source code for parser
    - evaluator : source code for partial evaluator (could also be adapted to use as interpreter)
    - emitter : source code for code generation

- programs : various programs to validate compilation process (test cases)

## manual

First, install both Rust and Cargo, the Rust package manager. If already installed, or after successful installation:

- `cargo test` to run unit tests

- `cargo run <filename>.pas` to run the compiler with `<filename>.pas` as input, output is `<filename>.wat`.

The generated file is in WebAssembly text-format, which is enough for testing using the [wat2wasm online tool](https://webassembly.github.io/wabt/demo/wat2wasm/).

**Alternatively**: the WebAssembly Binary Toolkit can be used to compile the file in text format into binary format.

- build the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt), make wat2wasm executable, and add to PATH, i.e. `export PATH=$PATH:/path/to/wat2wasm`

- `wat2wasm <filename>.wat` in root folder to compile file in text format into binary format, output is `<filename>.wasm`
    
- `wat2wasm <filename>.wat -v` in root folder to view the binary format of the generated module

For example, to compile the file test.pas and view output in binary format: `cargo run test.pas; wat2wasm test.wat -v`

## debugging

The compiler can be configured to output additional information, such as tokens, AST, and symbol table.

- set `DEBUG` as true to enter debug mode
- set `DEBUG_WITH_INPUT` as true to instruct the compiler to look for input file (otherwise expecting call to functions)
- set `DEBUG_SHOW_CHAR` as true to show characters recognised by the lexer
- set `DEBUG_SHOW_CHAR` as true to show characters recognised by the lexer
- set `DEBUG_SHOW_TOKEN` as true to show tokens consumed by the parser
- set `DEBUG_SHOW_TREE` as true to show the AST representation of the Pascal program, this is the output from parser
- set `DEBUG_SHOW_SYMBOL_TABLE` as true to show the symbol table, which contains variable decalarations
- set `DEBUG_SHOW_ASSIGNMENT_TABLE` as true to show the assignment table, which contains variable assignments used by the evaluator

Other useful information.

- set `OUTPUT_VERBOSE` as true to include compiler-related comments in the generated code, this does not include comments in the Pascal program (default is true)