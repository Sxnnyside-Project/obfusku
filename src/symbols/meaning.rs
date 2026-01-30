//! # Symbol System for Obfusku
//!
//! This module defines the semantic meaning of all symbols in Obfusku.
//! Symbols are the primary semantic units - they carry meaning beyond mere tokens.
//!
//! ## Design Philosophy
//!
//! In Obfusku, symbols are not just syntax - they are semantic entities with:
//! - **Glyph**: The Unicode character(s) representing the symbol
//! - **Category**: What kind of operation or type the symbol represents
//! - **Semantic Weight**: How the symbol affects execution context
//!
//! The symbol system is designed for extensibility - new symbols can be added
//! without modifying the core VM.

use rustc_hash::FxHashMap;
use std::fmt;

/// Categories of symbols in Obfusku
///
/// Each symbol belongs to exactly one category, which determines
/// how the parser and VM interpret it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolCategory {
    /// Type declarations (⟁ for int, ⌘ for string, etc.)
    TypeDeclaration,
    /// Operators that perform actions (✚, ☠︎, etc.)
    Operator,
    /// Control flow symbols (⊂, ⊃, ❧, etc.)
    ControlFlow,
    /// I/O operations (⚓, ⚡, ✤)
    InputOutput,
    /// Special values (∅ for null)
    SpecialValue,
    /// Modifiers that change behavior of following symbols
    Modifier,
    /// Delimiters and structural symbols
    Delimiter,
    /// Comparison operators
    Comparison,
    /// Logical operators
    Logical,
}

