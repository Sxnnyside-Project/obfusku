//! # VM Module
//!
//! The Obfusku Virtual Machine - a stack-based VM with symbolic execution.

pub mod stack;
pub mod context;
pub mod runtime;

pub use stack::{Stack, StackError};
pub use context::{Context, ContextError, CallFrame, Variable, Scope, ExceptionHandler};
pub use runtime::{Runtime, RuntimeError, RuntimeResult};
