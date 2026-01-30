//! # Obfusku Compiler
//!
//! Compiles Obfusku source code into bytecode for the VM.
//! This is a single-pass compiler that emits bytecode directly.

use crate::bytecode::{Chunk, FunctionInfo, OpCode, Value, ValueType};
use crate::lexer::{Lexer, LexerError, Token, TokenKind};
use crate::source_map::SourceMap;
use crate::symbols::{SymbolMeaning, SymbolTable};
use thiserror::Error;

/// Compilation errors with source context
#[derive(Error, Debug)]
pub enum CompileError {
    #[error("Lexer error: {0}")]
    LexerError(#[from] LexerError),

    #[error("🔮 Unexpected token '{lexeme}' at line {line}, column {column}\n   Expected: {expected}\n   {context}")]
    UnexpectedToken {
        lexeme: String,
        line: usize,
        column: usize,
        expected: String,
        context: String,
    },

    #[error("🔮 Unexpected end of spell. Expected {expected}")]
    UnexpectedEof { expected: String },

    #[error("🔮 Invalid expression at line {line}\n   {context}")]
    InvalidExpression { line: usize, context: String },

    #[error("⚠️ Spell does not end with ❧ — the universe remains unstable")]
    MissingEndProgram,

    #[error("📜 Too many constants in one spell (max 65535)")]
    TooManyConstants,

    #[error("📜 Too many local variables (max 65535)")]
    TooManyLocals,

    #[error("🔄 Loop nesting too deep")]
    LoopTooDeep,

    #[error("⚡ Function '{name}' is already defined")]
    DuplicateFunction { name: String },

    #[error("❓ Function '{name}' is not defined")]
    UndefinedFunction { name: String },

    #[error("🚫 Return statement outside of function — nowhere to return to")]
    ReturnOutsideFunction,

    #[error("⚡ Wrong number of arguments for '{name}': expected {expected}, got {got}")]
    WrongArity { name: String, expected: u8, got: u8 },

    #[error("🔮 Variable '{name}' not found in any scope")]
    UndefinedVariable { name: String },

    #[error("🔮 Cannot capture variable '{name}' — closure capture failed")]
    CaptureError { name: String },
}

type CompileResult<T> = Result<T, CompileError>;

/// Loop information for break/continue handling
struct LoopInfo {
    start: usize,
    break_jumps: Vec<usize>,
}

/// Function compilation state - used for tracking function context during compilation
/// Fields are used for closure capture analysis in v1.0.0
#[allow(dead_code)]
struct FunctionScope {
    /// Function name for error messages and debugging
    name: String,
    /// Number of parameters
    arity: u8,
    /// Parameter names and types for capture analysis
    params: Vec<(String, ValueType)>,
    /// Bytecode offset where function body starts
    start_offset: usize,
    /// Variables captured from outer scope (v1.0.0 closure support)
    captures: Vec<String>,
    /// Local variables declared in this function
    locals: Vec<String>,
    /// Whether this function needs closure (has captures)
    is_closure: bool,
}

impl FunctionScope {
    fn new(name: String, arity: u8, params: Vec<(String, ValueType)>, start_offset: usize) -> Self {
        let locals: Vec<String> = params.iter().map(|(n, _)| n.clone()).collect();
        Self {
            name,
            arity,
            params,
            start_offset,
            captures: Vec::new(),
            locals,
            is_closure: false,
        }
    }

    /// Check if a variable is local to this function
    fn is_local(&self, name: &str) -> bool {
        self.locals.contains(&name.to_string())
    }

    /// Add a local variable
    fn add_local(&mut self, name: String) {
        if !self.locals.contains(&name) {
            self.locals.push(name);
        }
    }

    /// Add a captured variable and return its index
    fn add_capture(&mut self, name: String) -> usize {
        if let Some(idx) = self.captures.iter().position(|n| n == &name) {
            idx
        } else {
            let idx = self.captures.len();
            self.captures.push(name);
            self.is_closure = true;
            idx
        }
    }

    /// Get capture index if variable is captured
    fn get_capture_index(&self, name: &str) -> Option<usize> {
        self.captures.iter().position(|n| n == name)
    }
}

/// The Obfusku compiler
pub struct Compiler<'a> {
    symbol_table: &'a SymbolTable,
    tokens: Vec<Token>,
    current: usize,
    chunk: Chunk,
    loops: Vec<LoopInfo>,
    had_end_program: bool,
    /// Stack of functions being compiled (supports nesting for closures)
    function_stack: Vec<FunctionScope>,
    /// Function name to index mapping
    function_indices: std::collections::HashMap<String, u16>,
    /// Source map for error reporting
    source_map: Option<SourceMap>,
    /// Original source code
    source: String,
    /// Closure capture info: function index -> capture variable names
    closure_captures: std::collections::HashMap<u16, Vec<String>>,
}

impl<'a> Compiler<'a> {
    /// Create a new compiler
    pub fn new(symbol_table: &'a SymbolTable) -> Self {
        Self {
            symbol_table,
            tokens: Vec::new(),
            current: 0,
            chunk: Chunk::new("main"),
            loops: Vec::new(),
            had_end_program: false,
            function_stack: Vec::new(),
            function_indices: std::collections::HashMap::new(),
            source_map: None,
            source: String::new(),
            closure_captures: std::collections::HashMap::new(),
        }
    }

