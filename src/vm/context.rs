//! # VM Execution Context
//!
//! Manages execution state, variable scopes, and call frames.
//! The context provides the environment in which code executes.

use crate::bytecode::{Chunk, Value, ValueType};
use rustc_hash::FxHashMap;
use thiserror::Error;

/// Context-related errors
#[derive(Error, Debug, Clone)]
pub enum ContextError {
    #[error("Variable '{name}' is not declared in this spell")]
    UndeclaredVariable { name: String },

    #[error("Variable '{name}' is already declared in this scope")]
    DuplicateVariable { name: String },

    #[error("Type mismatch: expected {expected}, got {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("Cannot assign null to non-optional variable '{name}'")]
    NullAssignment { name: String },

    #[error("Call stack overflow - maximum depth exceeded")]
    CallStackOverflow,

    #[error("Call stack underflow - no frame to return from")]
    CallStackUnderflow,

    #[error("Function '{name}' not found")]
    FunctionNotFound { name: String },
}

/// Variable information
#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub var_type: ValueType,
    pub optional: bool,
    pub mutable: bool,
}

impl Variable {
    pub fn new(value: Value, var_type: ValueType, optional: bool) -> Self {
        Self {
            value,
            var_type,
            optional,
            mutable: true,
        }
    }

    pub fn immutable(value: Value, var_type: ValueType) -> Self {
        Self {
            value,
            var_type,
            optional: false,
            mutable: false,
        }
    }
}

/// A scope containing variables
#[derive(Debug, Clone, Default)]
pub struct Scope {
    variables: FxHashMap<String, Variable>,
}

impl Scope {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn declare(&mut self, name: String, var: Variable) -> Result<(), ContextError> {
        if self.variables.contains_key(&name) {
            return Err(ContextError::DuplicateVariable { name });
        }
        self.variables.insert(name, var);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Variable> {
        self.variables.get_mut(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}

/// A call frame representing a function invocation
#[derive(Debug, Clone)]
pub struct CallFrame {
    /// The bytecode chunk being executed
    pub chunk_index: usize,
    /// Instruction pointer within the chunk
    pub ip: usize,
    /// Base pointer for the stack frame
    pub base_pointer: usize,
    /// Local scope for this frame
    pub scope: Scope,
    /// Name of the function (for debugging)
    pub name: String,
    /// Closure environment (if this is a closure call)
    pub closure: Option<crate::bytecode::Closure>,
}

impl CallFrame {
    pub fn new(chunk_index: usize, base_pointer: usize, name: String) -> Self {
        Self {
            chunk_index,
            ip: 0,
            base_pointer,
            scope: Scope::new(),
            name,
            closure: None,
        }
    }
    
    /// Create a new call frame for a closure
    pub fn with_closure(chunk_index: usize, base_pointer: usize, name: String, closure: crate::bytecode::Closure) -> Self {
        Self {
            chunk_index,
            ip: 0,
            base_pointer,
            scope: Scope::new(),
            name,
            closure: Some(closure),
        }
    }
}

/// Maximum call stack depth
pub const MAX_CALL_DEPTH: usize = 1024;

/// Exception handler for try-catch blocks (v1.0.0)
#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    /// IP to jump to on exception
    pub handler_ip: usize,
    /// Optional finally block IP
    pub finally_ip: Option<usize>,
    /// Stack depth to restore on exception
    pub stack_depth: usize,
    /// Call frame depth when handler was set
    pub frame_depth: usize,
    /// Chunk index for the handler
    pub chunk_index: usize,
}

impl ExceptionHandler {
    pub fn new(handler_ip: usize, stack_depth: usize, frame_depth: usize, chunk_index: usize) -> Self {
        Self {
            handler_ip,
            finally_ip: None,
            stack_depth,
            frame_depth,
            chunk_index,
        }
    }
    
