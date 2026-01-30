//! # Source Maps for Obfusku
//!
//! Maps character indices to line/column positions for better error reporting.

use std::fmt;

/// A position in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourcePos {
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// Byte offset from start
    pub offset: usize,
}

impl SourcePos {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
}

impl fmt::Display for SourcePos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A span in source code (start to end)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SourceSpan {
    pub start: SourcePos,
    pub end: SourcePos,
}

impl SourceSpan {
    pub fn new(start: SourcePos, end: SourcePos) -> Self {
        Self { start, end }
    }
    
    pub fn from_pos(pos: SourcePos) -> Self {
        Self { start: pos, end: pos }
    }
}

/// Source map that tracks line starts for fast lookups
#[derive(Debug, Clone)]
pub struct SourceMap {
    /// The original source code
    source: String,
    /// Byte offsets of line starts
    line_starts: Vec<usize>,
}

impl SourceMap {
    /// Create a new source map from source code
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0];
        
        for (i, c) in source.char_indices() {
            if c == '\n' {
                line_starts.push(i + 1);
            }
        }
        
        Self {
            source: source.to_string(),
            line_starts,
        }
    }
    
    /// Convert a byte offset to line/column
    pub fn offset_to_pos(&self, offset: usize) -> SourcePos {
        // Binary search for the line
        let line = match self.line_starts.binary_search(&offset) {
            Ok(exact) => exact,
            Err(after) => after.saturating_sub(1),
        };
        
        let line_start = self.line_starts.get(line).copied().unwrap_or(0);
        let column = offset - line_start + 1;
        
        SourcePos::new(line + 1, column, offset)
    }
    
    /// Get the source line at the given line number (1-based)
    pub fn get_line(&self, line: usize) -> Option<&str> {
        if line == 0 || line > self.line_starts.len() {
            return None;
        }
        
        let start = self.line_starts[line - 1];
        let end = self.line_starts.get(line).copied().unwrap_or(self.source.len());
        
        // Trim trailing newline
        let line_text = &self.source[start..end];
        Some(line_text.trim_end_matches('\n').trim_end_matches('\r'))
    }
    
    /// Get total number of lines
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
    
    /// Format an error with source context
    pub fn format_error(&self, pos: SourcePos, message: &str) -> String {
        let mut result = format!("🔮 Error at line {}, column {}:\n", pos.line, pos.column);
        result.push_str(&format!("   {}\n", message));
        
        if let Some(line) = self.get_line(pos.line) {
            result.push_str(&format!("   │ {}\n", line));
            result.push_str(&format!("   │ {}^\n", " ".repeat(pos.column.saturating_sub(1))));
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_source_map() {
        let source = "⟁x=5\n⚡[x]\n❧";
        let map = SourceMap::new(source);
        
        assert_eq!(map.line_count(), 3);
        
        let pos = map.offset_to_pos(0);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);
        
        // After first newline (position of ⚡)
        let pos = map.offset_to_pos(8);
        assert_eq!(pos.line, 2);
    }
    
    #[test]
    fn test_get_line() {
        let source = "line one\nline two\nline three";
        let map = SourceMap::new(source);
        
        assert_eq!(map.get_line(1), Some("line one"));
        assert_eq!(map.get_line(2), Some("line two"));
        assert_eq!(map.get_line(3), Some("line three"));
        assert_eq!(map.get_line(4), None);
    }
}
