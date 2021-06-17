# pas2wat: pascal-to-wat

This is a work-in-progress compiler (lexer, parser, and code generation) for a subset of Pascal targeting WebAssembly text-format. The end-goal is to build a versatile framework for implementing compilers or interpreters in Rust.

## folder structure

- src/
    - main.rs
        - [start here](https://github.com/michaelsjoeberg/pas2wat/blob/main/src/main.rs)
    - config.rs
        - configuration file
        - settings for debugging
    - const.rs
        - constants used in other files (keywords, strings, opcodes, etc.)
    - type.rs
        - source code for type used in other files
    - token.rs
        - source code for token type used in other files
    - lexer.rs
        - source code for lexer
    - ast.rs
        - source code for AST data structure
    - parser.rs
        - source code for parser
    - evaluator.rs
        - source code for partial evaluator
        - (could also be adapted to use as interpreter)
    - emitter.rs
        - source code for code generation

- programs/
    - various programs to validate compilation process (use cases)

## manual

First, install both Rust and Cargo, the Rust package manager. If already installed, or after successful installation:

- `cargo test` to run unit tests

- `cargo run <filename>.pas` to run the compiler with `<filename>.pas` as input, output is `<filename>.wat`

The generated file is in WebAssembly text-format, which is enough for testing using the [wat2wasm online tool](https://webassembly.github.io/wabt/demo/wat2wasm/), could also use the WebAssembly Binary Toolkit to compile text-format (wat) into binary format (wasm):

- build the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt), make wat2wasm executable, and add to PATH, i.e. `export PATH=$PATH:/path/to/wat2wasm`

- `wat2wasm <filename>.wat` to compile text-format into binary format, output is `<filename>.wasm`
    
- `wat2wasm <filename>.wat -v` to view the binary format

For example, to compile the file `test.pas` and view output in binary format: 

- `cargo run test.pas; wat2wasm test.wat -v`

## debugging

The compiler can be configured to output additional information, such as tokens, AST, and symbol table:

- set `DEBUG` as true to enter debug mode
- set `DEBUG_WITH_INPUT` as true to instruct the compiler to look for input file (otherwise expecting call to functions)
- set `DEBUG_SHOW_CHAR` as true to show characters recognised by the lexer
- set `DEBUG_SHOW_CHAR` as true to show characters recognised by the lexer
- set `DEBUG_SHOW_TOKEN` as true to show tokens consumed by the parser
- set `DEBUG_SHOW_TREE` as true to show the AST representation of the Pascal program, this is the output from parser
- set `DEBUG_SHOW_SYMBOL_TABLE` as true to show the symbol table, which contains variable decalarations
- set `DEBUG_SHOW_ASSIGNMENT_TABLE` as true to show the assignment table, which contains variable assignments used by the evaluator

Other useful information:

- set `OUTPUT_VERBOSE` as true to include compiler-related comments in the generated code, this does not include comments in the Pascal program (default is true)