/// Semantic meaning of a symbol
///
/// This enum captures what each symbol *means* in the language,
/// not just what it looks like.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SymbolMeaning {
    // ═══════════════════════════════════════════════════════════════
    // TYPE DECLARATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Integer type (⟁) - whole numbers
    TypeInteger,
    /// Real/Float type (⧆) - decimal numbers
    TypeReal,
    /// String type (⌘) - text sequences
    TypeString,
    /// Boolean type (☍) - true/false values
    TypeBoolean,
    /// Rune type (ᚱ) - single Unicode character
    TypeRune,
    /// Array type (⌬) - ordered collection
    TypeArray,
    /// Map type (⌖) - key-value collection
    TypeMap,

    // ═══════════════════════════════════════════════════════════════
    // SPECIAL VALUES
    // ═══════════════════════════════════════════════════════════════

    /// Null/Void value (∅)
    Null,
    /// True boolean (◉)
    True,
    /// False boolean (◎)
    False,

    // ═══════════════════════════════════════════════════════════════
    // ARITHMETIC OPERATORS
    // ═══════════════════════════════════════════════════════════════

    /// Addition (✚)
    Add,
    /// Subtraction (☠︎)
    Subtract,
    /// Multiplication (✱)
    Multiply,
    /// Division (÷)
    Divide,
    /// Modulo (⌘)
    Modulo,
    /// Power/Exponent (⬆)
    Power,
    /// Negate (−)
    Negate,

    // ═══════════════════════════════════════════════════════════════
    // COMPARISON OPERATORS
    // ═══════════════════════════════════════════════════════════════

    /// Equal (⩵)
    Equal,
    /// Not Equal (≠)
    NotEqual,
    /// Less Than (◁)
    LessThan,
    /// Greater Than (▷)
    GreaterThan,
    /// Less Than or Equal (◁⩵)
    LessOrEqual,
    /// Greater Than or Equal (▷⩵)
    GreaterOrEqual,

    // ═══════════════════════════════════════════════════════════════
    // LOGICAL OPERATORS
    // ═══════════════════════════════════════════════════════════════

    /// Logical AND (∧)
    And,
    /// Logical OR (∨)
    Or,
    /// Logical NOT (¬)
    Not,
    /// Logical XOR (⊕)
    Xor,

    // ═══════════════════════════════════════════════════════════════
    // ASSIGNMENT & EVALUATION
    // ═══════════════════════════════════════════════════════════════

    /// Assignment operator (⚙︎)
    Assign,
    /// Arrow for assignment target (→)
    Arrow,
    /// Binding/Let (≔)
    Bind,

    // ═══════════════════════════════════════════════════════════════
    // INPUT/OUTPUT
    // ═══════════════════════════════════════════════════════════════

    /// Input from user (⚓)
    Input,
    /// Output variable value (⚡)
    Output,
    /// Print literal text (✤)
    Print,
    /// Debug output (⌥)
    Debug,

    // ═══════════════════════════════════════════════════════════════
    // CONTROL FLOW
    // ═══════════════════════════════════════════════════════════════

    /// Start of cycle/loop (⊂)
    LoopStart,
    /// End of cycle/loop (⊃)
    LoopEnd,
    /// Conditional branch start (⟨)
    IfStart,
    /// Else branch (⟩)
    Else,
    /// End of conditional (⟫)
    IfEnd,
    /// Break from loop (⊗)
    Break,
    /// Continue to next iteration (⊕)
    Continue,
    /// End program (❧)
    EndProgram,
    /// Function definition start (λ)
    FunctionStart,
    /// Function definition end (Λ)
    FunctionEnd,
    /// Return from function (⤶)
    Return,
    /// Call function (⤷)
    Call,

    // ═══════════════════════════════════════════════════════════════
    // PATTERN MATCHING (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Match expression start (⟡)
    MatchStart,
    /// Match arm (⟢)
    MatchArm,
    /// Match expression end (⟣)
    MatchEnd,
    /// Wildcard pattern (◇)
    Wildcard,

    // ═══════════════════════════════════════════════════════════════
    // MODULE SYSTEM (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Import module (⟲)
    Import,
    /// Export symbol (⟳)
    Export,
    /// Module access (⊷)
    ModuleAccess,

    // ═══════════════════════════════════════════════════════════════
    // EXCEPTION HANDLING (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Try block start (☄)
    TryStart,
    /// Catch block (☊)
    CatchBlock,
    /// Finally block (☋)
    FinallyBlock,
    /// Throw exception (⚠)
    Throw,

    // ═══════════════════════════════════════════════════════════════
    // MAP OPERATIONS (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Map key-value separator (⇒)
    MapArrow,
    /// Map entry separator (⋄)
    MapSeparator,

    // ═══════════════════════════════════════════════════════════════
    // STACK OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Push to stack (⇑)
    Push,
    /// Pop from stack (⇓)
    Pop,
    /// Duplicate top of stack (⇕)
    Dup,
    /// Swap top two stack elements (⇆)
    Swap,
    /// Rotate stack (↻)
    Rotate,

    // ═══════════════════════════════════════════════════════════════
    // SPECIAL OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Accumulator (✹) - special counter variable
    Accumulator,
    /// Increment (⊕)
    Increment,
    /// Decrement (⊖)
    Decrement,
    /// Optional modifier (∅ suffix)
    Optional,
    /// Reference/Address (⌘)
    Reference,
    /// Dereference (⌦)
    Dereference,

    // ═══════════════════════════════════════════════════════════════
    // DELIMITERS
    // ═══════════════════════════════════════════════════════════════

    /// Left bracket [
    LeftBracket,
    /// Right bracket ]
    RightBracket,
    /// Left paren (
    LeftParen,
    /// Right paren )
    RightParen,
    /// Left brace { (for maps)
    LeftBrace,
    /// Right brace } (for maps)
    RightBrace,
    /// Separator (,)
    Separator,
    /// Statement terminator (⁂)
    Terminator,

    // ═══════════════════════════════════════════════════════════════
    // COMMENTS & META
    // ═══════════════════════════════════════════════════════════════

    /// Comment marker (//)
    Comment,
    /// Block comment start (⌈)
    BlockCommentStart,
    /// Block comment end (⌉)
    BlockCommentEnd,
}

/// A symbol definition containing its glyph and meaning
#[derive(Debug, Clone)]
pub struct Symbol {
    /// The Unicode glyph(s) representing this symbol
    pub glyph: &'static str,
    /// The semantic meaning
    pub meaning: SymbolMeaning,
    /// The category this symbol belongs to
    pub category: SymbolCategory,
    /// Human-readable description
    pub description: &'static str,
}

