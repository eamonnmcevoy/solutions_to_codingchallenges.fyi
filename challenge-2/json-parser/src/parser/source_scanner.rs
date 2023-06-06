use std::str::Chars;

use super::types::{ScanError, Token, TokenType};

pub struct SourceScanner<'a> {
    pub token_start: usize,
    pub cursor: usize,
    line: usize,
    line_start: usize,
    source: &'a str,
    length: usize,
    char_iter: Chars<'a>,
    pub tokens: Vec<Token>,
    pub errors: Vec<ScanError>,
}
impl<'a> SourceScanner<'a> {
    pub fn new(source: &'a str) -> SourceScanner<'a> {
        SourceScanner {
            token_start: 0,
            cursor: 0,
            line: 1,
            line_start: 0,
            source: source,
            length: source.len(),
            char_iter: source.chars().clone(),
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.char_iter.clone().peekable().peek().unwrap().clone();
    }
    pub fn is_at_end(&self) -> bool {
        return self.cursor >= self.length;
    }
    pub fn advance(&mut self) -> char {
        self.cursor += 1;
        let char = self.char_iter.next().unwrap();
        if char == '\n' {
            self.line += 1;
            self.line_start = self.cursor;
        }
        // dbg!(char);
        return char;
    }
    pub fn skip_whitespace(&mut self) {
        while self.peek() == ' '
            || self.peek() == '\r'
            || self.peek() == '\t'
            || self.peek() == '\n'
        {
            self.advance();
        }
        self.token_start = self.cursor;
    }
    pub fn add_token(&mut self, token_type: TokenType) {
        let lexeme = self.source[self.token_start..self.cursor].to_string();
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: lexeme,
            line: self.line,
            position_start: self.token_start,
            position_end: self.cursor,
        });
    }
    pub fn report_error(&mut self, message: String) -> ScanError {
        let lexeme = self.source[self.token_start..self.cursor].to_string();
        let scan_error = ScanError {
            line: self.line,
            line_start: self.line_start,
            lexeme: lexeme,
            position_start: self.token_start,
            position_end: self.cursor,
            message: message,
        };
        self.errors.push(scan_error.clone());
        return scan_error;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_at_end() {
        // Arrange
        let mut scanner = SourceScanner::new("abc");

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
        let mut scanner = SourceScanner::new("abc");

        // Act
        scanner.advance();
        scanner.advance();
        scanner.advance();
        let is_at_end = scanner.is_at_end();

        // Assert
        assert!(is_at_end);
    }

    #[test]
    fn test_add_token() {
        // Arrange
        let mut scanner = SourceScanner::new("abc");

        // Act
        scanner.advance();
        scanner.advance();
        scanner.add_token(TokenType::String);
        let token = &scanner.tokens[0];

        // Assert
        assert!(token.token_type == TokenType::String);
        assert!(token.lexeme == "ab");
    }

    #[test]
    fn test_report_error() {
        // Arrange
        let mut scanner = SourceScanner::new("abc");

        // Act
        scanner.advance();
        scanner.advance();
        scanner.report_error("test".to_string());
        let error = &scanner.errors[0];

        // Assert
        assert!(error.message == "test");
        assert!(error.lexeme == "ab");
    }

    #[test]
    fn test_peek() {
        // Arrange
        let scanner = SourceScanner::new("abc");

        // Act
        let peek = scanner.peek();

        // Assert
        assert!(peek == 'a');
    }

    #[test]
    fn test_advance_peek() {
        // Arrange
        let mut scanner = SourceScanner::new("abc");

        // Act
        scanner.advance();
        let peek = scanner.peek();

        // Assert
        assert!(peek == 'b');
    }

    #[test]
    fn test_advance() {
        let mut scanner = SourceScanner::new("ab\ncd");
        assert_eq!(scanner.advance(), 'a');
        assert_eq!(scanner.cursor, 1);
        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.advance(), 'b');
        assert_eq!(scanner.cursor, 2);
        assert_eq!(scanner.line, 1);
        assert_eq!(scanner.advance(), '\n');
        assert_eq!(scanner.cursor, 3);
        assert_eq!(scanner.line, 2);
        assert_eq!(scanner.advance(), 'c');
        assert_eq!(scanner.cursor, 4);
        assert_eq!(scanner.line, 2);
        assert_eq!(scanner.advance(), 'd');
        assert_eq!(scanner.cursor, 5);
        assert_eq!(scanner.line, 2);
    }
}
