use super::source_scanner::SourceScanner;
use super::types::{ScanError, Token, TokenType};

pub struct Lexer {}
impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn get_tokens(&self, source: &str) -> Result<Vec<Token>, ScanError> {
        let mut scanner = SourceScanner::new(source);
        scanner.skip_whitespace();

        while !scanner.is_at_end() {
            let result = self.get_next_token(&mut scanner);
            match result {
                Ok(token_type) => {
                    scanner.add_token(token_type);
                }
                Err(error) => {
                    let scan_error = scanner.report_error(error);
                    return Err(scan_error);
                }
            }
            scanner.skip_whitespace();
        }

        Ok(scanner.tokens)
    }

    fn get_next_token(&self, scanner: &mut SourceScanner) -> Result<TokenType, String> {
        match scanner.peek() {
            '{' => self.match_symbol(scanner, '{', TokenType::LeftBrace),
            '}' => self.match_symbol(scanner, '}', TokenType::RightBrace),
            '[' => self.match_symbol(scanner, '[', TokenType::LeftBracket),
            ']' => self.match_symbol(scanner, ']', TokenType::RightBracket),
            ':' => self.match_symbol(scanner, ':', TokenType::Colon),
            ',' => self.match_symbol(scanner, ',', TokenType::Comma),
            't' => self.match_keyword(scanner, "true", TokenType::Boolean),
            'f' => self.match_keyword(scanner, "false", TokenType::Boolean),
            'n' => self.match_keyword(scanner, "null", TokenType::Null),
            '"' => self.match_string(scanner),
            '0'..='9' | '-' => self.match_number(scanner),
            _ => {
                scanner.advance();
                Err("Unexpected character".to_string())
            }
        }
    }

    fn match_symbol(
        &self,
        scanner: &mut SourceScanner,
        symbol: char,
        token_type: TokenType,
    ) -> Result<TokenType, String> {
        if scanner.advance() == symbol {
            Ok(token_type)
        } else {
            Err(format!("Expected '{}' after '{}'", symbol, symbol))
        }
    }

    fn match_keyword(
        &self,
        scanner: &mut SourceScanner,
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

    fn match_string(&self, scanner: &mut SourceScanner) -> Result<TokenType, String> {
        if scanner.advance() != '"' {
            return Err("Expected '\"' at start of string".to_string());
        }

        let mut final_char = scanner.peek();
        loop {
            if scanner.is_at_end() {
                break;
            }

            if scanner.peek() == '\n' {
                return Err("Unescaped newline in string".to_string());
            }

            if scanner.peek() == '\t' {
                return Err("Unescaped tab in string".to_string());
            }

            if scanner.peek() == '"' {
                final_char = scanner.advance();
                break;
            } else if scanner.peek() == '\\' {
                scanner.advance();
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
            } else {
                final_char = scanner.advance();
            }
        }

        if final_char != '"' {
            return Err("Unterminated string".to_string());
        }

        return Ok(TokenType::String);
    }

    fn match_number(&self, scanner: &mut SourceScanner) -> Result<TokenType, String> {
        if scanner.peek() == '-' {
            scanner.advance();
        }
        
        if scanner.peek() == '0' {
            scanner.advance();
            if self.is_digit(scanner.peek()) {
                return Err("Invalid number - no leading zeros".to_string());
            }
        }
        else {
            while self.is_digit(scanner.peek()) {
                scanner.advance();
            }
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
            let next = scanner.peek();
            if !self.is_digit(next) {
                return Err("Expected digit after exponent".to_string());
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
    fn get_tokens_quote() {
        // Arrange
        let source = "[\"\\\"\"]";
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_tokens(source);

        // Assert
        assert!(!result.is_err());
        let tokens = result.unwrap();
        assert!(tokens.len() == 3);
        assert!(tokens[0].token_type == TokenType::LeftBracket);
        assert!(tokens[1].token_type == TokenType::String);
        assert!(tokens[2].token_type == TokenType::RightBracket);
    }

    #[test]
    fn test_get_next_token_brace() {
        // Arrange
        let mut scanner = SourceScanner::new("{");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::LeftBrace));
    }

    #[test]
    fn test_get_next_token_boolean() {
        // Arrange
        let mut scanner = SourceScanner::new("true");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Boolean));
    }

    #[test]
    fn test_get_next_token_null() {
        // Arrange
        let mut scanner = SourceScanner::new("null");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Null));
    }

    #[test]
    fn test_match_keyword() {
        // Arrange
        let mut scanner = SourceScanner::new("true");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Boolean));
    }

    #[test]
    fn test_match_string() {
        // Arrange
        let mut scanner = SourceScanner::new("\"test string\"");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::String));
    }

    #[test]
    fn test_match_string_with_quote() {
        // Arrange
        let mut scanner = SourceScanner::new("\"\\\"\"");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::String));
    }

    #[test]
    fn test_match_string_fail() {
        // Arrange
        let mut scanner = SourceScanner::new("\"test string");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_match_number() {
        // Arrange
        let mut scanner = SourceScanner::new("123");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result == Ok(TokenType::Number));
    }

    #[test]
    fn test_err_unquoted_str() {
        // Arrange
        let mut scanner = SourceScanner::new("unquoted string");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_err_number_no_digit_after_exponent() {
        // Arrange
        let mut scanner = SourceScanner::new("0e");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_err_number_no_leading_zeros() {
        // Arrange
        let mut scanner = SourceScanner::new("0123");
        let lexer = Lexer::new();

        // Act
        let result = lexer.get_next_token(&mut scanner);

        // Assert
        assert!(result.is_err());
    }
}