    /// Compile source code into bytecode
    pub fn compile(&mut self, source: &str) -> CompileResult<Chunk> {
        // Create source map for error reporting
        self.source_map = Some(SourceMap::new(source));
        self.source = source.to_string();
        // Lex the source
        let mut lexer = Lexer::new(source, self.symbol_table);
        self.tokens = lexer.tokenize()?;
        self.current = 0;
        self.chunk = Chunk::new("main");
        self.loops.clear();
        self.had_end_program = false;
        self.function_stack.clear();
        self.function_indices.clear();
        self.closure_captures.clear();

        // Parse and compile statements
        while !self.is_at_end() {
            self.statement()?;
        }

        // Verify the program ends with ❧
        if !self.had_end_program {
            return Err(CompileError::MissingEndProgram);
        }

        Ok(std::mem::take(&mut self.chunk))
    }

    // ═══════════════════════════════════════════════════════════════
    // STATEMENT COMPILATION
    // ═══════════════════════════════════════════════════════════════

    fn statement(&mut self) -> CompileResult<()> {
        let token = self.peek();

        match &token.kind {
            // Type declarations (⟁, ⌘, ☍, etc.)
            TokenKind::Symbol(SymbolMeaning::TypeInteger) |
            TokenKind::Symbol(SymbolMeaning::TypeReal) |
            TokenKind::Symbol(SymbolMeaning::TypeString) |
            TokenKind::Symbol(SymbolMeaning::TypeBoolean) |
            TokenKind::Symbol(SymbolMeaning::TypeRune) => {
                self.variable_declaration()?;
            }

            // Assignment (⚙︎)
            TokenKind::Symbol(SymbolMeaning::Assign) => {
                self.assignment()?;
            }

            // Output (⚡)
            TokenKind::Symbol(SymbolMeaning::Output) => {
                self.output()?;
            }

            // Print literal (✤)
            TokenKind::Symbol(SymbolMeaning::Print) => {
                self.print_literal()?;
            }

            // Input (⚓)
            TokenKind::Symbol(SymbolMeaning::Input) => {
                self.input()?;
            }

            // Loop start (⊂)
            TokenKind::Symbol(SymbolMeaning::LoopStart) => {
                self.loop_statement()?;
            }

            // Conditional (⟨)
            TokenKind::Symbol(SymbolMeaning::IfStart) => {
                self.if_statement()?;
            }

            // Break (⊗)
            TokenKind::Symbol(SymbolMeaning::Break) => {
                self.break_statement()?;
            }

            // Continue (↺)
            TokenKind::Symbol(SymbolMeaning::Continue) => {
                self.continue_statement()?;
            }

            // Accumulator (✹)
            TokenKind::Symbol(SymbolMeaning::Accumulator) => {
                self.accumulator_statement()?;
            }

            // End program (❧)
            TokenKind::Symbol(SymbolMeaning::EndProgram) => {
                self.advance();
                self.emit_op(OpCode::Halt);
                self.had_end_program = true;
            }

            // Function definition (λ)
            TokenKind::Symbol(SymbolMeaning::FunctionStart) => {
                self.function_definition()?;
            }

            // Return (⤶)
            TokenKind::Symbol(SymbolMeaning::Return) => {
                self.return_statement()?;
            }

            // Array declaration (⌬)
            TokenKind::Symbol(SymbolMeaning::TypeArray) => {
                self.array_declaration()?;
            }

            // Map declaration (⌖) - v0.3.0
            TokenKind::Symbol(SymbolMeaning::TypeMap) => {
                self.map_declaration()?;
            }

            // Match expression (⟡) - v0.3.0
            TokenKind::Symbol(SymbolMeaning::MatchStart) => {
                self.match_expression()?;
            }

            // Try block (☄) - v0.3.0
            TokenKind::Symbol(SymbolMeaning::TryStart) => {
                self.try_statement()?;
            }

            // Throw (⚠) - v0.3.0
            TokenKind::Symbol(SymbolMeaning::Throw) => {
                self.throw_statement()?;
            }

            // Import (⟲) - v0.3.0
            TokenKind::Symbol(SymbolMeaning::Import) => {
                self.import_statement()?;
            }

            // Push to stack (⇑)
            TokenKind::Symbol(SymbolMeaning::Push) => {
                self.push_statement()?;
            }

            // Pop from stack (⇓)
            TokenKind::Symbol(SymbolMeaning::Pop) => {
                self.advance();
                self.emit_op(OpCode::Pop);
            }

            // Identifier - could be a variable operation
            TokenKind::Identifier(_) => {
                // Check if followed by assignment operator
                if self.check_next(TokenKind::Equals) {
                    self.simple_assignment()?;
                } else {
                    return Err(self.error(&format!("Unexpected identifier")));
                }
            }

            TokenKind::Eof => {
                // End of file
            }

            _ => {
                let token = self.advance();
                let context = self.get_source_context(token.location.line);
                return Err(CompileError::UnexpectedToken {
                    lexeme: token.lexeme,
                    line: token.location.line,
                    column: token.location.column,
                    expected: "statement".to_string(),
                    context,
                });
            }
        }

        Ok(())
    }

