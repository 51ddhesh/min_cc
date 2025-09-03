// It parses a token stream into an AST representing arithmetic expressions in a main function.
//
// Rust features used:
// - Pattern matching for token and AST construction
// - Ownership and borrowing for token stream and AST nodes
// - Error handling via panic for unexpected tokens
//
// Functionality:
// - Converts a vector of tokens into an AST
// - Handles operator precedence and associativity for +, -, *, /
// - Expects a minimal C program structure: int main() { return <expr>; }
use crate::token::Token;
use crate::ast::Expr;

/// Parser that takes a list of tokens and produces an AST.
/// 
/// Fields:
/// - tokens: Vector of tokens to parse
/// - pos: Current position in the token stream
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    /// Creates a new parser with the given tokens.
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {tokens, pos: 0}
    }

    /// Returns the current token, or None if at end of input.
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    /// Consumes the current token if it matches the expected token, otherwise panics.
    /// Used to enforce the expected structure of the input program.
    fn eat(&mut self, expected: &Token) {
        if self.current() == Some(expected) {
            self.pos += 1;
        } else {
            panic!("Expected {:?}, got {:?}", expected, self.current());
        }
    }

    /// Parses a full minimal C program of the form: int main() { return <expr>; }
    /// Returns the parsed expression AST.
    pub fn parse(&mut self) -> Expr {
        // Expect the sequence of tokens for a minimal main function
        self.eat(&Token::Int); // 'int'
        self.eat(&Token::Ident("main".into())); // 'main'
        self.eat(&Token::LParen); // '('
        self.eat(&Token::RParen); // ')'
        self.eat(&Token::LBrace); // '{'
        self.eat(&Token::Return); // 'return'
        let expr = self.parse_expr(); // Parse the arithmetic expression
        self.eat(&Token::Semicolon); // ';'
        self.eat(&Token::RBrace); // '}'
        expr
    }

    /// Parses an expression, starting with addition/subtraction.
    /// This is the entry point for parsing arithmetic expressions.
    fn parse_expr(&mut self) -> Expr {
        self.parse_add_sub()
    }

    /// Parses addition and subtraction, left-associative.
    /// Handles chains of + and - operators, respecting precedence.
    fn parse_add_sub(&mut self) -> Expr {
        let mut node = self.parse_mul_div();

        while let Some(token) = self.current() {
            match token {
                Token::Plus | Token::Minus => {
                    let op = token.clone();
                    self.pos += 1;
                    let right = self.parse_mul_div();
                    node = Expr::BinaryOp {
                        op,
                        left: Box::new(node),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        node
    }

    /// Parses multiplication and division, left-associative.
    /// Handles chains of * and / operators, respecting precedence.
    fn parse_mul_div(&mut self) -> Expr {
        let mut node = self.parse_primary();

        while let Some(token) = self.current() {
            match token {
                Token::Star | Token::Slash => {
                    let op = token.clone();
                    self.pos += 1;
                    let right = self.parse_primary();
                    node = Expr::BinaryOp {
                        op,
                        left: Box::new(node),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        node
    }

    /// Parses a primary expression: number or parenthesized expression.
    /// Handles integer literals and expressions in parentheses.
    fn parse_primary(&mut self) -> Expr {
        match self.current() {
            Some(Token::Number(n)) => {
                let value = *n;
                self.pos += 1;
                Expr::Number(value)
            }
            Some(Token::LParen) => {
                self.pos += 1;
                let expr = self.parse_expr();
                self.eat(&Token::RParen);
                expr
            }
            _ => panic!("Unexpected token: {:?}", self.current()),
        }
    }
}

