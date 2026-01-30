//! # Bytecode Module
//!
//! Bytecode representation for the Obfusku VM.

pub mod opcode;

pub use opcode::{Chunk, Closure, FunctionInfo, OpCode, Value, ValueType};