    /// Variable declaration: ⟁x=5 or ⌘name="hello"
    fn variable_declaration(&mut self) -> CompileResult<()> {
        let type_token = self.advance();
        let var_type = self.token_to_value_type(&type_token)?;

        // Optional modifier (∅ for optional)
        let optional = self.match_symbol(SymbolMeaning::Null);

        // Variable name
        let name_token = self.consume_identifier("variable name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };
        let name_idx = self.chunk.add_string(&name);

        // Register as local variable for closure capture analysis
        self.register_local(&name);

        // Optional initializer
        if self.match_token(TokenKind::Equals) {
            self.expression()?;
        } else if optional {
            self.emit_op(OpCode::Null);
        } else {
            // Default value based on type
            match var_type {
                ValueType::Integer => self.emit_constant(Value::Integer(0)),
                ValueType::Real => self.emit_constant(Value::Real(0.0)),
                ValueType::String => self.emit_constant(Value::String(String::new())),
                ValueType::Boolean => self.emit_op(OpCode::False),
                ValueType::Rune => self.emit_constant(Value::Rune('\0')),
                _ => self.emit_op(OpCode::Null),
            }
        }

        // Emit declaration
        self.emit_op(OpCode::DeclareVar);
        self.emit_u16(name_idx);
        self.emit_byte(var_type as u8);

        Ok(())
    }

    /// Assignment: ⚙︎[expr]→var
    fn assignment(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⚙︎

        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;
        self.expression()?;
        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        self.consume_symbol(SymbolMeaning::Arrow, "→")?;

        // Target can be a type symbol followed by identifier, or just identifier
        if let TokenKind::Symbol(meaning) = &self.peek().kind {
            if matches!(meaning,
                SymbolMeaning::TypeInteger |
                SymbolMeaning::TypeReal |
                SymbolMeaning::TypeString |
                SymbolMeaning::TypeBoolean |
                SymbolMeaning::Accumulator
            ) {
                if *meaning == SymbolMeaning::Accumulator {
                    self.advance();
                    self.emit_op(OpCode::StoreAcc);
                    return Ok(());
                }
                self.advance(); // skip type symbol
            }
        }

        let name_token = self.consume_identifier("variable name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };
        let name_idx = self.chunk.add_string(&name);

        self.emit_op(OpCode::StoreVar);
        self.emit_u16(name_idx);

        Ok(())
    }

    /// Simple assignment: var=expr
    fn simple_assignment(&mut self) -> CompileResult<()> {
        let name_token = self.advance();
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };
        let name_idx = self.chunk.add_string(&name);

        self.consume(TokenKind::Equals, "=")?;
        self.expression()?;

        self.emit_op(OpCode::StoreVar);
        self.emit_u16(name_idx);

        Ok(())
    }

    /// Output: ⚡[var]
    fn output(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⚡

        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;
        self.expression()?;
        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        self.emit_op(OpCode::Print);

        Ok(())
    }

    /// Print literal: ✤["text"]
    fn print_literal(&mut self) -> CompileResult<()> {
        self.advance(); // consume ✤

        // Can have optional brackets
        let has_bracket = self.match_symbol(SymbolMeaning::LeftBracket);

        let text = self.consume_string("string literal")?;
        let idx = self.chunk.add_string(&text);

        if has_bracket {
            self.consume_symbol(SymbolMeaning::RightBracket, "]")?;
        }

        self.emit_op(OpCode::PrintLit);
        self.emit_u16(idx);

        Ok(())
    }

    /// Input: ⚓⟁var
    fn input(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⚓

        let type_token = self.advance();
        let var_type = self.token_to_value_type(&type_token)?;

        let name_token = self.consume_identifier("variable name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };
        let name_idx = self.chunk.add_string(&name);

        self.emit_op(OpCode::Input);
        self.emit_u16(name_idx);
        self.emit_byte(var_type as u8);

        Ok(())
    }

    /// Loop: ⊂[condition] ... ⊃
    fn loop_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⊂

        let loop_start = self.chunk.current_offset();

        // Condition
        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;
        self.expression()?;
        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        // Jump if false (to loop end)
        self.emit_op(OpCode::JumpIfFalse);
        let exit_jump = self.chunk.current_offset();
        self.emit_u16(0xFFFF); // Placeholder

        // Loop body
        self.loops.push(LoopInfo {
            start: loop_start,
            break_jumps: Vec::new(),
        });

        while !self.check_symbol(SymbolMeaning::LoopEnd) && !self.is_at_end() {
            self.statement()?;
        }

        self.consume_symbol(SymbolMeaning::LoopEnd, "⊃")?;

        // Jump back to start
        self.emit_op(OpCode::Loop);
        let loop_offset = self.chunk.current_offset() - loop_start + 2;
        self.emit_u16(loop_offset as u16);

        // Patch exit jump
        self.chunk.patch_jump(exit_jump);

        // Patch break jumps
        let loop_info = self.loops.pop().unwrap();
        for jump in loop_info.break_jumps {
            self.chunk.patch_jump(jump);
        }

        Ok(())
    }

    /// Conditional: ⟨condition⟩...⟩else⟫
    fn if_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⟨

        self.expression()?;

        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        // Jump if false
        self.emit_op(OpCode::JumpIfFalse);
        let then_jump = self.chunk.current_offset();
        self.emit_u16(0xFFFF);

        // Then branch
        while !self.check_symbol(SymbolMeaning::Else) &&
              !self.check_symbol(SymbolMeaning::IfEnd) &&
              !self.is_at_end() {
            self.statement()?;
        }

        // Optional else
        if self.match_symbol(SymbolMeaning::Else) {
            self.emit_op(OpCode::Jump);
            let else_jump = self.chunk.current_offset();
            self.emit_u16(0xFFFF);

            self.chunk.patch_jump(then_jump);

            while !self.check_symbol(SymbolMeaning::IfEnd) && !self.is_at_end() {
                self.statement()?;
            }

            self.chunk.patch_jump(else_jump);
        } else {
            self.chunk.patch_jump(then_jump);
        }

        self.consume_symbol(SymbolMeaning::IfEnd, "⟫")?;

        Ok(())
    }

