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
