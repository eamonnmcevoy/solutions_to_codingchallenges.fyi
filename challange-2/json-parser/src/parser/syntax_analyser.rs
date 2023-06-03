use core::fmt;

use super::{types::{Token, TokenType}, token_scanner::TokenScanner};

#[derive(Copy, Clone, PartialEq, Eq)]
enum StateType {
    Object,
    ObjectProperty,
    ObjectPropertyEnd,
    Array,
    ArrayItem,
    ArrayItemEnd,
    Invalid,
    End
}
impl fmt::Display for StateType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StateType::Object => write!(f, "Object"),
            StateType::ObjectProperty => write!(f, "ObjectProperty"),
            StateType::ObjectPropertyEnd => write!(f, "ObjectPropertyEnd"),
            StateType::Array => write!(f, "Array"),
            StateType::ArrayItem => write!(f, "ArrayItem"),
            StateType::ArrayItemEnd => write!(f, "ArrayItemEnd"),
            StateType::Invalid => write!(f, "Invalid"),
            StateType::End => write!(f, "End"),
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub enum ErrorType {
    EmptyTokens,
    InvalidInitialState,
    UnexpectedToken,
    UnexpectedState,
    TrailingTokens,
}
impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::EmptyTokens => write!(f, "EmptyTokens"),
            ErrorType::InvalidInitialState => write!(f, "InvalidInitialState"),
            ErrorType::UnexpectedToken => write!(f, "UnexpectedToken"),
            ErrorType::UnexpectedState => write!(f, "UnexpectedState"),
            ErrorType::TrailingTokens => write!(f, "TrailingTokens"),
        }
    }
}

pub struct SyntaxAnalyzer {
    stack: Vec<StateType>,
}

impl SyntaxAnalyzer {
    pub fn new() -> SyntaxAnalyzer {
        SyntaxAnalyzer {
            stack: vec![],
        }
    }

    pub fn parse(& mut self, tokens: Vec<Token>) -> Result<(), ErrorType> {
        if tokens.len() == 0 {
            return Err(ErrorType::EmptyTokens);
        }

        let mut scanner = TokenScanner::new(&tokens);

        let first_token: Token = scanner.current().unwrap();
        let mut state = match first_token.token_type {
            TokenType::LeftBrace => StateType::Object,
            TokenType::LeftBracket => StateType::Array,
            _ => StateType::Invalid
        };

        if state == StateType::Invalid {
            return Err(ErrorType::InvalidInitialState);
        }

        while state != StateType::End {

            if state == StateType::Invalid {
                return Err(ErrorType::UnexpectedToken);
            }

            let next_state_result: Option<StateType> = match state {
                StateType::Object => SyntaxAnalyzer::parse_object(& mut scanner),
                StateType::ObjectProperty => SyntaxAnalyzer::parse_object_property(& mut scanner),
                StateType::ObjectPropertyEnd => SyntaxAnalyzer::parse_object_property_end(& mut scanner),
                StateType::Array => SyntaxAnalyzer::parse_array(& mut scanner),
                StateType::ArrayItem => SyntaxAnalyzer::parse_value(&mut scanner),
                StateType::ArrayItemEnd => SyntaxAnalyzer::parse_array_item_end(& mut scanner),
                _ => return Err(ErrorType::UnexpectedState)
            };

            if next_state_result.is_some() {
                let next_state: StateType = next_state_result.unwrap();
                if next_state == StateType::ObjectProperty {
                    self.stack.push(StateType::ObjectPropertyEnd);
                } 
                else if next_state == StateType::ArrayItem {
                    self.stack.push(StateType::ArrayItemEnd);
                } 
                else {
                    self.stack.push(state);
                }
                
                state = next_state;
            }
            else if self.stack.len() > 0 {
                scanner.advance();
                state = self.stack.pop().unwrap();
            }
            else {
                state = StateType::End;
            }
        }

        let current = scanner.current().unwrap();
        if !scanner.is_at_end() || (current.token_type != TokenType::RightBrace && current.token_type != TokenType::RightBracket) {
            print!("{} ", current);
            return Err(ErrorType::TrailingTokens);
        }

        Ok(())
    }

    fn parse_object(scanner: & mut TokenScanner) -> Option<StateType> {
        let current: Token = scanner.current().unwrap();
        if current.token_type != TokenType::LeftBrace {
            return Some(StateType::Invalid);
        }
        
        let next = scanner.advance().unwrap();
        match next.token_type {
            TokenType::RightBrace => return None,
            TokenType::String => return Some(StateType::ObjectProperty),
            _ => return Some(StateType::Invalid)
        }
    }

    fn parse_object_property(scanner: & mut TokenScanner) -> Option<StateType> {
        let mut current: Token = scanner.current().unwrap();
        if current.token_type != TokenType::String {
            return Some(StateType::Invalid);
        }

        current = scanner.advance().unwrap();
        if current.token_type != TokenType::Colon {
            return Some(StateType::Invalid);
        }

        scanner.advance().unwrap();
        SyntaxAnalyzer::parse_value(scanner)
    }

