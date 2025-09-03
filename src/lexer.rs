// Import the Token enum, which defines all possible token types.
use crate::token::Token;

/// Tokenizes the input source string into a vector of tokens.
///
/// This lexer scans the input character by character, recognizing keywords, identifiers,
/// numbers, and symbols, and produces a corresponding sequence of `Token` values.
///
/// # Arguments
/// * `input` - The source code as a string slice.
///
/// # Returns
/// * `Vec<Token>` - A vector containing the tokens found in the input.
pub fn tokenize(input: &str) -> Vec<Token> {
    // Create a peekable iterator over the input characters.
    let mut chars = input.chars().peekable();
    // Vector to store the resulting tokens.
    let mut tokens = Vec::new();

    // Main loop: process each character until the end of input.
    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace characters (space, newline, tab).
            ' ' | '\n' | '\t' => {
                chars.next();
            }

            // Parse numeric literals (integers).
            '0'..='9' => {
                let mut num = String::new();
                // Collect consecutive digits into a string.
                while let Some(&c @ '0'..='9') = chars.peek() {
                    num.push(c);
                    chars.next();
                }
                // Convert the string to an integer and create a Number token.
                tokens.push(Token::Number(num.parse().unwrap()));
            }

            // Parse identifiers and keywords.
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                // Collect consecutive valid identifier characters (letters, digits, underscore).
                while let Some(&c @ ('a'..='z' | 'A'..='Z' | '_' | '0'..='9')) = chars.peek() {
                    ident.push(c);
                    chars.next();
                }
                // Check for reserved keywords; otherwise, treat as identifier.
                match ident.as_str() {
                    "int" => tokens.push(Token::Int),
                    "return" => tokens.push(Token::Return),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }

            // Single-character tokens for operators and punctuation.
            '+' => { tokens.push(Token::Plus); chars.next(); } // Plus operator
            '-' => { tokens.push(Token::Minus); chars.next(); } // Minus operator
            '*' => { tokens.push(Token::Star); chars.next(); } // Multiplication operator
            '/' => { tokens.push(Token::Slash); chars.next(); } // Division operator
            '(' => { tokens.push(Token::LParen); chars.next(); } // Left parenthesis
            ')' => { tokens.push(Token::RParen); chars.next(); } // Right parenthesis
            '{' => { tokens.push(Token::LBrace); chars.next(); } // Left brace
            '}' => { tokens.push(Token::RBrace); chars.next(); } // Right brace
            ';' => { tokens.push(Token::Semicolon); chars.next(); } // Semicolon

            // Any other character is unexpected and causes a panic.
            _ => panic!("Unexpected character: {}", ch),
        }
    }
    // Return the vector of tokens.
    tokens
}
