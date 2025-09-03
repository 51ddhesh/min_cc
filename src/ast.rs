// This module defines the AST (Abstract Syntax Tree) for arithmetic expressions.
// The AST is used to represent parsed expressions before code generation.
//
// Rust features used:
// - Enums for representing different expression types
// - Box for heap allocation and recursive data structures
// - Derive(Debug) for easy printing and debugging
//
// Functionality:
// - Models integer literals and binary operations (+, -, *, /)
// - Used by the parser and code generator to represent and process expressions
use crate::token::Token;

/// Expression node for the AST.
/// 
/// - Number: Represents an integer literal.
/// - BinaryOp: Represents a binary operation (e.g., +, -, *, /) with left and right operands.
#[derive(Debug)]
pub enum Expr {
    /// Integer literal
    Number(i64),
    /// Binary operation (e.g., +, -, *, /)
    BinaryOp {
        op: Token,           // Operator token (+, -, *, /)
        left: Box<Expr>,     // Left operand (another Expr)
        right: Box<Expr>,    // Right operand (another Expr)
    },
}

