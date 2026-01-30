//! # Obfusku Lexer
//!
//! A Unicode-first lexer that treats symbols as primary tokens.
//! This lexer is designed to be efficient and extensible, supporting
//! the full range of Unicode characters used in Obfusku.
//!
//! ## Design Philosophy
//!
//! Unlike traditional lexers that focus on keywords, this lexer:
//! - Prioritizes symbol recognition over alphabetic tokens
//! - Uses the symbol table for semantic classification
//! - Handles multi-character symbols efficiently
//! - Preserves source location for error reporting

use crate::symbols::{SymbolMeaning, SymbolTable};
use std::str::Chars;
use std::iter::Peekable;
use thiserror::Error;

/// Errors that can occur during lexing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum LexerError {
    #[error("Unknown symbol '{glyph}' at line {line}, column {column}")]
    UnknownSymbol {
        glyph: String,
        line: usize,
        column: usize,
    },

    #[error("Unterminated string starting at line {line}, column {column}")]
    UnterminatedString {
        line: usize,
        column: usize,
    },

    #[error("Invalid escape sequence '\\{char}' at line {line}, column {column}")]
    InvalidEscape {
        char: char,
        line: usize,
        column: usize,
    },

    #[error("Invalid number format at line {line}, column {column}")]
    InvalidNumber {
        line: usize,
        column: usize,
    },
}

/// Source location for error reporting and debugging
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl SourceLocation {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self { line: 1, column: 1, offset: 0 }
    }
}

/// Token types in Obfusku
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    /// A recognized symbol with its semantic meaning
    Symbol(SymbolMeaning),

    /// An identifier (variable name, function name)
    Identifier(String),

    /// Integer literal
    Integer(i64),

    /// Floating-point literal
    Float(f64),

    /// String literal (contents, without quotes)
    String(String),

    /// Boolean literal
    Boolean(bool),

    /// Assignment operator (=)
    Equals,

    /// End of file
    Eof,
}

/// A token with its kind and source location
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub location: SourceLocation,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, location: SourceLocation) -> Self {
        Self { kind, lexeme, location }
    }

    pub fn is_symbol(&self, meaning: SymbolMeaning) -> bool {
        matches!(&self.kind, TokenKind::Symbol(m) if *m == meaning)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }
}

