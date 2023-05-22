use super::scanner::Scanner;
use super::types::{ScanError, Token, TokenType};

pub struct Lexer {}
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn get_tokens(&self, source: &str) -> Result<Vec<Token>, Vec<ScanError>> {
        let mut scanner = Scanner::new(source);
        while !scanner.is_at_end() {
            let result = self.get_next_token(&mut scanner);
            match result {
                Ok(token_type) => {
                    scanner.add_token(token_type);
                }
                Err(error) => {
                    scanner.report_error(error);
                }
            }
        }

        if scanner.had_error() {
            Err(scanner.errors)
        } else {
            Ok(scanner.tokens)
        }
    }

    fn get_next_token(&self, scanner: &mut Scanner) -> Result<TokenType, String> {
        scanner.skip_whitespace();

        let c = scanner.advance();
        match c {
            '{' => Ok(TokenType::LeftBrace),
            '}' => Ok(TokenType::RightBrace),
            '[' => Ok(TokenType::LeftBracket),
            ']' => Ok(TokenType::RightBracket),
            ':' => Ok(TokenType::Colon),
            ',' => Ok(TokenType::Comma),
            't' => self.match_keyword(scanner, "rue", TokenType::Boolean),
            'f' => self.match_keyword(scanner, "alse", TokenType::Boolean),
            'n' => self.match_keyword(scanner, "ull", TokenType::Null),
            '"' => self.match_string(scanner),
            '0'..='9' | '-' => self.match_number(scanner),
            _ => Err("Unexpected character".to_string()),
        }
    }

    fn match_keyword(
        &self,
        scanner: &mut Scanner,
        keyword: &str,
        token_type: TokenType,
    ) -> Result<TokenType, String> {
        for (i, expected) in keyword.chars().enumerate() {
            if scanner.advance() != expected {
                return Err(format!("Expected '{}' at index {} of keyword", expected, i));
            }
        }
        Ok(token_type)
    }

    fn match_string(&self, scanner: &mut Scanner) -> Result<TokenType, String> {
        let mut final_char = scanner.peek();
        loop {
            if scanner.is_at_end() {
                break
            }

            if scanner.peek() == '\n' {
                return Err("Unescaped newline in string".to_string());
            }

            if scanner.peek() == '"' {
                final_char = scanner.advance();
                break;
            }

            if scanner.peek() == '\\' {
                final_char = scanner.advance();
                match scanner.peek() {
                    '\\' | '/' | '"' | 'b' | 'f' | 'n' | 'r' | 't' => {
                        final_char = scanner.advance();
                    }
                    'u' => {
                        final_char = scanner.advance();
                        for _ in 0..4 {
                            if self.is_hex(scanner.peek()) {
                                final_char = scanner.advance();
                            } else {
                                return Err("Invalid escape character".to_string());
                            }
                        }
                    }
                    _ => {
                        return Err("Invalid escape character".to_string());
                    }
                }
            }

            final_char = scanner.advance();
        }

        if final_char != '"' {
            return Err("Unterminated string".to_string());
        }

        return Ok(TokenType::String);
    }

    fn match_number(&self, scanner: &mut Scanner) -> Result<TokenType, String> {
        while self.is_digit(scanner.peek()) {
            scanner.advance();
        }

        if scanner.peek() == '.' {
            scanner.advance();

            while self.is_digit(scanner.peek()) {
                scanner.advance();
            }
        }

        if scanner.peek() == 'e' || scanner.peek() == 'E' {
            scanner.advance();
            if scanner.peek() == '+' || scanner.peek() == '-' {
                scanner.advance();
            }
            while self.is_digit(scanner.peek()) {
                scanner.advance();
            }
        }

        return Ok(TokenType::Number);
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn is_hex(&self, c: char) -> bool {
        return self.is_digit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_next_token_brace() {
        // Arrange
        let mut scanner = Scanner::new("{");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::LeftBrace));
    }

    #[test]
    fn test_get_next_token_boolean() {
        // Arrange
        let mut scanner = Scanner::new("true");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Boolean));
    }

    #[test]
    fn test_get_next_token_null() {
        // Arrange
        let mut scanner = Scanner::new("null");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Null));
    }

    #[test]
    fn test_match_keyword() {
        // Arrange
        let mut scanner = Scanner::new("true");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Boolean));
    }

    #[test]
    fn test_match_string() {
        // Arrange
        let mut scanner = Scanner::new("\"test string\"");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::String));
    }

    #[test]
    fn test_match_string_fail() {
        // Arrange
        let mut scanner = Scanner::new("\"test string");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_match_number() {
        // Arrange
        let mut scanner = Scanner::new("123");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Number));
    }
}
