use super::types::{Token};

#[derive(Debug)]
pub struct TokenScanner<'a> {
    pub tokens: &'a Vec<Token>,
    pub cursor: usize,
    length: usize,
}
impl TokenScanner<'_> {
    pub fn new(tokens: &Vec<Token>) -> TokenScanner {
        TokenScanner {
            tokens,
            cursor: 0,
            length: tokens.len(),
        }
    }
    pub fn current(&self) -> Option<Token> {
        return Some(self.tokens[self.cursor].clone());
    }
    pub fn is_at_end(&self) -> bool {
        return self.cursor >= self.length-1;
    }
    pub fn advance(&mut self) -> Option<Token> {
        if self.is_at_end() {
            return None;
        }

        self.cursor+=1;
        return Some(self.tokens[self.cursor].clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::types::TokenType;

    use super::*;

    #[test]
    fn test_is_at_end() {
        // Arrange
        let tokens = &vec![ 
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
        ];
        let mut scanner = TokenScanner::new(tokens);

        // Act
        scanner.advance();
        scanner.advance();
        let is_at_end = scanner.is_at_end();

        // Assert
        assert!(!is_at_end);
    }

    #[test]
    fn test_is_at_end2() {
        // Arrange
        let tokens = &vec![ 
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 },
        ];
        let mut scanner = TokenScanner::new(tokens);

        // Act
        scanner.advance();
        scanner.advance();
        scanner.advance();
        let is_at_end = scanner.is_at_end();

        // Assert
        assert!(is_at_end);
    }
}
