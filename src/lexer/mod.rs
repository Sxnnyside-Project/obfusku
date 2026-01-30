//! # Lexer Module
//!
//! Unicode-aware lexer for Obfusku that treats symbols as primary tokens.

pub mod symbols;

pub use symbols::{Lexer, LexerError, SourceLocation, Token, TokenKind};
