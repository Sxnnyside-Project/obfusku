//! # Obfusku Virtual Machine Runtime
//!
//! The core execution engine for Obfusku bytecode.
//! This is a stack-based VM with symbolic execution semantics.
//!
//! ## Design Philosophy
//!
//! The runtime is designed to:
//! - Execute bytecode efficiently
//! - Provide clear error messages with magical flavor
//! - Support extensibility through the context system
//! - Maintain the ritualistic feel of Obfusku

use crate::bytecode::{Chunk, OpCode, Value, ValueType};
use crate::vm::context::{CallFrame, Context, ContextError};
use crate::vm::stack::{Stack, StackError};
use std::io::{self, BufRead, Write};
use thiserror::Error;

/// Runtime errors - magical errors for magical code
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("🔮 Stack corruption: {0}")]
    StackError(#[from] StackError),

    #[error("🌀 Context corruption: {0}")]
    ContextError(#[from] ContextError),

    #[error("⚡ Type mismatch in magical operation: expected {expected}, found {actual}")]
    TypeMismatch { expected: String, actual: String },

    #[error("💀 Division by zero is forbidden in the arcane arts")]
    DivisionByZero,

    #[error("🔥 Arithmetic overflow - the numbers exceeded mortal bounds")]
    ArithmeticOverflow,

    #[error("❓ Unknown opcode {0:#04x} - this magic is not recognized")]
    UnknownOpcode(u8),

    #[error("📜 Invalid bytecode - the spell is corrupted at offset {offset}")]
    InvalidBytecode { offset: usize },

    #[error("🚫 Break used outside of a cycle - there is nothing to break from")]
    BreakOutsideLoop,

    #[error("🚫 Continue used outside of a cycle - there is no next iteration")]
    ContinueOutsideLoop,

    #[error("⚠️ Universe corruption - spell ended without ❧")]
    UniverseCorruption,

    #[error("📖 I/O error: {0}")]
    IoError(String),

    #[error("🔮 Invalid input: {0}")]
    InvalidInput(String),

    #[error("🔮 Function '{name}' not found")]
    FunctionNotFound { name: String },

    #[error("📊 Array index {index} out of bounds (length {length})")]
    IndexOutOfBounds { index: i64, length: usize },
}

/// Result type for runtime operations
pub type RuntimeResult<T> = Result<T, RuntimeError>;

/// The Obfusku Virtual Machine
///
/// Executes bytecode in a stack-based manner with symbolic semantics.
pub struct Runtime {
    /// Execution context (variables, call stack, etc.)
    context: Context,
    /// The execution stack
    stack: Stack,
    /// Whether to show debug output
    debug_mode: bool,
}

impl Runtime {
    /// Create a new runtime instance
    pub fn new() -> Self {
        Self {
            context: Context::new(),
            stack: Stack::new(),
            debug_mode: false,
        }
    }

    /// Enable or disable debug mode
    pub fn set_debug(&mut self, debug: bool) {
        self.debug_mode = debug;
    }

    /// Execute a chunk of bytecode
    pub fn execute(&mut self, chunk: Chunk) -> RuntimeResult<()> {
        let chunk_index = self.context.add_chunk(chunk);
        let frame = CallFrame::new(chunk_index, self.stack.len(), "main".to_string());
        self.context.push_frame(frame)?;

        self.run()
    }

    /// Main execution loop
    fn run(&mut self) -> RuntimeResult<()> {
        loop {
            // Check halt flag
            if self.context.halted {
                return Ok(());
            }

            // Get current instruction
            let (chunk_index, ip) = {
                let frame = self.context.current_frame()
                    .ok_or(RuntimeError::UniverseCorruption)?;
                (frame.chunk_index, frame.ip)
            };

            let chunk = self.context.get_chunk(chunk_index)
                .ok_or(RuntimeError::InvalidBytecode { offset: ip })?;

            // Check if we've reached the end
            if ip >= chunk.code.len() {
                // Pop the frame
                self.context.pop_frame()?;

                // If no more frames, we're done
                if self.context.current_frame().is_none() {
                    return Err(RuntimeError::UniverseCorruption);
                }
                continue;
            }

            // Decode and execute
            let opcode = chunk.code[ip];

            if self.debug_mode {
                eprintln!("[DEBUG] IP={:04} OP={:#04x} STACK={:?}", ip, opcode, self.stack.values());
            }

            // Advance IP before execution (so jumps work correctly)
            if let Some(frame) = self.context.current_frame_mut() {
                frame.ip += 1;
            }

            self.execute_instruction(opcode, chunk_index, ip)?;
        }
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, opcode: u8, chunk_index: usize, base_ip: usize) -> RuntimeResult<()> {
        let op: OpCode = unsafe { std::mem::transmute(opcode) };

        match op {
            // ═══════════════════════════════════════════════════════════
            // STACK OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::Const => {
                let idx = self.read_u16(chunk_index)?;
                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let value = chunk.constants.get(idx as usize)
                    .cloned()
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?;
                self.stack.push(value)?;
            }

            OpCode::Null => {
                self.stack.push(Value::Null)?;
            }

            OpCode::True => {
                self.stack.push(Value::Boolean(true))?;
            }

            OpCode::False => {
                self.stack.push(Value::Boolean(false))?;
            }

            OpCode::Pop => {
                self.stack.pop()?;
            }

            OpCode::Dup => {
                self.stack.dup()?;
            }

            OpCode::Swap => {
                self.stack.swap()?;
            }

            OpCode::Rot => {
                self.stack.rotate()?;
            }

            // ═══════════════════════════════════════════════════════════
            // VARIABLE OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::DeclareVar => {
                let name_idx = self.read_u16(chunk_index)?;
                let type_byte = self.read_byte(chunk_index)?;
                let var_type: ValueType = unsafe { std::mem::transmute(type_byte) };

                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let name = chunk.strings.get(name_idx as usize)
                    .cloned()
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?;

                let value = self.stack.pop()?;
                let optional = matches!(value, Value::Null);

                self.context.declare_variable(name, value, var_type, optional)?;
            }

            OpCode::LoadVar => {
                let name_idx = self.read_u16(chunk_index)?;
                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let name = chunk.strings.get(name_idx as usize)
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?;

                let value = self.context.get_variable(name)?.clone();
                self.stack.push(value)?;
            }

            OpCode::StoreVar => {
                let name_idx = self.read_u16(chunk_index)?;
                let name = {
                    let chunk = self.context.get_chunk(chunk_index).unwrap();
                    chunk.strings.get(name_idx as usize)
                        .cloned()
                        .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                };

                let value = self.stack.pop()?;
                self.context.set_variable(&name, value)?;
            }

            OpCode::LoadGlobal => {
                let name_idx = self.read_u16(chunk_index)?;
                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let name = chunk.strings.get(name_idx as usize)
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?;

                let value = self.context.get_global(name)?.clone();
                self.stack.push(value)?;
            }

            OpCode::StoreGlobal => {
                let name_idx = self.read_u16(chunk_index)?;
                let name = {
                    let chunk = self.context.get_chunk(chunk_index).unwrap();
                    chunk.strings.get(name_idx as usize)
                        .cloned()
                        .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                };

                let value = self.stack.pop()?;
                self.context.set_global(&name, value)?;
            }

            // ═══════════════════════════════════════════════════════════
            // ARITHMETIC OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::Add => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.add(a, b)?;
                self.stack.push(result)?;
            }

            OpCode::Sub => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.subtract(a, b)?;
                self.stack.push(result)?;
            }

            OpCode::Mul => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.multiply(a, b)?;
                self.stack.push(result)?;
            }

            OpCode::Div => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.divide(a, b)?;
                self.stack.push(result)?;
            }

            OpCode::Mod => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.modulo(a, b)?;
                self.stack.push(result)?;
            }

            OpCode::Pow => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.power(a, b)?;
                self.stack.push(result)?;
            }

            OpCode::Neg => {
                let a = self.stack.pop()?;
                let result = self.negate(a)?;
                self.stack.push(result)?;
            }

            OpCode::Inc => {
                let a = self.stack.pop()?;
                let result = match a {
                    Value::Integer(i) => Value::Integer(i.checked_add(1).ok_or(RuntimeError::ArithmeticOverflow)?),
                    Value::Real(r) => Value::Real(r + 1.0),
                    _ => return Err(RuntimeError::TypeMismatch {
                        expected: "numeric".to_string(),
                        actual: a.value_type().to_string(),
                    }),
                };
                self.stack.push(result)?;
            }

            OpCode::Dec => {
                let a = self.stack.pop()?;
                let result = match a {
                    Value::Integer(i) => Value::Integer(i.checked_sub(1).ok_or(RuntimeError::ArithmeticOverflow)?),
                    Value::Real(r) => Value::Real(r - 1.0),
                    _ => return Err(RuntimeError::TypeMismatch {
                        expected: "numeric".to_string(),
                        actual: a.value_type().to_string(),
                    }),
                };
                self.stack.push(result)?;
            }

            // ═══════════════════════════════════════════════════════════
            // COMPARISON OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::Eq => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(a == b))?;
            }

            OpCode::Ne => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(a != b))?;
            }

            OpCode::Lt => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.compare_less(a, b)?;
                self.stack.push(Value::Boolean(result))?;
            }

            OpCode::Gt => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = self.compare_less(b, a)?; // Note: swapped
                self.stack.push(Value::Boolean(result))?;
            }

            OpCode::Le => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = !self.compare_less(b, a)?;
                self.stack.push(Value::Boolean(result))?;
            }

            OpCode::Ge => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = !self.compare_less(a, b)?;
                self.stack.push(Value::Boolean(result))?;
            }

            // ═══════════════════════════════════════════════════════════
            // LOGICAL OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::And => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(a.is_truthy() && b.is_truthy()))?;
            }

            OpCode::Or => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(a.is_truthy() || b.is_truthy()))?;
            }

            OpCode::Not => {
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(!a.is_truthy()))?;
            }

            OpCode::Xor => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(Value::Boolean(a.is_truthy() ^ b.is_truthy()))?;
            }

            // ═══════════════════════════════════════════════════════════
            // CONTROL FLOW
            // ═══════════════════════════════════════════════════════════

            OpCode::Jump => {
                let offset = self.read_u16(chunk_index)?;
                if let Some(frame) = self.context.current_frame_mut() {
                    frame.ip += offset as usize;
                }
            }

            OpCode::JumpIfFalse => {
                let offset = self.read_u16(chunk_index)?;
                let condition = self.stack.pop()?;
                if !condition.is_truthy() {
                    if let Some(frame) = self.context.current_frame_mut() {
                        frame.ip += offset as usize;
                    }
                }
            }

            OpCode::JumpIfTrue => {
                let offset = self.read_u16(chunk_index)?;
                let condition = self.stack.pop()?;
                if condition.is_truthy() {
                    if let Some(frame) = self.context.current_frame_mut() {
                        frame.ip += offset as usize;
                    }
                }
            }

            OpCode::Loop => {
                let offset = self.read_u16(chunk_index)?;
                if let Some(frame) = self.context.current_frame_mut() {
                    frame.ip -= offset as usize;
                }
            }

            OpCode::Break => {
                if !self.context.in_loop() {
                    return Err(RuntimeError::BreakOutsideLoop);
                }
                self.context.break_flag = true;
            }

            OpCode::Continue => {
                if !self.context.in_loop() {
                    return Err(RuntimeError::ContinueOutsideLoop);
                }
                self.context.continue_flag = true;
            }

            // ═══════════════════════════════════════════════════════════
            // FUNCTION OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::Call => {
                let func_idx = self.read_u16(chunk_index)?;
                let arity = self.read_byte(chunk_index)?;

                // Get function info
                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let func_info = chunk.get_function(func_idx as usize)
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                    .clone();

                // Collect arguments from stack
                let mut args = Vec::with_capacity(arity as usize);
                for _ in 0..arity {
                    args.push(self.stack.pop()?);
                }
                args.reverse();

                // Create new frame
                let frame = CallFrame::new(chunk_index, self.stack.len(), func_info.name.clone());
                self.context.push_frame(frame)?;

                // Bind parameters to arguments
                for (i, (param_name, param_type)) in func_info.params.iter().enumerate() {
                    let value = args.get(i).cloned().unwrap_or(Value::Null);
                    self.context.declare_variable(param_name.clone(), value, *param_type, false)?;
                }

                // Jump to function start
                if let Some(frame) = self.context.current_frame_mut() {
                    frame.ip = func_info.start;
                }
            }

            OpCode::Return => {
                let result = self.stack.pop().unwrap_or(Value::Null);
                self.context.pop_frame()?;
                self.stack.push(result)?;
            }

            OpCode::DefineFunc => {
                // Skip - function definitions are handled at compile time
                let _name_idx = self.read_u16(chunk_index)?;
                let _arity = self.read_byte(chunk_index)?;
                let _body_len = self.read_u16(chunk_index)?;
            }

            OpCode::LoadFunc => {
                let func_idx = self.read_u16(chunk_index)?;
                self.stack.push(Value::Function(func_idx as usize))?;
            }

            OpCode::CallClosure => {
                let arity = self.read_byte(chunk_index)?;
                
                // Pop the closure from stack
                let closure_val = self.stack.pop()?;
                
                match closure_val {
                    Value::ClosureVal(closure) => {
                        // Get function info from closure
                        let chunk = self.context.get_chunk(chunk_index).unwrap();
                        let func_info = chunk.get_function(closure.function_index)
                            .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                            .clone();
                        
                        // Collect arguments from stack
                        let mut args = Vec::with_capacity(arity as usize);
                        for _ in 0..arity {
                            args.push(self.stack.pop()?);
                        }
                        args.reverse();
                        
                        // Create new frame with closure environment
                        let frame = CallFrame::with_closure(
                            chunk_index, 
                            self.stack.len(), 
                            func_info.name.clone(),
                            (*closure).clone()
                        );
                        self.context.push_frame(frame)?;
                        
                        // Bind parameters to arguments
                        for (i, (param_name, param_type)) in func_info.params.iter().enumerate() {
                            let value = args.get(i).cloned().unwrap_or(Value::Null);
                            self.context.declare_variable(param_name.clone(), value, *param_type, false)?;
                        }
                        
                        // Jump to function start
                        if let Some(frame) = self.context.current_frame_mut() {
                            frame.ip = func_info.start;
                        }
                    }
                    Value::Function(func_idx) => {
                        // Also support calling regular functions
                        let chunk = self.context.get_chunk(chunk_index).unwrap();
                        let func_info = chunk.get_function(func_idx)
                            .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                            .clone();
                        
                        let mut args = Vec::with_capacity(arity as usize);
                        for _ in 0..arity {
                            args.push(self.stack.pop()?);
                        }
                        args.reverse();
                        
                        let frame = CallFrame::new(chunk_index, self.stack.len(), func_info.name.clone());
                        self.context.push_frame(frame)?;
                        
                        for (i, (param_name, param_type)) in func_info.params.iter().enumerate() {
                            let value = args.get(i).cloned().unwrap_or(Value::Null);
                            self.context.declare_variable(param_name.clone(), value, *param_type, false)?;
                        }
                        
                        if let Some(frame) = self.context.current_frame_mut() {
                            frame.ip = func_info.start;
                        }
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "closure or function".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            // ═══════════════════════════════════════════════════════════
            // ARRAY OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::MakeArray => {
                let count = self.read_u16(chunk_index)?;
                let mut elements = Vec::with_capacity(count as usize);
                for _ in 0..count {
                    elements.push(self.stack.pop()?);
                }
                elements.reverse();
                self.stack.push(Value::Array(elements))?;
            }

            OpCode::ArrayGet => {
                let index = self.stack.pop()?;
                let array = self.stack.pop()?;

                match (array, index) {
                    (Value::Array(arr), Value::Integer(idx)) => {
                        let idx = if idx < 0 {
                            arr.len() as i64 + idx
                        } else {
                            idx
                        };

                        if idx < 0 || idx >= arr.len() as i64 {
                            return Err(RuntimeError::IndexOutOfBounds {
                                index: idx,
                                length: arr.len()
                            });
                        }

                        self.stack.push(arr[idx as usize].clone())?;
                    }
                    (arr, idx) => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "array and integer index".to_string(),
                            actual: format!("{} and {}", arr.value_type(), idx.value_type()),
                        });
                    }
                }
            }

            OpCode::ArraySet => {
                let value = self.stack.pop()?;
                let index = self.stack.pop()?;
                let array = self.stack.pop()?;

                match (array, index) {
                    (Value::Array(mut arr), Value::Integer(idx)) => {
                        let idx = if idx < 0 {
                            arr.len() as i64 + idx
                        } else {
                            idx
                        };

                        if idx < 0 || idx >= arr.len() as i64 {
                            return Err(RuntimeError::IndexOutOfBounds {
                                index: idx,
                                length: arr.len()
                            });
                        }

                        arr[idx as usize] = value;
                        self.stack.push(Value::Array(arr))?;
                    }
                    (arr, idx) => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "array and integer index".to_string(),
                            actual: format!("{} and {}", arr.value_type(), idx.value_type()),
                        });
                    }
                }
            }

            OpCode::ArrayLen => {
                let array = self.stack.pop()?;
                match array {
                    Value::Array(arr) => {
                        self.stack.push(Value::Integer(arr.len() as i64))?;
                    }
                    Value::String(s) => {
                        self.stack.push(Value::Integer(s.chars().count() as i64))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "array or string".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            OpCode::ArrayPush => {
                let value = self.stack.pop()?;
                let array = self.stack.pop()?;

                match array {
                    Value::Array(mut arr) => {
                        arr.push(value);
                        self.stack.push(Value::Array(arr))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "array".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            // ═══════════════════════════════════════════════════════════
            // MAP OPERATIONS (v0.3.0)
            // ═══════════════════════════════════════════════════════════

            OpCode::MakeMap => {
                let pair_count = self.read_u16(chunk_index)? as usize;
                let mut entries = Vec::with_capacity(pair_count);
                
                // Pop key-value pairs from stack
                for _ in 0..pair_count {
                    let value = self.stack.pop()?;
                    let key = self.stack.pop()?;
                    entries.push((key, value));
                }
                entries.reverse();
                
                self.stack.push(Value::Map(entries))?;
            }

            OpCode::MapGet => {
                let key = self.stack.pop()?;
                let map = self.stack.pop()?;
                
                match map {
                    Value::Map(entries) => {
                        let value = entries.iter()
                            .find(|(k, _)| k == &key)
                            .map(|(_, v)| v.clone())
                            .unwrap_or(Value::Null);
                        self.stack.push(value)?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "map".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            OpCode::MapSet => {
                let value = self.stack.pop()?;
                let key = self.stack.pop()?;
                let map = self.stack.pop()?;
                
                match map {
                    Value::Map(mut entries) => {
                        // Update existing or add new
                        if let Some(pos) = entries.iter().position(|(k, _)| k == &key) {
                            entries[pos] = (key, value);
                        } else {
                            entries.push((key, value));
                        }
                        self.stack.push(Value::Map(entries))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "map".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            OpCode::MapHas => {
                let key = self.stack.pop()?;
                let map = self.stack.pop()?;
                
                match map {
                    Value::Map(entries) => {
                        let has = entries.iter().any(|(k, _)| k == &key);
                        self.stack.push(Value::Boolean(has))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "map".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            OpCode::MapRemove => {
                let key = self.stack.pop()?;
                let map = self.stack.pop()?;
                
                match map {
                    Value::Map(mut entries) => {
                        entries.retain(|(k, _)| k != &key);
                        self.stack.push(Value::Map(entries))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "map".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            OpCode::MapKeys => {
                let map = self.stack.pop()?;
                
                match map {
                    Value::Map(entries) => {
                        let keys: Vec<Value> = entries.into_iter().map(|(k, _)| k).collect();
                        self.stack.push(Value::Array(keys))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "map".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            OpCode::MapValues => {
                let map = self.stack.pop()?;
                
                match map {
                    Value::Map(entries) => {
                        let values: Vec<Value> = entries.into_iter().map(|(_, v)| v).collect();
                        self.stack.push(Value::Array(values))?;
                    }
                    other => {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "map".to_string(),
                            actual: other.value_type().to_string(),
                        });
                    }
                }
            }

            // ═══════════════════════════════════════════════════════════
            // CLOSURE OPERATIONS (v1.0.0)
            // ═══════════════════════════════════════════════════════════

            OpCode::MakeClosure => {
                let func_idx = self.read_u16(chunk_index)? as usize;
                let capture_count = self.read_byte(chunk_index)? as usize;
                
                // Pop captured values from stack (in order)
                let mut captures = Vec::with_capacity(capture_count);
                for _ in 0..capture_count {
                    captures.push(self.stack.pop()?);
                }
                captures.reverse();
                
                let closure = crate::bytecode::Closure::new(func_idx, captures);
                self.stack.push(Value::ClosureVal(Box::new(closure)))?;
            }

            OpCode::LoadCapture => {
                let capture_idx = self.read_u16(chunk_index)? as usize;
                
                // Get current closure from call frame
                if let Some(frame) = self.context.current_frame() {
                    if let Some(closure) = &frame.closure {
                        if let Some(value) = closure.captures.get(capture_idx) {
                            self.stack.push(value.clone())?;
                        } else {
                            return Err(RuntimeError::InvalidBytecode { offset: base_ip });
                        }
                    } else {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "closure context".to_string(),
                            actual: "no closure in frame".to_string(),
                        });
                    }
                } else {
                    return Err(RuntimeError::UniverseCorruption);
                }
            }

            OpCode::StoreCapture => {
                let capture_idx = self.read_u16(chunk_index)? as usize;
                let value = self.stack.pop()?;
                
                // Store to current closure's captures
                if let Some(frame) = self.context.current_frame_mut() {
                    if let Some(ref mut closure) = frame.closure {
                        if capture_idx < closure.captures.len() {
                            closure.captures[capture_idx] = value;
                        } else {
                            return Err(RuntimeError::InvalidBytecode { offset: base_ip });
                        }
                    } else {
                        return Err(RuntimeError::TypeMismatch {
                            expected: "closure context".to_string(),
                            actual: "no closure in frame".to_string(),
                        });
                    }
                } else {
                    return Err(RuntimeError::UniverseCorruption);
                }
            }

            // ═══════════════════════════════════════════════════════════
            // PATTERN MATCHING (v1.0.0)
            // ═══════════════════════════════════════════════════════════

            OpCode::MatchBegin => {
                let _arm_count = self.read_byte(chunk_index)?;
                // Match value is on the stack, keep it there
            }

            OpCode::MatchArm => {
                let jump_offset = self.read_u16(chunk_index)?;
                // Compare top two stack values, jump if no match
                let pattern = self.stack.pop()?;
                let value = self.stack.peek()?.clone();
                
                if value != pattern {
                    if let Some(frame) = self.context.current_frame_mut() {
                        frame.ip += jump_offset as usize;
                    }
                }
            }

            OpCode::MatchEnd => {
                // Pop the match value from stack
                self.stack.pop()?;
            }

            OpCode::MatchWildcard => {
                // Wildcard always matches, do nothing
            }

            OpCode::MatchBind => {
                let name_idx = self.read_u16(chunk_index)?;
                let name = {
                    let chunk = self.context.get_chunk(chunk_index).unwrap();
                    chunk.strings.get(name_idx as usize)
                        .cloned()
                        .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                };
                
                // Bind matched value to variable
                let value = self.stack.peek()?.clone();
                self.context.declare_variable(name, value, ValueType::Null, true)?;
            }

            // ═══════════════════════════════════════════════════════════
            // MODULE OPERATIONS (v0.3.0) - Stub implementations
            // ═══════════════════════════════════════════════════════════

            OpCode::Import => {
                let _module_name_idx = self.read_u16(chunk_index)?;
                // TODO: implement module loading
            }

            OpCode::Export => {
                let _symbol_name_idx = self.read_u16(chunk_index)?;
                // TODO: implement symbol export
            }

            OpCode::LoadModule => {
                let _module_idx = self.read_u16(chunk_index)?;
                let _symbol_idx = self.read_u16(chunk_index)?;
                // TODO: implement module symbol loading
                self.stack.push(Value::Null)?;
            }

            // ═══════════════════════════════════════════════════════════
            // EXCEPTION HANDLING (v1.0.0)
            // ═══════════════════════════════════════════════════════════

            OpCode::TryBegin => {
                let handler_offset = self.read_u16(chunk_index)?;
                
                // Calculate handler IP
                let handler_ip = {
                    let frame = self.context.current_frame()
                        .ok_or(RuntimeError::UniverseCorruption)?;
                    frame.ip + handler_offset as usize
                };
                
                // Push exception handler
                let handler = crate::vm::context::ExceptionHandler::new(
                    handler_ip,
                    self.stack.len(),
                    self.context.call_depth(),
                    chunk_index,
                );
                self.context.push_exception_handler(handler);
            }

            OpCode::TryEnd => {
                // Pop exception handler on normal exit
                self.context.pop_exception_handler();
            }

            OpCode::Throw => {
                let exception = self.stack.pop()?;
                
                // Look for exception handler
                if self.context.has_exception_handler() {
                    let handler = self.context.pop_exception_handler().unwrap();
                    
                    // Restore stack to handler's stack depth
                    while self.stack.len() > handler.stack_depth {
                        self.stack.pop()?;
                    }
                    
                    // Unwind call frames if needed
                    while self.context.call_depth() > handler.frame_depth {
                        self.context.pop_frame()?;
                    }
                    
                    // Store exception for catch block
                    self.context.current_exception = Some(exception);
                    
                    // Jump to handler
                    if let Some(frame) = self.context.current_frame_mut() {
                        frame.ip = handler.handler_ip;
                    }
                } else {
                    // No handler - propagate as runtime error
                    return Err(RuntimeError::TypeMismatch {
                        expected: "exception handler".to_string(),
                        actual: format!("unhandled exception: {}", exception),
                    });
                }
            }

            OpCode::Catch => {
                let var_name_idx = self.read_u16(chunk_index)?;
                let name = {
                    let chunk = self.context.get_chunk(chunk_index).unwrap();
                    chunk.strings.get(var_name_idx as usize)
                        .cloned()
                        .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?
                };
                
                // Bind exception to variable
                if let Some(exception) = self.context.current_exception.take() {
                    self.context.declare_variable(name, exception, ValueType::Null, true)?;
                }
            }

            OpCode::Finally => {
                // Finally blocks execute regardless - handled by compiler jump logic
                // No runtime action needed
            }

            // ═══════════════════════════════════════════════════════════
            // INPUT/OUTPUT
            // ═══════════════════════════════════════════════════════════

            OpCode::Print => {
                let value = self.stack.pop()?;
                println!("{}", value);
            }

            OpCode::PrintLit => {
                let str_idx = self.read_u16(chunk_index)?;
                let chunk = self.context.get_chunk(chunk_index).unwrap();
                if let Some(s) = chunk.strings.get(str_idx as usize) {
                    println!("{}", s);
                }
            }

            OpCode::Input => {
                let name_idx = self.read_u16(chunk_index)?;
                let type_byte = self.read_byte(chunk_index)?;
                let var_type: ValueType = unsafe { std::mem::transmute(type_byte) };

                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let name = chunk.strings.get(name_idx as usize)
                    .cloned()
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?;

                let input = self.read_input()?;
                let value = self.parse_input(&input, var_type)?;

                self.context.set_variable(&name, value)?;
            }

            OpCode::Debug => {
                let value = self.stack.peek()?.clone();
                eprintln!("⌥ DEBUG: {:?}", value);
            }

            // ═══════════════════════════════════════════════════════════
            // SPECIAL OPERATIONS
            // ═══════════════════════════════════════════════════════════

            OpCode::LoadAcc => {
                let acc = self.context.accumulator();
                self.stack.push(Value::Integer(acc))?;
            }

            OpCode::StoreAcc => {
                let value = self.stack.pop()?;
                if let Some(i) = value.to_integer() {
                    self.context.set_accumulator(i);
                } else {
                    return Err(RuntimeError::TypeMismatch {
                        expected: "integer".to_string(),
                        actual: value.value_type().to_string(),
                    });
                }
            }

            OpCode::IncAcc => {
                self.context.increment_accumulator();
            }

            OpCode::DecAcc => {
                self.context.decrement_accumulator();
            }

            OpCode::Concat => {
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                let result = format!("{}{}", a.to_string_value(), b.to_string_value());
                self.stack.push(Value::String(result))?;
            }

            OpCode::Convert => {
                let type_byte = self.read_byte(chunk_index)?;
                let target_type: ValueType = unsafe { std::mem::transmute(type_byte) };
                let value = self.stack.pop()?;
                let converted = self.convert_value(value, target_type)?;
                self.stack.push(converted)?;
            }

            OpCode::Interpolate => {
                let str_idx = self.read_u16(chunk_index)?;
                let var_count = self.read_byte(chunk_index)?;

                let chunk = self.context.get_chunk(chunk_index).unwrap();
                let template = chunk.strings.get(str_idx as usize)
                    .cloned()
                    .ok_or(RuntimeError::InvalidBytecode { offset: base_ip })?;

                // Collect values from stack
                let mut values = Vec::with_capacity(var_count as usize);
                for _ in 0..var_count {
                    values.push(self.stack.pop()?);
                }
                values.reverse();

                // Perform interpolation: replace {0}, {1}, etc. with values
                let mut result = template;
                for (i, value) in values.iter().enumerate() {
                    let placeholder = format!("{{{}}}", i);
                    result = result.replace(&placeholder, &value.to_string_value());
                }

                self.stack.push(Value::String(result))?;
            }

            OpCode::Nop => {
                // No operation
            }

            OpCode::Halt => {
                self.context.halted = true;
            }

            #[allow(unreachable_patterns)]
            _ => {
                return Err(RuntimeError::UnknownOpcode(opcode));
            }
        }

        Ok(())
    }

    // ═══════════════════════════════════════════════════════════════
    // HELPER METHODS
    // ═══════════════════════════════════════════════════════════════

    /// Read the next byte from the current chunk
    fn read_byte(&mut self, chunk_index: usize) -> RuntimeResult<u8> {
        let ip = {
            let frame = self.context.current_frame()
                .ok_or(RuntimeError::UniverseCorruption)?;
            frame.ip
        };

        let chunk = self.context.get_chunk(chunk_index)
            .ok_or(RuntimeError::InvalidBytecode { offset: ip })?;

        let byte = chunk.code.get(ip)
            .copied()
            .ok_or(RuntimeError::InvalidBytecode { offset: ip })?;

        if let Some(frame) = self.context.current_frame_mut() {
            frame.ip += 1;
        }

        Ok(byte)
    }

    /// Read a 16-bit value from the current chunk
    fn read_u16(&mut self, chunk_index: usize) -> RuntimeResult<u16> {
        let low = self.read_byte(chunk_index)? as u16;
        let high = self.read_byte(chunk_index)? as u16;
        Ok((high << 8) | low)
    }

    /// Add two values
    fn add(&self, a: Value, b: Value) -> RuntimeResult<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                x.checked_add(y)
                    .map(Value::Integer)
                    .ok_or(RuntimeError::ArithmeticOverflow)
            }
            (Value::Real(x), Value::Real(y)) => Ok(Value::Real(x + y)),
            (Value::Integer(x), Value::Real(y)) => Ok(Value::Real(x as f64 + y)),
            (Value::Real(x), Value::Integer(y)) => Ok(Value::Real(x + y as f64)),
            (Value::String(x), Value::String(y)) => Ok(Value::String(x + &y)),
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "compatible types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Subtract two values
    fn subtract(&self, a: Value, b: Value) -> RuntimeResult<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                x.checked_sub(y)
                    .map(Value::Integer)
                    .ok_or(RuntimeError::ArithmeticOverflow)
            }
            (Value::Real(x), Value::Real(y)) => Ok(Value::Real(x - y)),
            (Value::Integer(x), Value::Real(y)) => Ok(Value::Real(x as f64 - y)),
            (Value::Real(x), Value::Integer(y)) => Ok(Value::Real(x - y as f64)),
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "numeric types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Multiply two values
    fn multiply(&self, a: Value, b: Value) -> RuntimeResult<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                x.checked_mul(y)
                    .map(Value::Integer)
                    .ok_or(RuntimeError::ArithmeticOverflow)
            }
            (Value::Real(x), Value::Real(y)) => Ok(Value::Real(x * y)),
            (Value::Integer(x), Value::Real(y)) => Ok(Value::Real(x as f64 * y)),
            (Value::Real(x), Value::Integer(y)) => Ok(Value::Real(x * y as f64)),
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "numeric types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Divide two values
    fn divide(&self, a: Value, b: Value) -> RuntimeResult<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if y == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                x.checked_div(y)
                    .map(Value::Integer)
                    .ok_or(RuntimeError::ArithmeticOverflow)
            }
            (Value::Real(x), Value::Real(y)) => {
                if y == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Real(x / y))
            }
            (Value::Integer(x), Value::Real(y)) => {
                if y == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Real(x as f64 / y))
            }
            (Value::Real(x), Value::Integer(y)) => {
                if y == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Real(x / y as f64))
            }
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "numeric types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Modulo operation
    fn modulo(&self, a: Value, b: Value) -> RuntimeResult<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if y == 0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Integer(x % y))
            }
            (Value::Real(x), Value::Real(y)) => {
                if y == 0.0 {
                    return Err(RuntimeError::DivisionByZero);
                }
                Ok(Value::Real(x % y))
            }
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "numeric types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Power operation
    fn power(&self, a: Value, b: Value) -> RuntimeResult<Value> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => {
                if y < 0 {
                    Ok(Value::Real((x as f64).powi(y as i32)))
                } else {
                    x.checked_pow(y as u32)
                        .map(Value::Integer)
                        .ok_or(RuntimeError::ArithmeticOverflow)
                }
            }
            (Value::Real(x), Value::Real(y)) => Ok(Value::Real(x.powf(y))),
            (Value::Integer(x), Value::Real(y)) => Ok(Value::Real((x as f64).powf(y))),
            (Value::Real(x), Value::Integer(y)) => Ok(Value::Real(x.powi(y as i32))),
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "numeric types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Negate a value
    fn negate(&self, a: Value) -> RuntimeResult<Value> {
        match a {
            Value::Integer(x) => x.checked_neg()
                .map(Value::Integer)
                .ok_or(RuntimeError::ArithmeticOverflow),
            Value::Real(x) => Ok(Value::Real(-x)),
            a => Err(RuntimeError::TypeMismatch {
                expected: "numeric".to_string(),
                actual: a.value_type().to_string(),
            }),
        }
    }

    /// Compare two values (less than)
    fn compare_less(&self, a: Value, b: Value) -> RuntimeResult<bool> {
        match (a, b) {
            (Value::Integer(x), Value::Integer(y)) => Ok(x < y),
            (Value::Real(x), Value::Real(y)) => Ok(x < y),
            (Value::Integer(x), Value::Real(y)) => Ok((x as f64) < y),
            (Value::Real(x), Value::Integer(y)) => Ok(x < (y as f64)),
            (Value::String(x), Value::String(y)) => Ok(x < y),
            (a, b) => Err(RuntimeError::TypeMismatch {
                expected: "comparable types".to_string(),
                actual: format!("{} and {}", a.value_type(), b.value_type()),
            }),
        }
    }

    /// Read input from stdin
    fn read_input(&self) -> RuntimeResult<String> {
        print!("⚓ "); // Input prompt
        io::stdout().flush().map_err(|e| RuntimeError::IoError(e.to_string()))?;

        let stdin = io::stdin();
        let mut line = String::new();
        stdin.lock().read_line(&mut line)
            .map_err(|e| RuntimeError::IoError(e.to_string()))?;

        Ok(line.trim().to_string())
    }

    /// Parse input string to a value of the given type
    fn parse_input(&self, input: &str, var_type: ValueType) -> RuntimeResult<Value> {
        match var_type {
            ValueType::Integer => input.parse::<i64>()
                .map(Value::Integer)
                .map_err(|_| RuntimeError::InvalidInput(format!("Expected integer, got '{}'", input))),
            ValueType::Real => input.parse::<f64>()
                .map(Value::Real)
                .map_err(|_| RuntimeError::InvalidInput(format!("Expected real number, got '{}'", input))),
            ValueType::String => Ok(Value::String(input.to_string())),
            ValueType::Boolean => match input.to_lowercase().as_str() {
                "true" | "◉" | "1" | "yes" => Ok(Value::Boolean(true)),
                "false" | "◎" | "0" | "no" => Ok(Value::Boolean(false)),
                _ => Err(RuntimeError::InvalidInput(format!("Expected boolean, got '{}'", input))),
            },
            ValueType::Rune => input.chars().next()
                .map(Value::Rune)
                .ok_or_else(|| RuntimeError::InvalidInput("Expected a character".to_string())),
            _ => Err(RuntimeError::InvalidInput(format!("Cannot input type {}", var_type))),
        }
    }

    /// Convert a value to another type
    fn convert_value(&self, value: Value, target: ValueType) -> RuntimeResult<Value> {
        match target {
            ValueType::Integer => value.to_integer()
                .map(Value::Integer)
                .ok_or_else(|| RuntimeError::TypeMismatch {
                    expected: "integer".to_string(),
                    actual: value.value_type().to_string(),
                }),
            ValueType::Real => value.to_real()
                .map(Value::Real)
                .ok_or_else(|| RuntimeError::TypeMismatch {
                    expected: "real".to_string(),
                    actual: value.value_type().to_string(),
                }),
            ValueType::String => Ok(Value::String(value.to_string_value())),
            ValueType::Boolean => Ok(Value::Boolean(value.is_truthy())),
            _ => Err(RuntimeError::TypeMismatch {
                expected: target.to_string(),
                actual: value.value_type().to_string(),
            }),
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let runtime = Runtime::new();

        let result = runtime.add(Value::Integer(5), Value::Integer(3)).unwrap();
        assert_eq!(result, Value::Integer(8));

        let result = runtime.subtract(Value::Integer(5), Value::Integer(3)).unwrap();
        assert_eq!(result, Value::Integer(2));

        let result = runtime.multiply(Value::Integer(5), Value::Integer(3)).unwrap();
        assert_eq!(result, Value::Integer(15));
    }

    #[test]
    fn test_division_by_zero() {
        let runtime = Runtime::new();

        let result = runtime.divide(Value::Integer(5), Value::Integer(0));
        assert!(matches!(result, Err(RuntimeError::DivisionByZero)));
    }
}