    fn parse_object_property_end(scanner: & mut TokenScanner) -> Option<StateType> {
        let current: Token = scanner.current().unwrap();

        if current.token_type == TokenType::Comma {
            scanner.advance();
            return Some(StateType::ObjectProperty);
        }
        else if current.token_type == TokenType::RightBrace {
            return None;
        }
        
        return Some(StateType::Invalid);
    }

    fn parse_array(scanner: & mut TokenScanner) -> Option<StateType> {
        let current: Token = scanner.current().unwrap();
        if current.token_type != TokenType::LeftBracket {
            return Some(StateType::Invalid);
        }

        let next = scanner.advance().unwrap();
        match next.token_type {
            TokenType::RightBracket => return None,
            TokenType::Comma => return Some(StateType::Invalid),
            _ => return Some(StateType::ArrayItem),
        }        
    }

    fn parse_array_item_end(scanner: & mut TokenScanner) -> Option<StateType> {
        let current: Token = scanner.current().unwrap();

        if current.token_type == TokenType::Comma {
            scanner.advance();
            return Some(StateType::ArrayItem);
        }
        else if current.token_type == TokenType::RightBracket {
            return None;
        }
        
        return Some(StateType::Invalid);
    }

    fn parse_value(scanner: & mut TokenScanner) -> Option<StateType> {
        let current: Token = scanner.current().unwrap();
        if current.token_type == TokenType::LeftBrace {
            return SyntaxAnalyzer::parse_object(scanner);
        }

        if current.token_type == TokenType::LeftBracket {
            return SyntaxAnalyzer::parse_array(scanner);
        }

        if current.token_type.is_literal() {
            return None;
        }

        Some(StateType::Invalid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_should_return_err_if_tokens_empty() {
        //Arrange
        let input: Vec<Token> = vec![];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert_eq!(result, Err(ErrorType::EmptyTokens));
    }

    #[test]
    fn test_parse_should_return_err_if_first_token_is_not_valid() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::Boolean, lexeme: "true".to_string(), line: 1, position_start: 0, position_end: 0 }
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert_eq!(result, Err(ErrorType::InvalidInitialState));
    }

    #[test]
    fn test_parse_should_return_true_for_empty_object() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 1 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 1, position_end: 2 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_object_with_single_value() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 1 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 1, position_end: 7 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 7, position_end: 8 },
            Token { token_type: TokenType::String, lexeme: "value".to_string(), line: 0, position_start: 8, position_end: 15 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 15, position_end: 16 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_object_with_multiple_values() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 1 },
            Token { token_type: TokenType::String, lexeme: "\"test\"".to_string(), line: 0, position_start: 1, position_end: 7 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 7, position_end: 8 },
            Token { token_type: TokenType::String, lexeme: "\"value\"".to_string(), line: 0, position_start: 8, position_end: 15 },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), line: 0, position_start: 15, position_end: 16 },
            Token { token_type: TokenType::String, lexeme: "\"test2\"".to_string(), line: 0, position_start: 16, position_end: 23 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 23, position_end: 24 },
            Token { token_type: TokenType::String, lexeme: "1".to_string(), line: 0, position_start: 25, position_end: 26 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 26, position_end: 27 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input.into());

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_nested_object() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "the key".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "value".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "test2".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "value2".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 0, position_end: 0 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_error_for_unclosed_object() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 1 },
            Token { token_type: TokenType::String, lexeme: "\"test\"".to_string(), line: 0, position_start: 1, position_end: 7 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 7, position_end: 8 },
            Token { token_type: TokenType::String, lexeme: "\"value\"".to_string(), line: 0, position_start: 8, position_end: 15 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input.into());

        //Assert
        assert_eq!(result, Err(ErrorType::UnexpectedToken));
    }

    #[test]
    fn test_parse_should_return_true_for_empty_array() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 1 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 1, position_end: 2 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_array_with_single_value() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 1 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 1, position_end: 7 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 7, position_end: 8 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_nested_array() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_array_with_object() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_for_mixed_array() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBrace, lexeme: "{".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Colon, lexeme: ":".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Number, lexeme: "123".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBrace, lexeme: "}".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_true_2d_array() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_should_return_err_for_array_with_trailing_comma() {
        //Arrange
        let input: Vec<Token> = vec![
            Token { token_type: TokenType::LeftBracket, lexeme: "[".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::String, lexeme: "test".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::RightBracket, lexeme: "]".to_string(), line: 0, position_start: 0, position_end: 0 },
            Token { token_type: TokenType::Comma, lexeme: ",".to_string(), line: 0, position_start: 0, position_end: 0 },
        ];
        let mut syntax_analyser = SyntaxAnalyzer::new();

        // Act
        let result = syntax_analyser.parse(input);

        //Assert
        assert!(result.is_err());
    }

}