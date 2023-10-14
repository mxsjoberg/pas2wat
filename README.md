![pas2wat.png](pas2wat.png)

# Pascal-to-WebAssembly text-format compiler in Rust

This is an experimental Pascal `.pas` to WebAssembly text-format `.wat` compiler written in Rust.

[[start here](https://github.com/michaelsjoeberg/pas2wat/blob/main/src/main.rs)]

## TODO

- make SEMICOLON optional (no reason to require in input since omitted in webassembly)
- use func export for procedures (callable within module)
- see data.wat and data.js for implementing string using memory

## manual

Install Rust and Cargo, then:

- `cargo test` to run unit tests

- `cargo run <filename>.pas` to run the compiler with `<filename>.pas` as input, output is `<filename>.wat`

The generated file is in WebAssembly text-format, test using [wat2wasm online tool](https://webassembly.github.io/wabt/demo/wat2wasm/), or WebAssembly Binary Toolkit:

- build the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt), make wat2wasm executable, and add to PATH, i.e. `export PATH=$PATH:/path/to/wat2wasm`

- `wat2wasm <filename>.wat` to compile text-format into binary format, output is `<filename>.wasm`
    
- `wat2wasm <filename>.wat -v` to view the binary format

For example, to compile the file `test.pas` and view output in binary format: 

- `cargo run test.pas; wat2wasm test.wat -v`

## debugging

The compiler can be verbose:

- set `DEBUG` as true to enter debug mode
- set `DEBUG_WITH_INPUT` as true to instruct the compiler to look for input file (otherwise expecting call to functions)
- set `DEBUG_SHOW_CHAR` as true to show characters recognised by the lexer
- set `DEBUG_SHOW_CHAR` as true to show characters recognised by the lexer
- set `DEBUG_SHOW_TOKEN` as true to show tokens consumed by the parser
- set `DEBUG_SHOW_TREE` as true to show the AST representation of the Pascal program, this is the output from parser
- set `DEBUG_SHOW_SYMBOL_TABLE` as true to show the symbol table, which contains variable decalarations
- set `DEBUG_SHOW_ASSIGNMENT_TABLE` as true to show the assignment table, which contains variable assignments used by the evaluator

The compiler can generate verbose code:

- set `OUTPUT_VERBOSE` as true to include compiler-related comments in the generated code, this does not include comments in the Pascal program (default is true)
