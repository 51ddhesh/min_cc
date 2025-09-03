// This module is responsible for generating x86_64 assembly code from the AST produced by parsing C code.
// It traverses the AST recursively and emits instructions for arithmetic expressions.
// The generated assembly is suitable for use with a Linux system and expects the main function to return an integer.
//
// Rust features used:
// - Pattern matching for AST traversal
// - String formatting and mutation
// - Ownership and borrowing for AST nodes
// - Panic for error handling on unsupported operators
//
// Functionality:
// - Converts an arithmetic expression AST into assembly code
// - Handles binary operations and integer literals
// - Produces a minimal Linux program that exits with the result of main()
use crate::ast::Expr;
use crate::token::Token;

/// Generates x86_64 assembly code from an expression AST.
/// 
/// # Arguments
/// * `expr` - The root of the AST representing the return value of main().
/// 
/// # Returns
/// A String containing the full assembly code for a minimal Linux program.
pub fn generate_asm(expr: &Expr) -> String {
    // Buffer to accumulate instructions for the main function
    let mut code = String::new();
    // Recursively generate code for the expression
    gen_expr(expr, &mut code);
    // Add the return instruction for main
    code.push_str("    ret\n");

    // The assembly includes:
    // - _start: entry point, calls main, exits with main's return value
    // - main: computes the result and returns it in rax
    format!(
        "global _start\n        global main\n        section .text\n\n        _start:\n            call main\n            mov rdi, rax\n            mov rax, 60\n            syscall\n\n        main:\n        {}",
            code
    )
}

/// Recursively walks the AST and generates assembly instructions for each node.
/// Handles numbers and binary operations (+, -, *, /).
/// 
/// # Arguments
/// * `expr` - The AST node to generate code for.
/// * `code` - Mutable string buffer to append instructions.
fn gen_expr(expr: &Expr, code: &mut String) {
    match expr {
        // For a number literal, move its value into rax
        Expr::Number(n) => {
            code.push_str(&format!("    mov rax, {}\n", n));
        }
        // For a binary operation, recursively generate code for operands
        Expr::BinaryOp { op, left, right } => {
            // Evaluate right operand first and push its result onto the stack
            gen_expr(right, code);         // Evaluate right expr and put result in rax
            code.push_str("    push rax\n"); // Save right operand to stack
            gen_expr(left, code);          // Evaluate left expr and put result in rax
            code.push_str("    pop rcx\n");  // Restore right operand to rcx

            // Emit the appropriate instruction based on the operator
            match op {
                Token::Plus => code.push_str("    add rax, rcx\n"), // rax = left + right
                Token::Minus => code.push_str("    sub rax, rcx\n"), // rax = left - right
                Token::Star => code.push_str("    imul rax, rcx\n"), // rax = left * right
                Token::Slash => {
                    // Prepare for signed division: rdx:rax / rcx
                    code.push_str("    cqo\n");     // Sign-extend rax into rdx for division
                    code.push_str("    idiv rcx\n"); // Divide rdx:rax by rcx, result in rax
                }
                _ => panic!("Unsupported operator: {:?}", op), // Panic if operator is not supported
            }
        }
    }
}
