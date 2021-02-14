# pawa: pascal-to-wat prototype compiler

This is a prototype compiler in Rust for Pascal targeting WebAssembly text-format. Focus is on structure and implementation for demonstration projects, where goal for project is to be adaptable to work with other programming languages.

## folder structure and files

**src/main.rs** : Source code for compiler.

**test.\*** : A simple test used to verify that everything works.

**/test\_case\_\*\_\*.\*** : Test cases used for testing implemented definitions.

**/Cargo.\*** : Cargo build files.

## manual

First, install both Rust and Cargo, which is the Rust package manager. If already installed, or after successful installation:

- <code>cargo run \<filename\>.pas</code> in project root folder to run the compiler with <code>\<filename\>.pas</code> as input, output is <code>\<filename\>.wat</code>.

The generated file is in WebAssembly text-format, which is enough for testing using the [WebAssembly wat2wasm online tool](https://webassembly.github.io/wabt/demo/wat2wasm/).

**Alternatively**: the WebAssembly Binary Toolkit can be used to compile the file in text format into binary format. Note that this is not required to test the generated modules.

- build the [WebAssembly Binary Toolkit](https://github.com/WebAssembly/wabt), make wat2wasm executable, and add to PATH, i.e. <code>export PATH=$PATH:\<path\_to\_wat2wasm\></code> (MacOS)

- <code>wat2wasm \<filename\>.wat</code> in root folder to compile file in text format into binary format, output is <code>\<filename\>.wasm</code>
	
- <code>wat2wasm \<filename\>.wat -v</code> in root folder to view the binary format of the generated module

For example, to compile the file test.pas and view output in binary format: <code>cargo run test.pas; wat2wasm test.wat -v</code>

Debugging: the compiler can be configured to output additional information, such as tokens, AST, and symbol table.

- set <code>DEBUG\_SHOW\_CHAR</code> as true to show characters recognised by the lexer
- set <code>DEBUG\_SHOW\_TOKEN</code> as true to show tokens consumed by the parser
- set <code>DEBUG\_SHOW\_TREE</code> as true to show the AST representation of the Pascal program, this is the output from parser
- set <code>DEBUG\_SHOW\_SYMBOL\_TABLE</code> as true to show the symbol table, which contains variable decalarations
- set <code>DEBUG\_SHOW\_ASSIGNMENT_TABLE</code> as true to show the assignment table, which contains variable assignments used by the evaluator
- set <code>OUTPUT\_VERBOSE</code> as true to include compiler-related comments in the generated code, this does not include comments in the Pascal program

Note that <code>OUTPUT\_VERBOSE</code> is set to true by default.