impl Symbol {
    pub const fn new(
        glyph: &'static str,
        meaning: SymbolMeaning,
        category: SymbolCategory,
        description: &'static str,
    ) -> Self {
        Self {
            glyph,
            meaning,
            category,
            description,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.glyph)
    }
}

/// The master symbol table - maps glyphs to their semantic meanings
///
/// This is the canonical source of truth for all symbols in Obfusku.
/// The table is built at startup and used by the lexer for fast lookups.
pub struct SymbolTable {
    /// Maps glyph strings to their symbol definitions
    symbols: FxHashMap<&'static str, Symbol>,
    /// Maximum glyph length (for multi-char symbols)
    max_glyph_len: usize,
}

impl SymbolTable {
    /// Create a new symbol table with all standard Obfusku symbols
    pub fn new() -> Self {
        let mut table = Self {
            symbols: FxHashMap::default(),
            max_glyph_len: 1,
        };

        // Register all standard symbols
        table.register_standard_symbols();

        table
    }

    /// Register a single symbol
    fn register(&mut self, symbol: Symbol) {
        let glyph_len = symbol.glyph.chars().count();
        if glyph_len > self.max_glyph_len {
            self.max_glyph_len = glyph_len;
        }
        self.symbols.insert(symbol.glyph, symbol);
    }

    /// Look up a symbol by its glyph
    pub fn lookup(&self, glyph: &str) -> Option<&Symbol> {
        self.symbols.get(glyph)
    }

    /// Get the maximum glyph length (for lexer lookahead)
    pub fn max_glyph_length(&self) -> usize {
        self.max_glyph_len
    }

    /// Check if a glyph exists in the table
    pub fn contains(&self, glyph: &str) -> bool {
        self.symbols.contains_key(glyph)
    }

    /// Get all symbols of a specific category
    pub fn symbols_in_category(&self, category: SymbolCategory) -> Vec<&Symbol> {
        self.symbols
            .values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Register all standard Obfusku symbols
    fn register_standard_symbols(&mut self) {
        use SymbolCategory::*;
        use SymbolMeaning::*;

        // ═══════════════════════════════════════════════════════════════
        // TYPE DECLARATIONS - Symbols that declare variable types
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⟁", TypeInteger, TypeDeclaration,
            "Integer type - declares whole number variables"));
        self.register(Symbol::new("⧆", TypeReal, TypeDeclaration,
            "Real type - declares floating-point variables"));
        self.register(Symbol::new("⌘", TypeString, TypeDeclaration,
            "String type - declares text variables"));
        self.register(Symbol::new("☍", TypeBoolean, TypeDeclaration,
            "Boolean type - declares true/false variables"));
        self.register(Symbol::new("ᚱ", TypeRune, TypeDeclaration,
            "Rune type - declares single character variables"));
        self.register(Symbol::new("⌬", TypeArray, TypeDeclaration,
            "Array type - declares ordered collection"));
        self.register(Symbol::new("⌖", TypeMap, TypeDeclaration,
            "Map type - declares key-value collection"));

