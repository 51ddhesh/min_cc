/// Token types produced by the lexer.
///
/// Each variant represents a distinct syntactic element in the source code.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// The `int` keyword, used for declaring integer types.
    Int,
    /// The `return` keyword, used for returning values from functions.
    Return,
    /// An identifier, such as variable or function names.
    /// Contains the identifier's string value.
    Ident(String),
    /// A numeric literal (integer). Contains the parsed value.
    Number(i64),
    /// The plus operator (`+`).
    Plus,
    /// The minus operator (`-`).
    Minus,
    /// The multiplication operator (`*`).
    Star,
    /// The division operator (`/`).
    Slash,
    /// Left parenthesis (`(`), used for grouping expressions or function calls.
    LParen,
    /// Right parenthesis (`)`), used for grouping expressions or function calls.
    RParen,
    /// Left brace (`{`), used to start a block of code.
    LBrace,
    /// Right brace (`}`), used to end a block of code.
    RBrace,
    /// Semicolon (`;`), used to terminate statements.
    Semicolon,
}