    /// Break statement
    fn break_statement(&mut self) -> CompileResult<()> {
        self.advance();

        if self.loops.is_empty() {
            return Err(self.error("⊗ (break) used outside of loop"));
        }

        self.emit_op(OpCode::Jump);
        let jump = self.chunk.current_offset();
        self.emit_u16(0xFFFF);

        self.loops.last_mut().unwrap().break_jumps.push(jump);

        Ok(())
    }

    /// Continue statement
    fn continue_statement(&mut self) -> CompileResult<()> {
        self.advance();

        if self.loops.is_empty() {
            return Err(self.error("↺ (continue) used outside of loop"));
        }

        let loop_start = self.loops.last().unwrap().start;
        let offset = self.chunk.current_offset() - loop_start + 3;

        self.emit_op(OpCode::Loop);
        self.emit_u16(offset as u16);

        Ok(())
    }

    /// Accumulator statement: ✹=value or ✹ (increment)
    fn accumulator_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ✹

        if self.match_token(TokenKind::Equals) {
            self.expression()?;
            self.emit_op(OpCode::StoreAcc);
        } else if self.match_symbol(SymbolMeaning::Increment) {
            self.emit_op(OpCode::IncAcc);
        } else if self.match_symbol(SymbolMeaning::Decrement) {
            self.emit_op(OpCode::DecAcc);
        } else {
            // Just ✹ means increment
            self.emit_op(OpCode::IncAcc);
        }