    pub fn with_finally(mut self, finally_ip: usize) -> Self {
        self.finally_ip = Some(finally_ip);
        self
    }
}

/// Execution context for the VM
///
/// Manages the call stack, global variables, and execution modes.
#[derive(Debug)]
pub struct Context {
    /// The call stack
    frames: Vec<CallFrame>,
    /// Global variables
    globals: Scope,
    /// Loaded chunks (bytecode)
    chunks: Vec<Chunk>,
    /// Function registry (name -> chunk index)
    functions: FxHashMap<String, usize>,
    /// The magical accumulator
    accumulator: i64,
    /// Accumulator stack for nested loops
    accumulator_stack: Vec<i64>,
    /// Execution flags
    pub halted: bool,
    /// Break flag for loop control
    pub break_flag: bool,
    /// Continue flag for loop control
    pub continue_flag: bool,
    /// Loop depth tracking
    loop_depth: usize,
    /// Exception handler stack (v1.0.0)
    exception_handlers: Vec<ExceptionHandler>,
    /// Current exception value (if any)
    pub current_exception: Option<crate::bytecode::Value>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            frames: Vec::with_capacity(64),
            globals: Scope::new(),
            chunks: Vec::new(),
            functions: FxHashMap::default(),
            accumulator: 0,
            accumulator_stack: Vec::new(),
            halted: false,
            break_flag: false,
            continue_flag: false,
            loop_depth: 0,
            exception_handlers: Vec::new(),
            current_exception: None,
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // EXCEPTION HANDLING (v1.0.0)
    // ═══════════════════════════════════════════════════════════════

    /// Push an exception handler
    pub fn push_exception_handler(&mut self, handler: ExceptionHandler) {
        self.exception_handlers.push(handler);
    }
    
    /// Pop the current exception handler
    pub fn pop_exception_handler(&mut self) -> Option<ExceptionHandler> {
        self.exception_handlers.pop()
    }
    
    /// Get the current exception handler (if any)
    pub fn current_exception_handler(&self) -> Option<&ExceptionHandler> {
        self.exception_handlers.last()
    }
    
    /// Check if we have an active exception handler
    pub fn has_exception_handler(&self) -> bool {
        !self.exception_handlers.is_empty()
    }

    // ═══════════════════════════════════════════════════════════════
    // CHUNK MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Add a chunk and return its index
    pub fn add_chunk(&mut self, chunk: Chunk) -> usize {
        let index = self.chunks.len();
        self.chunks.push(chunk);
        index
    }

    /// Get a chunk by index
    pub fn get_chunk(&self, index: usize) -> Option<&Chunk> {
        self.chunks.get(index)
    }

    /// Get the current chunk being executed
    pub fn current_chunk(&self) -> Option<&Chunk> {
        self.frames.last().and_then(|f| self.chunks.get(f.chunk_index))
    }