/// The Obfusku lexer
///
/// Converts source code into a stream of tokens, prioritizing
/// symbol recognition for the esoteric, symbol-first design.
pub struct Lexer<'a> {
    /// Original source (kept for error context and debugging)
    #[allow(dead_code)]
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    symbol_table: &'a SymbolTable,

    // Current position tracking
    current_offset: usize,
    line: usize,
    column: usize,

    // For multi-character symbol lookahead
    lookahead_buffer: String,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer for the given source code
    pub fn new(source: &'a str, symbol_table: &'a SymbolTable) -> Self {
        Self {
            source,
            chars: source.chars().peekable(),
            symbol_table,
            current_offset: 0,
            line: 1,
            column: 1,
            lookahead_buffer: String::with_capacity(8),
        }
    }

    /// Tokenize the entire source and return all tokens
    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            let is_eof = token.is_eof();
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        Ok(tokens)
    }

    /// Get the next token from the source
    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace_and_comments();

        let start_location = self.current_location();

        // Check for EOF
        let Some(c) = self.peek_char() else {
            return Ok(Token::new(TokenKind::Eof, String::new(), start_location));
        };

        // Try to match a symbol first (symbol-first design)
        if let Some(token) = self.try_match_symbol()? {
            return Ok(token);
        }

        // String literal
        if c == '"' {
            return self.scan_string();
        }

        // Number literal
        if c.is_ascii_digit() || (c == '-' && self.peek_second_char().map_or(false, |c| c.is_ascii_digit())) {
            return self.scan_number();
        }

        // Identifier or keyword
        if c.is_alphabetic() || c == '_' {
            return self.scan_identifier();
        }

        // Simple equals sign
        if c == '=' {
            self.advance_char();
            return Ok(Token::new(TokenKind::Equals, "=".to_string(), start_location));
        }

        // Unknown character - try to give a helpful error
        let glyph = self.advance_char().unwrap().to_string();
        Err(LexerError::UnknownSymbol {
            glyph,
            line: start_location.line,
            column: start_location.column,
        })
    }

    /// Try to match a symbol from the symbol table
    ///
    /// Uses greedy matching - tries longest possible match first
    fn try_match_symbol(&mut self) -> Result<Option<Token>, LexerError> {
        let start_location = self.current_location();
        let max_len = self.symbol_table.max_glyph_length();

        // Build lookahead buffer
        self.lookahead_buffer.clear();
        let mut temp_chars = self.chars.clone();
        for _ in 0..max_len {
            if let Some(c) = temp_chars.next() {
                self.lookahead_buffer.push(c);
            } else {
                break;
            }
        }

        // Try longest match first (greedy)
        for len in (1..=self.lookahead_buffer.chars().count()).rev() {
            let candidate: String = self.lookahead_buffer.chars().take(len).collect();

            if let Some(symbol) = self.symbol_table.lookup(&candidate) {
                // Found a match! Consume the characters
                for _ in 0..len {
                    self.advance_char();
                }

                return Ok(Some(Token::new(
                    TokenKind::Symbol(symbol.meaning),
                    candidate,
                    start_location,
                )));
            }
        }

        Ok(None)
    }

    /// Scan a string literal
    fn scan_string(&mut self) -> Result<Token, LexerError> {
        let start_location = self.current_location();

        // Consume opening quote
        self.advance_char();

        let mut value = String::new();

        loop {
            match self.peek_char() {
                None => {
                    return Err(LexerError::UnterminatedString {
                        line: start_location.line,
                        column: start_location.column,
                    });
                }
                Some('"') => {
                    self.advance_char();
                    break;
                }
                Some('\\') => {
                    self.advance_char();
                    let escaped = self.scan_escape_sequence()?;
                    value.push(escaped);
                }
                Some(c) => {
                    value.push(c);
                    self.advance_char();
                }
            }
        }

        let lexeme = format!("\"{}\"", value);
        Ok(Token::new(TokenKind::String(value), lexeme, start_location))
    }

    /// Scan an escape sequence after backslash
    fn scan_escape_sequence(&mut self) -> Result<char, LexerError> {
        let location = self.current_location();

        match self.advance_char() {
            Some('n') => Ok('\n'),
            Some('t') => Ok('\t'),
            Some('r') => Ok('\r'),
            Some('\\') => Ok('\\'),
            Some('"') => Ok('"'),
            Some('0') => Ok('\0'),
            Some(c) => Err(LexerError::InvalidEscape {
                char: c,
                line: location.line,
                column: location.column,
            }),
            None => Err(LexerError::UnterminatedString {
                line: location.line,
                column: location.column,
            }),
        }
    }

    /// Scan a number (integer or float)
    fn scan_number(&mut self) -> Result<Token, LexerError> {
        let start_location = self.current_location();
        let mut lexeme = String::new();
        let mut is_float = false;

        // Handle negative sign
        if self.peek_char() == Some('-') {
            lexeme.push('-');
            self.advance_char();
        }

        // Scan integer part
        while let Some(c) = self.peek_char() {
            if c.is_ascii_digit() {
                lexeme.push(c);
                self.advance_char();
            } else {
                break;
            }
        }

        // Check for decimal point
        if self.peek_char() == Some('.') {
            if let Some(next) = self.peek_second_char() {
                if next.is_ascii_digit() {
                    is_float = true;
                    lexeme.push('.');
                    self.advance_char();

                    // Scan fractional part
                    while let Some(c) = self.peek_char() {
                        if c.is_ascii_digit() {
                            lexeme.push(c);
                            self.advance_char();
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        // Check for exponent
        if let Some(c) = self.peek_char() {
            if c == 'e' || c == 'E' {
                is_float = true;
                lexeme.push(c);
                self.advance_char();

                // Optional sign
                if let Some(sign) = self.peek_char() {
                    if sign == '+' || sign == '-' {
                        lexeme.push(sign);
                        self.advance_char();
                    }
                }

                // Exponent digits
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() {
                        lexeme.push(c);
                        self.advance_char();
                    } else {
                        break;
                    }
                }
            }
        }

        // Parse the number
        if is_float {
            match lexeme.parse::<f64>() {
                Ok(value) => Ok(Token::new(TokenKind::Float(value), lexeme, start_location)),
                Err(_) => Err(LexerError::InvalidNumber {
                    line: start_location.line,
                    column: start_location.column,
                }),
            }
        } else {
            match lexeme.parse::<i64>() {
                Ok(value) => Ok(Token::new(TokenKind::Integer(value), lexeme, start_location)),
                Err(_) => Err(LexerError::InvalidNumber {
                    line: start_location.line,
                    column: start_location.column,
                }),
            }
        }
    }

    /// Scan an identifier or keyword
    fn scan_identifier(&mut self) -> Result<Token, LexerError> {
        let start_location = self.current_location();
        let mut lexeme = String::new();

        while let Some(c) = self.peek_char() {
            if c.is_alphanumeric() || c == '_' {
                lexeme.push(c);
                self.advance_char();
            } else {
                break;
            }
        }

        // Check for boolean keywords
        let kind = match lexeme.as_str() {
            "true" => TokenKind::Boolean(true),
            "false" => TokenKind::Boolean(false),
            _ => TokenKind::Identifier(lexeme.clone()),
        };

        Ok(Token::new(kind, lexeme, start_location))
    }

    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            // Skip whitespace
            while let Some(c) = self.peek_char() {
                if c.is_whitespace() {
                    self.advance_char();
                } else {
                    break;
                }
            }

            // Check for line comment (//)
            if self.peek_char() == Some('/') {
                let mut temp = self.chars.clone();
                temp.next();
                if temp.peek() == Some(&'/') {
                    // Skip until end of line
                    while let Some(c) = self.advance_char() {
                        if c == '\n' {
                            break;
                        }
                    }
                    continue;
                }
            }

            // Check for block comment (⌈ ... ⌉)
            if self.peek_char() == Some('⌈') {
                self.advance_char();
                let mut depth = 1;
                while depth > 0 {
                    match self.advance_char() {
                        Some('⌈') => depth += 1,
                        Some('⌉') => depth -= 1,
                        None => break,
                        _ => {}
                    }
                }
                continue;
            }

            break;
        }
    }

    /// Peek at the current character without consuming it
    fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    /// Peek at the second character without consuming
    fn peek_second_char(&self) -> Option<char> {
        let mut temp = self.chars.clone();
        temp.next();
        temp.next()
    }

    /// Advance to the next character
    fn advance_char(&mut self) -> Option<char> {
        let c = self.chars.next()?;

        self.current_offset += c.len_utf8();

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(c)
    }

    /// Get the current source location
    fn current_location(&self) -> SourceLocation {
        SourceLocation::new(self.line, self.column, self.current_offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex(source: &str) -> Result<Vec<Token>, LexerError> {
        let table = SymbolTable::new();
        let mut lexer = Lexer::new(source, &table);
        lexer.tokenize()
    }

    #[test]
    fn test_integer_type_declaration() {
        let tokens = lex("⟁x=5").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Symbol(SymbolMeaning::TypeInteger)));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "x"));
        assert!(matches!(tokens[2].kind, TokenKind::Equals));
        assert!(matches!(tokens[3].kind, TokenKind::Integer(5)));
    }

    #[test]
    fn test_string_literal() {
        let tokens = lex("\"Hello, World!\"").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::String(ref s) if s == "Hello, World!"));
    }

    #[test]
    fn test_comment_skip() {
        let tokens = lex("// this is a comment\n⟁x").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Symbol(SymbolMeaning::TypeInteger)));
    }

    #[test]
    fn test_float_number() {
        let tokens = lex("3.14159").unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Float(f) if (f - 3.14159).abs() < 0.00001));
    }
}