        Ok(())
    }

    /// Function definition: λname[params] ... Λ
    fn function_definition(&mut self) -> CompileResult<()> {
        self.advance(); // consume λ

        // Get function name
        let name_token = self.consume_identifier("function name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        // Check for duplicate
        if self.function_indices.contains_key(&name) {
            return Err(CompileError::DuplicateFunction { name });
        }

        // Parse parameters: [⟁a, ⌘b, ...]
        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;

        let mut params: Vec<(String, ValueType)> = Vec::new();

        if !self.check_symbol(SymbolMeaning::RightBracket) {
            loop {
                // Parse type
                let type_token = self.advance();
                let param_type = self.token_to_value_type(&type_token)?;

                // Parse name
                let param_name_token = self.consume_identifier("parameter name")?;
                let param_name = match &param_name_token.kind {
                    TokenKind::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };

                params.push((param_name, param_type));

                if !self.match_symbol(SymbolMeaning::Separator) {
                    break;
                }
            }
        }

        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        let arity = params.len() as u8;

        // Jump over function body
        self.emit_op(OpCode::Jump);
        let jump_over = self.chunk.current_offset();
        self.emit_u16(0xFFFF);

        let func_start = self.chunk.current_offset();

        // Enter function scope (push onto stack for nesting)
        self.function_stack.push(FunctionScope::new(
            name.clone(),
            arity,
            params.clone(),
            func_start,
        ));

        // Compile function body
        while !self.check_symbol(SymbolMeaning::FunctionEnd) && !self.is_at_end() {
            self.statement()?;
        }

        self.consume_symbol(SymbolMeaning::FunctionEnd, "Λ")?;

        // Implicit return null
        self.emit_op(OpCode::Null);
        self.emit_op(OpCode::Return);

        let func_length = self.chunk.current_offset() - func_start;

        // Patch jump
        self.chunk.patch_jump(jump_over);

        // Get closure info before popping
        let func_scope = self.function_stack.pop().unwrap();
        let is_closure = func_scope.is_closure;
        let captures = func_scope.captures.clone();

        // Register function with capture info
        let mut func_info = FunctionInfo::new(name.clone(), arity, params, func_start, func_length);
        func_info.capture_names = captures.clone();
        let func_idx = self.chunk.add_function(func_info);
        self.function_indices.insert(name.clone(), func_idx);

        // Store capture info for when function is loaded as value
        if is_closure {
            // Store the capture names in a hashmap for later
            self.closure_captures.insert(func_idx, captures);
        }

        Ok(())
    }

    /// Return statement: ⤶[expr] or ⤶
    fn return_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⤶

        if self.function_stack.is_empty() {
            return Err(CompileError::ReturnOutsideFunction);
        }

        // Optional return value
        if self.match_symbol(SymbolMeaning::LeftBracket) {
            self.expression()?;
            self.consume_symbol(SymbolMeaning::RightBracket, "]")?;
        } else {
            self.emit_op(OpCode::Null);
        }

        self.emit_op(OpCode::Return);

        Ok(())
    }

    /// Array declaration: ⌬name=[elements]
    fn array_declaration(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⌬

        // Variable name
        let name_token = self.consume_identifier("array name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };
        let name_idx = self.chunk.add_string(&name);

        self.consume(TokenKind::Equals, "=")?;
        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;

        // Parse elements
        let mut count: u16 = 0;

        if !self.check_symbol(SymbolMeaning::RightBracket) {
            loop {
                self.expression()?;
                count += 1;

                // Elements can be separated by spaces or commas
                if self.check_symbol(SymbolMeaning::RightBracket) {
                    break;
                }
                // Optional separator
                self.match_symbol(SymbolMeaning::Separator);
            }
        }

        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        // Create array from stack values
        self.emit_op(OpCode::MakeArray);
        self.emit_u16(count);

        // Declare variable
        self.emit_op(OpCode::DeclareVar);
        self.emit_u16(name_idx);
        self.emit_byte(ValueType::Array as u8);

        Ok(())
    }

    /// Map declaration: ⌖name={key⇒value, ...}
    fn map_declaration(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⌖

        // Variable name
        let name_token = self.consume_identifier("map name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };
        let name_idx = self.chunk.add_string(&name);

        self.consume(TokenKind::Equals, "=")?;
        self.consume_symbol(SymbolMeaning::LeftBrace, "{")?;

        // Parse key-value pairs
        let mut pair_count: u16 = 0;

        if !self.check_symbol(SymbolMeaning::RightBrace) {
            loop {
                // Key expression
                self.expression()?;

                // Expect ⇒ separator
                self.consume_symbol(SymbolMeaning::MapArrow, "⇒")?;

                // Value expression
                self.expression()?;

                pair_count += 1;

                // Check for more pairs
                if self.check_symbol(SymbolMeaning::RightBrace) {
                    break;
                }
                // Optional separator (⋄ or ,)
                if !self.match_symbol(SymbolMeaning::MapSeparator) {
                    self.match_symbol(SymbolMeaning::Separator);
                }
            }
        }

        self.consume_symbol(SymbolMeaning::RightBrace, "}")?;

        // Create map from stack values
        self.emit_op(OpCode::MakeMap);
        self.emit_u16(pair_count);

        // Declare variable
        self.emit_op(OpCode::DeclareVar);
        self.emit_u16(name_idx);
        self.emit_byte(ValueType::Map as u8);

        Ok(())
    }

    /// Match expression: ⟡expr] ⟢pattern] ... ⟢pattern] ... ⟣
    fn match_expression(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⟡

        // Evaluate expression to match
        self.expression()?;
        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        // Track jump patches for each arm
        let mut end_jumps: Vec<usize> = Vec::new();

        // Parse match arms
        while self.check_symbol(SymbolMeaning::MatchArm) {
            self.advance(); // consume ⟢

            // Duplicate the match value for comparison
            self.emit_op(OpCode::Dup);

            // Check for wildcard
            if self.match_symbol(SymbolMeaning::Wildcard) {
                // Wildcard matches anything - just pop the dup'd value
                self.emit_op(OpCode::Pop);
            } else {
                // Pattern expression
                self.expression()?;

                // Compare
                self.emit_op(OpCode::Eq);

                // Jump if no match
                self.emit_op(OpCode::JumpIfFalse);
                let no_match_jump = self.chunk.current_offset();
                self.emit_u16(0xFFFF);

                self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

                // Arm body
                while !self.check_symbol(SymbolMeaning::MatchArm) &&
                      !self.check_symbol(SymbolMeaning::MatchEnd) &&
                      !self.is_at_end() {
                    self.statement()?;
                }

                // Jump to end after arm executes
                self.emit_op(OpCode::Jump);
                end_jumps.push(self.chunk.current_offset());
                self.emit_u16(0xFFFF);

                // Patch the no-match jump
                self.chunk.patch_jump(no_match_jump);

                continue;
            }

            self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

            // Arm body (for wildcard)
            while !self.check_symbol(SymbolMeaning::MatchArm) &&
                  !self.check_symbol(SymbolMeaning::MatchEnd) &&
                  !self.is_at_end() {
                self.statement()?;
            }

            // Jump to end
            self.emit_op(OpCode::Jump);
            end_jumps.push(self.chunk.current_offset());
            self.emit_u16(0xFFFF);
        }

        self.consume_symbol(SymbolMeaning::MatchEnd, "⟣")?;

        // Pop the match value
        self.emit_op(OpCode::Pop);

        // Patch all end jumps
        for jump in end_jumps {
            self.chunk.patch_jump(jump);
        }

        Ok(())
    }

    /// Try statement: ☄ ... ☊ ... ☋ ... ⟣
    fn try_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ☄

        // Emit try begin with handler offset (to be patched)
        self.emit_op(OpCode::TryBegin);
        let handler_jump = self.chunk.current_offset();
        self.emit_u16(0xFFFF);

        // Try block body
        while !self.check_symbol(SymbolMeaning::CatchBlock) &&
              !self.check_symbol(SymbolMeaning::FinallyBlock) &&
              !self.check_symbol(SymbolMeaning::MatchEnd) &&
              !self.is_at_end() {
            self.statement()?;
        }

        self.emit_op(OpCode::TryEnd);

        // Jump over catch block
        self.emit_op(OpCode::Jump);
        let end_jump = self.chunk.current_offset();
        self.emit_u16(0xFFFF);

        // Patch handler jump
        self.chunk.patch_jump(handler_jump);

        // Catch block
        if self.match_symbol(SymbolMeaning::CatchBlock) {
            // Optional variable binding for exception
            if self.check_symbol(SymbolMeaning::LeftBracket) {
                self.advance();
                let var_token = self.consume_identifier("exception variable")?;
                let var_name = match &var_token.kind {
                    TokenKind::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };
                let var_idx = self.chunk.add_string(&var_name);
                self.emit_op(OpCode::Catch);
                self.emit_u16(var_idx);
                self.consume_symbol(SymbolMeaning::RightBracket, "]")?;
            }

            // Catch body
            while !self.check_symbol(SymbolMeaning::FinallyBlock) &&
                  !self.check_symbol(SymbolMeaning::MatchEnd) &&
                  !self.is_at_end() {
                self.statement()?;
            }
        }

        // Finally block (optional)
        if self.match_symbol(SymbolMeaning::FinallyBlock) {
            self.emit_op(OpCode::Finally);

            while !self.check_symbol(SymbolMeaning::MatchEnd) && !self.is_at_end() {
                self.statement()?;
            }
        }

        self.consume_symbol(SymbolMeaning::MatchEnd, "⟣")?;

        // Patch end jump
        self.chunk.patch_jump(end_jump);

        Ok(())
    }

    /// Throw statement: ⚠[expr]
    fn throw_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⚠

        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;
        self.expression()?;
        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        self.emit_op(OpCode::Throw);

        Ok(())
    }

    /// Import statement: ⟲"module_name"
    fn import_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⟲

        let module_name = self.consume_string("module name")?;
        let module_idx = self.chunk.add_string(&module_name);

        self.emit_op(OpCode::Import);
        self.emit_u16(module_idx);

        Ok(())
    }

    /// Push statement: ⇑[expr]
    fn push_statement(&mut self) -> CompileResult<()> {
        self.advance(); // consume ⇑

        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;
        self.expression()?;
        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        // Value is already on stack
        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // EXPRESSION COMPILATION
    // ═══════════════════════════════════════════════════════════════

    fn expression(&mut self) -> CompileResult<()> {
        self.or_expression()
    }

    fn or_expression(&mut self) -> CompileResult<()> {
        self.and_expression()?;

        while self.match_symbol(SymbolMeaning::Or) {
            self.and_expression()?;
            self.emit_op(OpCode::Or);
        }

        Ok(())
    }

    fn and_expression(&mut self) -> CompileResult<()> {
        self.equality_expression()?;

        while self.match_symbol(SymbolMeaning::And) {
            self.equality_expression()?;
            self.emit_op(OpCode::And);
        }

        Ok(())
    }

    fn equality_expression(&mut self) -> CompileResult<()> {
        self.comparison_expression()?;

        loop {
            if self.match_symbol(SymbolMeaning::Equal) {
                self.comparison_expression()?;
                self.emit_op(OpCode::Eq);
            } else if self.match_symbol(SymbolMeaning::NotEqual) {
                self.comparison_expression()?;
                self.emit_op(OpCode::Ne);
            } else {
                break;
            }
        }

        Ok(())
    }

    fn comparison_expression(&mut self) -> CompileResult<()> {
        self.additive_expression()?;

        loop {
            if self.match_symbol(SymbolMeaning::LessThan) {
                self.additive_expression()?;
                self.emit_op(OpCode::Lt);
            } else if self.match_symbol(SymbolMeaning::GreaterThan) {
                self.additive_expression()?;
                self.emit_op(OpCode::Gt);
            } else if self.match_symbol(SymbolMeaning::LessOrEqual) {
                self.additive_expression()?;
                self.emit_op(OpCode::Le);
            } else if self.match_symbol(SymbolMeaning::GreaterOrEqual) {
                self.additive_expression()?;
                self.emit_op(OpCode::Ge);
            } else {
                break;
            }
        }

        Ok(())
    }

    fn additive_expression(&mut self) -> CompileResult<()> {
        self.multiplicative_expression()?;

        loop {
            if self.match_symbol(SymbolMeaning::Add) {
                self.multiplicative_expression()?;
                self.emit_op(OpCode::Add);
            } else if self.match_symbol(SymbolMeaning::Subtract) {
                self.multiplicative_expression()?;
                self.emit_op(OpCode::Sub);
            } else {
                break;
            }
        }

        Ok(())
    }

    fn multiplicative_expression(&mut self) -> CompileResult<()> {
        self.unary_expression()?;

        loop {
            if self.match_symbol(SymbolMeaning::Multiply) {
                self.unary_expression()?;
                self.emit_op(OpCode::Mul);
            } else if self.match_symbol(SymbolMeaning::Divide) {
                self.unary_expression()?;
                self.emit_op(OpCode::Div);
            } else if self.match_symbol(SymbolMeaning::Modulo) {
                self.unary_expression()?;
                self.emit_op(OpCode::Mod);
            } else {
                break;
            }
        }

        Ok(())
    }

    fn unary_expression(&mut self) -> CompileResult<()> {
        if self.match_symbol(SymbolMeaning::Not) {
            self.unary_expression()?;
            self.emit_op(OpCode::Not);
        } else if self.match_symbol(SymbolMeaning::Subtract) {
            self.unary_expression()?;
            self.emit_op(OpCode::Neg);
        } else {
            self.primary()?;
        }

        Ok(())
    }

    fn primary(&mut self) -> CompileResult<()> {
        let token = self.advance();

        match &token.kind {
            TokenKind::Integer(i) => {
                self.emit_constant(Value::Integer(*i));
            }

            TokenKind::Float(f) => {
                self.emit_constant(Value::Real(*f));
            }

            TokenKind::String(s) => {
                self.emit_constant(Value::String(s.clone()));
            }

            TokenKind::Boolean(b) => {
                if *b {
                    self.emit_op(OpCode::True);
                } else {
                    self.emit_op(OpCode::False);
                }
            }

            TokenKind::Symbol(SymbolMeaning::Null) => {
                self.emit_op(OpCode::Null);
            }

            TokenKind::Symbol(SymbolMeaning::True) => {
                self.emit_op(OpCode::True);
            }

            TokenKind::Symbol(SymbolMeaning::False) => {
                self.emit_op(OpCode::False);
            }

            TokenKind::Symbol(SymbolMeaning::Accumulator) => {
                self.emit_op(OpCode::LoadAcc);
            }

            // Function call (⤷name[args])
            TokenKind::Symbol(SymbolMeaning::Call) => {
                self.function_call()?;
            }

            TokenKind::Symbol(SymbolMeaning::LeftParen) => {
                self.expression()?;
                self.consume_symbol(SymbolMeaning::RightParen, ")")?;
            }

            TokenKind::Identifier(name) => {
                // Check if this is a function name (for first-class function support)
                if let Some(&func_idx) = self.function_indices.get(name) {
                    // Check if this function needs closure (has captures)
                    if let Some(captures) = self.closure_captures.get(&func_idx).cloned() {
                        // Push captured values onto stack
                        for capture_name in &captures {
                            self.emit_variable_load(capture_name)?;
                        }
                        // Create closure
                        self.emit_op(OpCode::MakeClosure);
                        self.emit_u16(func_idx);
                        self.emit_byte(captures.len() as u8);
                    } else {
                        // Load function as value (no captures)
                        self.emit_op(OpCode::LoadFunc);
                        self.emit_u16(func_idx);
                    }
                } else {
                    // Check if we need to capture from outer scope
                    self.emit_variable_load(name)?;
                }
            }

            // Type prefix followed by identifier (⟁x, ⌘name, etc.)
            TokenKind::Symbol(meaning) if matches!(meaning,
                SymbolMeaning::TypeInteger |
                SymbolMeaning::TypeReal |
                SymbolMeaning::TypeString |
                SymbolMeaning::TypeBoolean) => {
                // Skip the type prefix and get the identifier
                let name_token = self.consume_identifier("variable name")?;
                let name = match &name_token.kind {
                    TokenKind::Identifier(s) => s.clone(),
                    _ => unreachable!(),
                };
                let name_idx = self.chunk.add_string(&name);
                self.emit_op(OpCode::LoadVar);
                self.emit_u16(name_idx);
            }

            _ => {
                let context = self.get_source_context(token.location.line);
                return Err(CompileError::UnexpectedToken {
                    lexeme: token.lexeme.clone(),
                    line: token.location.line,
                    column: token.location.column,
                    expected: "expression".to_string(),
                    context,
                });
            }
        }

        Ok(())
    }

    /// Function call: ⤷name[args]
    fn function_call(&mut self) -> CompileResult<()> {
        // Get function name
        let name_token = self.consume_identifier("function name")?;
        let name = match &name_token.kind {
            TokenKind::Identifier(s) => s.clone(),
            _ => unreachable!(),
        };

        self.consume_symbol(SymbolMeaning::LeftBracket, "[")?;

        // Parse arguments
        let mut arg_count: u8 = 0;

        if !self.check_symbol(SymbolMeaning::RightBracket) {
            loop {
                self.expression()?;
                arg_count += 1;

                if !self.match_symbol(SymbolMeaning::Separator) {
                    break;
                }
            }
        }

        self.consume_symbol(SymbolMeaning::RightBracket, "]")?;

        // Check if this is a direct function call or indirect (via variable)
        if let Some(&func_idx) = self.function_indices.get(&name) {
            // Direct function call
            self.emit_op(OpCode::Call);
            self.emit_u16(func_idx);
            self.emit_byte(arg_count);
        } else {
            // Indirect call - load the variable value and call as closure
            self.emit_variable_load(&name)?;
            self.emit_op(OpCode::CallClosure);
            self.emit_byte(arg_count);
        }

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // VARIABLE RESOLUTION WITH CLOSURE CAPTURE
    // ═══════════════════════════════════════════════════════════════

    /// Emit bytecode to load a variable, handling closure captures
    fn emit_variable_load(&mut self, name: &str) -> CompileResult<()> {
        // Check if we're inside a function
        if let Some(current_depth) = self.function_stack.len().checked_sub(1) {
            let current_func = &self.function_stack[current_depth];

            // Check if it's a local variable in current function
            if current_func.is_local(name) {
                let name_idx = self.chunk.add_string(name);
                self.emit_op(OpCode::LoadVar);
                self.emit_u16(name_idx);
                return Ok(());
            }

            // Check if already captured
            if let Some(capture_idx) = current_func.get_capture_index(name) {
                self.emit_op(OpCode::LoadCapture);
                self.emit_u16(capture_idx as u16);
                return Ok(());
            }

            // Check if variable exists in outer scopes (needs capture)
            let mut found_in_outer = false;
            for depth in (0..current_depth).rev() {
                if self.function_stack[depth].is_local(name) {
                    found_in_outer = true;
                    break;
                }
            }

            if found_in_outer {
                // Add to captures and emit LoadCapture
                // We need to modify function_stack mutably
                let capture_idx = self.function_stack[current_depth].add_capture(name.to_string());
                self.emit_op(OpCode::LoadCapture);
                self.emit_u16(capture_idx as u16);
                return Ok(());
            }
        }

        // Default: load as regular variable (global or not in function)
        let name_idx = self.chunk.add_string(name);
        self.emit_op(OpCode::LoadVar);
        self.emit_u16(name_idx);
        Ok(())
    }

    /// Register a local variable in the current function scope
    fn register_local(&mut self, name: &str) {
        if let Some(func) = self.function_stack.last_mut() {
            func.add_local(name.to_string());
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // EMIT HELPERS
    // ═══════════════════════════════════════════════════════════════

    fn emit_op(&mut self, op: OpCode) {
        let line = self.previous().location.line;
        self.chunk.write_op(op, line);
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous().location.line;
        self.chunk.write(byte, line);
    }

    fn emit_u16(&mut self, value: u16) {
        let line = self.previous().location.line;
        self.chunk.write_u16(value, line);
    }

    fn emit_constant(&mut self, value: Value) {
        let idx = self.chunk.add_constant(value);
        self.emit_op(OpCode::Const);
        self.emit_u16(idx);
    }

    // ═══════════════════════════════════════════════════════════════
    // TOKEN HELPERS
    // ═══════════════════════════════════════════════════════════════

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::Eof)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind)
    }

    fn check_symbol(&self, meaning: SymbolMeaning) -> bool {
        matches!(&self.peek().kind, TokenKind::Symbol(m) if *m == meaning)
    }

    fn check_next(&self, kind: TokenKind) -> bool {
        if self.current + 1 >= self.tokens.len() { return false; }
        std::mem::discriminant(&self.tokens[self.current + 1].kind) == std::mem::discriminant(&kind)
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn match_symbol(&mut self, meaning: SymbolMeaning) -> bool {
        if self.check_symbol(meaning) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn consume(&mut self, kind: TokenKind, expected: &str) -> CompileResult<Token> {
        if self.check(kind) {
            Ok(self.advance())
        } else if self.is_at_end() {
            Err(CompileError::UnexpectedEof { expected: expected.to_string() })
        } else {
            let token = self.peek().clone();
            let context = self.get_source_context(token.location.line);
            Err(CompileError::UnexpectedToken {
                lexeme: token.lexeme,
                line: token.location.line,
                column: token.location.column,
                expected: expected.to_string(),
                context,
            })
        }
    }

    fn consume_symbol(&mut self, meaning: SymbolMeaning, expected: &str) -> CompileResult<Token> {
        if self.check_symbol(meaning) {
            Ok(self.advance())
        } else if self.is_at_end() {
            Err(CompileError::UnexpectedEof { expected: expected.to_string() })
        } else {
            let token = self.peek().clone();
            let context = self.get_source_context(token.location.line);
            Err(CompileError::UnexpectedToken {
                lexeme: token.lexeme,
                line: token.location.line,
                column: token.location.column,
                expected: expected.to_string(),
                context,
            })
        }
    }

    fn consume_identifier(&mut self, expected: &str) -> CompileResult<Token> {
        if matches!(self.peek().kind, TokenKind::Identifier(_)) {
            Ok(self.advance())
        } else if self.is_at_end() {
            Err(CompileError::UnexpectedEof { expected: expected.to_string() })
        } else {
            let token = self.peek().clone();
            let context = self.get_source_context(token.location.line);
            Err(CompileError::UnexpectedToken {
                lexeme: token.lexeme,
                line: token.location.line,
                column: token.location.column,
                expected: expected.to_string(),
                context,
            })
        }
    }

    fn consume_string(&mut self, expected: &str) -> CompileResult<String> {
        if let TokenKind::String(s) = &self.peek().kind {
            let s = s.clone();
            self.advance();
            Ok(s)
        } else if self.is_at_end() {
            Err(CompileError::UnexpectedEof { expected: expected.to_string() })
        } else {
            let token = self.peek().clone();
            let context = self.get_source_context(token.location.line);
            Err(CompileError::UnexpectedToken {
                lexeme: token.lexeme,
                line: token.location.line,
                column: token.location.column,
                expected: expected.to_string(),
                context,
            })
        }
    }

    fn token_to_value_type(&self, token: &Token) -> CompileResult<ValueType> {
        match &token.kind {
            TokenKind::Symbol(SymbolMeaning::TypeInteger) => Ok(ValueType::Integer),
            TokenKind::Symbol(SymbolMeaning::TypeReal) => Ok(ValueType::Real),
            TokenKind::Symbol(SymbolMeaning::TypeString) => Ok(ValueType::String),
            TokenKind::Symbol(SymbolMeaning::TypeBoolean) => Ok(ValueType::Boolean),
            TokenKind::Symbol(SymbolMeaning::TypeRune) => Ok(ValueType::Rune),
            TokenKind::Symbol(SymbolMeaning::TypeArray) => Ok(ValueType::Array),
            TokenKind::Symbol(SymbolMeaning::TypeMap) => Ok(ValueType::Map),
            _ => {
                let context = self.get_source_context(token.location.line);
                Err(CompileError::UnexpectedToken {
                    lexeme: token.lexeme.clone(),
                    line: token.location.line,
                    column: token.location.column,
                    expected: "type symbol".to_string(),
                    context,
                })
            }
        }
    }

    /// Get source context for error messages
    fn get_source_context(&self, line: usize) -> String {
        if let Some(ref source_map) = self.source_map {
            if let Some(line_text) = source_map.get_line(line) {
                return format!("   │ {}", line_text);
            }
        }
        String::new()
    }

    fn error(&self, message: &str) -> CompileError {
        let token = self.peek();
        let context = self.get_source_context(token.location.line);
        CompileError::UnexpectedToken {
            lexeme: token.lexeme.clone(),
            line: token.location.line,
            column: token.location.column,
            expected: message.to_string(),
            context,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compile_source(source: &str) -> CompileResult<Chunk> {
        let table = SymbolTable::new();
        let mut compiler = Compiler::new(&table);
        compiler.compile(source)
    }

    #[test]
    fn test_simple_program() {
        let result = compile_source("⟁x=5\n⚡[x]\n❧");
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_end() {
        let result = compile_source("⟁x=5\n⚡[x]");
        assert!(matches!(result, Err(CompileError::MissingEndProgram)));
    }
}