    // ═══════════════════════════════════════════════════════════════
    // CALL FRAME MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Push a new call frame
    pub fn push_frame(&mut self, frame: CallFrame) -> Result<(), ContextError> {
        if self.frames.len() >= MAX_CALL_DEPTH {
            return Err(ContextError::CallStackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    /// Pop the current call frame
    pub fn pop_frame(&mut self) -> Result<CallFrame, ContextError> {
        self.frames.pop().ok_or(ContextError::CallStackUnderflow)
    }

    /// Get the current call frame
    pub fn current_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    /// Get mutable reference to current frame
    pub fn current_frame_mut(&mut self) -> Option<&mut CallFrame> {
        self.frames.last_mut()
    }

    /// Get call stack depth
    pub fn call_depth(&self) -> usize {
        self.frames.len()
    }

    // ═══════════════════════════════════════════════════════════════
    // VARIABLE MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Declare a variable in the current scope
    pub fn declare_variable(
        &mut self,
        name: String,
        value: Value,
        var_type: ValueType,
        optional: bool,
    ) -> Result<(), ContextError> {
        let var = Variable::new(value, var_type, optional);

        if let Some(frame) = self.frames.last_mut() {
            frame.scope.declare(name, var)
        } else {
            self.globals.declare(name, var)
        }
    }

    /// Get a variable value, searching scopes from innermost to global
    pub fn get_variable(&self, name: &str) -> Result<&Value, ContextError> {
        // Search local scopes first (innermost to outermost)
        for frame in self.frames.iter().rev() {
            if let Some(var) = frame.scope.get(name) {
                return Ok(&var.value);
            }
        }

        // Then check globals
        if let Some(var) = self.globals.get(name) {
            return Ok(&var.value);
        }

        Err(ContextError::UndeclaredVariable { name: name.to_string() })
    }

    /// Set a variable value
    pub fn set_variable(&mut self, name: &str, value: Value) -> Result<(), ContextError> {
        // Search local scopes first
        for frame in self.frames.iter_mut().rev() {
            if let Some(var) = frame.scope.get_mut(name) {
                // Type check
                if !var.optional && matches!(value, Value::Null) {
                    return Err(ContextError::NullAssignment { name: name.to_string() });
                }
                if var.value.value_type() != value.value_type() && !matches!(value, Value::Null) {
                    return Err(ContextError::TypeMismatch {
                        expected: var.var_type.to_string(),
                        actual: value.value_type().to_string(),
                    });
                }
                var.value = value;
                return Ok(());
            }
        }

        // Then check globals
        if let Some(var) = self.globals.get_mut(name) {
            if !var.optional && matches!(value, Value::Null) {
                return Err(ContextError::NullAssignment { name: name.to_string() });
            }
            var.value = value;
            return Ok(());
        }

        Err(ContextError::UndeclaredVariable { name: name.to_string() })
    }

    /// Declare a global variable
    pub fn declare_global(
        &mut self,
        name: String,
        value: Value,
        var_type: ValueType,
        optional: bool,
    ) -> Result<(), ContextError> {
        let var = Variable::new(value, var_type, optional);
        self.globals.declare(name, var)
    }

    /// Get a global variable
    pub fn get_global(&self, name: &str) -> Result<&Value, ContextError> {
        self.globals
            .get(name)
            .map(|v| &v.value)
            .ok_or_else(|| ContextError::UndeclaredVariable { name: name.to_string() })
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: &str, value: Value) -> Result<(), ContextError> {
        if let Some(var) = self.globals.get_mut(name) {
            var.value = value;
            Ok(())
        } else {
            Err(ContextError::UndeclaredVariable { name: name.to_string() })
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // FUNCTION MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Register a function
    pub fn register_function(&mut self, name: String, chunk_index: usize) {
        self.functions.insert(name, chunk_index);
    }

    /// Get a function's chunk index
    pub fn get_function(&self, name: &str) -> Result<usize, ContextError> {
        self.functions
            .get(name)
            .copied()
            .ok_or_else(|| ContextError::FunctionNotFound { name: name.to_string() })
    }

    // ═══════════════════════════════════════════════════════════════
    // ACCUMULATOR (✹) MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Get the current accumulator value
    pub fn accumulator(&self) -> i64 {
        self.accumulator
    }

    /// Set the accumulator value
    pub fn set_accumulator(&mut self, value: i64) {
        self.accumulator = value;
    }

    /// Increment the accumulator
    pub fn increment_accumulator(&mut self) {
        self.accumulator += 1;
    }

    /// Decrement the accumulator
    pub fn decrement_accumulator(&mut self) {
        self.accumulator -= 1;
    }

    /// Push accumulator for nested loop
    pub fn push_accumulator(&mut self, initial: i64) {
        self.accumulator_stack.push(self.accumulator);
        self.accumulator = initial;
    }

    /// Pop accumulator when exiting loop
    pub fn pop_accumulator(&mut self) {
        if let Some(prev) = self.accumulator_stack.pop() {
            self.accumulator = prev;
        }
    }

    // ═══════════════════════════════════════════════════════════════
    // LOOP MANAGEMENT
    // ═══════════════════════════════════════════════════════════════

    /// Enter a loop
    pub fn enter_loop(&mut self) {
        self.loop_depth += 1;
    }

    /// Exit a loop
    pub fn exit_loop(&mut self) {
        if self.loop_depth > 0 {
            self.loop_depth -= 1;
        }
        self.break_flag = false;
        self.continue_flag = false;
    }

    /// Check if we're inside a loop
    pub fn in_loop(&self) -> bool {
        self.loop_depth > 0
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let mut ctx = Context::new();

        ctx.declare_global(
            "x".to_string(),
            Value::Integer(42),
            ValueType::Integer,
            false,
        ).unwrap();

        assert_eq!(*ctx.get_global("x").unwrap(), Value::Integer(42));
    }

    #[test]
    fn test_accumulator() {
        let mut ctx = Context::new();

        ctx.set_accumulator(5);
        assert_eq!(ctx.accumulator(), 5);

        ctx.increment_accumulator();
        assert_eq!(ctx.accumulator(), 6);

        ctx.decrement_accumulator();
        assert_eq!(ctx.accumulator(), 5);
    }

    #[test]
    fn test_nested_accumulator() {
        let mut ctx = Context::new();

        ctx.set_accumulator(10);
        ctx.push_accumulator(5);
        assert_eq!(ctx.accumulator(), 5);

        ctx.pop_accumulator();
        assert_eq!(ctx.accumulator(), 10);
    }
}
