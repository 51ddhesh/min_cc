# min_cc

`min_cc` is an ultra minimal C compiler written in Rust. It demonstrates the basics of compiler construction, including tokenization, parsing, AST generation, and x86_64 assembly code generation for simple arithmetic expressions in a C `main` function.

## Project Overview

This project parses a minimal subset of C programs of the form:

```c
int main() {
	 return <arithmetic expression>;
}
```

It generates x86_64 assembly code, which can be assembled and linked to produce a Linux executable. Only integer arithmetic expressions are supported.

## Project Structure

```
min_cc/
├── Cargo.toml         # Rust project manifest
├── src/
│   ├── main.rs        # Entry point: orchestrates compilation
│   ├── lexer.rs       # Tokenizer for C source code
│   ├── parser.rs      # Recursive descent parser for arithmetic expressions
│   ├── ast.rs         # AST (Abstract Syntax Tree) definitions
│   ├── codegen.rs     # x86_64 assembly code generator
│   └── token.rs       # Token definitions
├── examples/
│   └── test.c         # Example C file for testing
└── run.sh             # Quick start script (optional)
```

**Key files:**
- `src/main.rs`: Reads input, runs the compiler pipeline, writes output.
- `src/lexer.rs`: Splits C source into tokens.
- `src/parser.rs`: Parses tokens into an AST.
- `src/ast.rs`: Defines the AST structure.
- `src/codegen.rs`: Converts AST to assembly code.
- `src/token.rs`: Token types used by the lexer and parser.
- `examples/test.c`: Example input file.
- `run.sh`: Script to build and run the project quickly.

## Requirements

- Rust (for building min_cc)
- `nasm` (Netwide Assembler)
- GNU `ld` (Linker)

Install requirements on Ubuntu/Debian:

### Quick Start
Though, not a good practice, you can run the [`run.sh`](./run.sh) to test the project

```bash
chmod +x run.sh
./run.sh # build and run the project with cargo
./out # run the executable
echo $? # check the last return value
```

## Usage

1. **Build the compiler:**

	```bash
	cargo build --release
	```

2. **Compile a C file:**

	```bash
	./target/release/min_cc examples/test.c
	```

	This will generate `output.asm` in the project directory.

3. **Assemble and link the output:**

	```bash
	nasm -f elf64 output.asm -o output.o
	ld output.o -o output
	```

4. **Run the resulting executable:**

	```bash
	./output
	echo $?
	```

	The exit code will be the result of the arithmetic expression in `main`.
