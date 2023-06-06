use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TokenType {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String,
    Number,
    Boolean,
    Null,
}
impl TokenType {
    pub fn is_literal(&self) -> bool {
        match self {
            TokenType::String => true,
            TokenType::Number => true,
            TokenType::Boolean => true,
            TokenType::Null => true,
            _ => false,
        }
    }
}
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LeftBrace => write!(f, "LeftBrace"),
            TokenType::RightBrace => write!(f, "RightBrace"),
            TokenType::LeftBracket => write!(f, "LeftBracket"),
            TokenType::RightBracket => write!(f, "RightBracket"),
            TokenType::Colon => write!(f, "Colon"),
            TokenType::Comma => write!(f, "Comma"),
            TokenType::String => write!(f, "String"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Boolean => write!(f, "Boolean"),
            TokenType::Null => write!(f, "Null"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub position_start: usize,
    pub position_end: usize,
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}, position {}..{}] {} \"{}\"",
            self.line, self.position_start, self.position_end, self.token_type, self.lexeme
        )
    }
}

#[derive(Debug, Clone)]
pub struct ScanError {
    pub line: usize,
    pub line_start: usize,
    pub lexeme: String,
    pub position_start: usize,
    pub position_end: usize,
    pub message: String,
}
impl fmt::Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}, position {}..{}] Error: {} \"{}\"",
            self.line,
            self.position_start - self.line_start,
            self.position_end - self.line_start,
            self.message,
            self.lexeme
        )
    }
}