        // ═══════════════════════════════════════════════════════════════
        // SPECIAL VALUES
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("∅", Null, SpecialValue,
            "Null/void - represents absence of value"));
        self.register(Symbol::new("◉", True, SpecialValue,
            "Boolean true value"));
        self.register(Symbol::new("◎", False, SpecialValue,
            "Boolean false value"));

        // ═══════════════════════════════════════════════════════════════
        // ARITHMETIC OPERATORS
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("✚", Add, Operator,
            "Addition - adds two values"));
        self.register(Symbol::new("☠︎", Subtract, Operator,
            "Subtraction - subtracts second from first"));
        self.register(Symbol::new("✱", Multiply, Operator,
            "Multiplication - multiplies two values"));
        self.register(Symbol::new("÷", Divide, Operator,
            "Division - divides first by second"));
        self.register(Symbol::new("⌗", Modulo, Operator,
            "Modulo - remainder after division"));
        self.register(Symbol::new("⬆", Power, Operator,
            "Power - raises to exponent"));

        // ═══════════════════════════════════════════════════════════════
        // COMPARISON OPERATORS
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⩵", Equal, Comparison,
            "Equal - tests if two values are equal"));
        self.register(Symbol::new("≠", NotEqual, Comparison,
            "Not equal - tests if values differ"));
        self.register(Symbol::new("◁", LessThan, Comparison,
            "Less than - tests if first is smaller"));
        self.register(Symbol::new("▷", GreaterThan, Comparison,
            "Greater than - tests if first is larger"));
        self.register(Symbol::new("⩽", LessOrEqual, Comparison,
            "Less or equal - tests if first is smaller or equal"));
        self.register(Symbol::new("⩾", GreaterOrEqual, Comparison,
            "Greater or equal - tests if first is larger or equal"));

        // ═══════════════════════════════════════════════════════════════
        // LOGICAL OPERATORS
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("∧", And, Logical,
            "Logical AND - true if both operands true"));
        self.register(Symbol::new("∨", Or, Logical,
            "Logical OR - true if either operand true"));
        self.register(Symbol::new("¬", Not, Logical,
            "Logical NOT - inverts boolean value"));
        self.register(Symbol::new("⊻", Xor, Logical,
            "Logical XOR - true if exactly one operand true"));

        // ═══════════════════════════════════════════════════════════════
        // ASSIGNMENT & EVALUATION
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⚙︎", Assign, Operator,
            "Assignment - evaluates and assigns to variable"));
        self.register(Symbol::new("→", Arrow, Operator,
            "Arrow - directs result to target variable"));
        self.register(Symbol::new("≔", Bind, Operator,
            "Bind - creates immutable binding"));

        // ═══════════════════════════════════════════════════════════════
        // INPUT/OUTPUT
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⚓", Input, InputOutput,
            "Input - reads value from user"));
        self.register(Symbol::new("⚡", Output, InputOutput,
            "Output - displays variable value"));
        self.register(Symbol::new("✤", Print, InputOutput,
            "Print - displays literal text"));
        self.register(Symbol::new("⌥", Debug, InputOutput,
            "Debug - displays debug information"));

        // ═══════════════════════════════════════════════════════════════
        // CONTROL FLOW
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⊂", LoopStart, ControlFlow,
            "Loop start - begins repetition block"));
        self.register(Symbol::new("⊃", LoopEnd, ControlFlow,
            "Loop end - ends repetition block"));
        self.register(Symbol::new("⟨", IfStart, ControlFlow,
            "If start - begins conditional block"));
        self.register(Symbol::new("⟩", Else, ControlFlow,
            "Else - alternative branch"));
        self.register(Symbol::new("⟫", IfEnd, ControlFlow,
            "If end - ends conditional block"));
        self.register(Symbol::new("⊗", Break, ControlFlow,
            "Break - exits current loop"));
        self.register(Symbol::new("↺", Continue, ControlFlow,
            "Continue - skips to next iteration"));
        self.register(Symbol::new("❧", EndProgram, ControlFlow,
            "End program - terminates execution"));

        // ═══════════════════════════════════════════════════════════════
        // FUNCTIONS
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("λ", FunctionStart, ControlFlow,
            "Function start - begins function definition"));
        self.register(Symbol::new("Λ", FunctionEnd, ControlFlow,
            "Function end - ends function definition"));
        self.register(Symbol::new("⤶", Return, ControlFlow,
            "Return - returns value from function"));
        self.register(Symbol::new("⤷", Call, ControlFlow,
            "Call - invokes a function"));

        // ═══════════════════════════════════════════════════════════════
        // PATTERN MATCHING (v0.3.0)
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⟡", MatchStart, ControlFlow,
            "Match start - begins pattern match expression"));
        self.register(Symbol::new("⟢", MatchArm, ControlFlow,
            "Match arm - defines a pattern case"));
        self.register(Symbol::new("⟣", MatchEnd, ControlFlow,
            "Match end - ends pattern match expression"));
        self.register(Symbol::new("◇", Wildcard, ControlFlow,
            "Wildcard - matches any value"));

        // ═══════════════════════════════════════════════════════════════
        // MODULE SYSTEM (v0.3.0)
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⟲", Import, ControlFlow,
            "Import - imports a module"));
        self.register(Symbol::new("⟳", Export, ControlFlow,
            "Export - exports a symbol"));
        self.register(Symbol::new("⊷", ModuleAccess, Operator,
            "Module access - accesses module member"));

        // ═══════════════════════════════════════════════════════════════
        // EXCEPTION HANDLING (v0.3.0)
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("☄", TryStart, ControlFlow,
            "Try - begins protected block"));
        self.register(Symbol::new("☊", CatchBlock, ControlFlow,
            "Catch - handles exception"));
        self.register(Symbol::new("☋", FinallyBlock, ControlFlow,
            "Finally - always executes"));
        self.register(Symbol::new("⚠", Throw, ControlFlow,
            "Throw - raises an exception"));

        // ═══════════════════════════════════════════════════════════════
        // MAP OPERATIONS (v0.3.0)
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⇒", MapArrow, Operator,
            "Map arrow - separates key from value"));
        self.register(Symbol::new("⋄", MapSeparator, Delimiter,
            "Map separator - separates entries"));
        self.register(Symbol::new("{", LeftBrace, Delimiter,
            "Left brace - opens map literal"));
        self.register(Symbol::new("}", RightBrace, Delimiter,
            "Right brace - closes map literal"));

        // ═══════════════════════════════════════════════════════════════
        // STACK OPERATIONS (for advanced low-level control)
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("⇑", Push, Operator,
            "Push - pushes value onto stack"));
        self.register(Symbol::new("⇓", Pop, Operator,
            "Pop - removes top value from stack"));
        self.register(Symbol::new("⇕", Dup, Operator,
            "Dup - duplicates top of stack"));
        self.register(Symbol::new("⇆", Swap, Operator,
            "Swap - swaps top two stack values"));
        self.register(Symbol::new("↻", Rotate, Operator,
            "Rotate - rotates top three stack values"));

        // ═══════════════════════════════════════════════════════════════
        // SPECIAL OPERATIONS
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("✹", Accumulator, Operator,
            "Accumulator - special counter variable"));
        self.register(Symbol::new("⊕", Increment, Operator,
            "Increment - increases value by one"));
        self.register(Symbol::new("⊖", Decrement, Operator,
            "Decrement - decreases value by one"));

        // ═══════════════════════════════════════════════════════════════
        // DELIMITERS
        // ═══════════════════════════════════════════════════════════════

        self.register(Symbol::new("[", LeftBracket, Delimiter,
            "Left bracket - opens expression"));
        self.register(Symbol::new("]", RightBracket, Delimiter,
            "Right bracket - closes expression"));
        self.register(Symbol::new("(", LeftParen, Delimiter,
            "Left paren - groups expression"));
        self.register(Symbol::new(")", RightParen, Delimiter,
            "Right paren - ends grouping"));
        self.register(Symbol::new(",", Separator, Delimiter,
            "Separator - separates arguments"));
        self.register(Symbol::new("⁂", Terminator, Delimiter,
            "Terminator - ends statement"));
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_lookup() {
        let table = SymbolTable::new();

        let int_symbol = table.lookup("⟁").unwrap();
        assert_eq!(int_symbol.meaning, SymbolMeaning::TypeInteger);

        let null_symbol = table.lookup("∅").unwrap();
        assert_eq!(null_symbol.meaning, SymbolMeaning::Null);
    }

    #[test]
    fn test_category_filter() {
        let table = SymbolTable::new();
        let type_symbols = table.symbols_in_category(SymbolCategory::TypeDeclaration);
        assert!(type_symbols.len() >= 4); // At least int, real, string, bool
    }
}
