// Entry point for the minimal C compiler.
// This file coordinates the compilation process:
// 1. Reads the input C file
// 2. Tokenizes the source
// 3. Parses tokens into an AST
// 4. Generates x86_64 assembly from the AST
// 5. Writes the assembly to output.asm
mod lexer;
mod parser;
mod ast;
mod codegen;
mod token;

use std::env; // For reading command-line arguments
use std::fs;  // For file I/O

use lexer::tokenize;         // Tokenizer for C source
use parser::Parser;          // Parser for tokens to AST
use codegen::generate_asm;   // Code generator for assembly

/// Main function: orchestrates the compilation pipeline.
///
/// Steps:
/// 1. Checks for correct usage (expects one argument: input file)
/// 2. Reads the input C file
/// 3. Tokenizes the input
/// 4. Parses tokens into an AST
/// 5. Generates assembly code from the AST
/// 6. Writes the assembly to output.asm
fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the user provided exactly one input file
    if args.len() != 2 {
        eprintln!("Usage: c_compiler <file.c>");
        std::process::exit(1);
    }

    // Read the input C source file
    let input = fs::read_to_string(&args[1])
        .expect("Failed to read input file");

    // Tokenize the input source code
    let tokens = tokenize(&input);
    // Parse tokens into an AST
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    // Generate x86_64 assembly from the AST
    let asm = generate_asm(&ast);

    // Write the generated assembly to output.asm
    fs::write("output.asm", asm).expect("Failed to write output.asm");
    println!("Assembly written to output.asm");
}
