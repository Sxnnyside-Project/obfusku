//! # VM Stack
//!
//! The execution stack for the Obfusku virtual machine.
//! Provides efficient stack operations with overflow protection.

use crate::bytecode::Value;
use thiserror::Error;

/// Stack-related errors
#[derive(Error, Debug, Clone, PartialEq)]
pub enum StackError {
    #[error("Stack overflow - maximum depth exceeded")]
    Overflow,

    #[error("Stack underflow - attempted to pop from empty stack")]
    Underflow,

    #[error("Invalid stack access at index {index}")]
    InvalidAccess { index: usize },
}

/// The maximum stack depth (configurable, but sane default)
pub const MAX_STACK_DEPTH: usize = 65536;

/// The VM execution stack
///
/// A simple, efficient stack implementation optimized for
/// the common case of push/pop operations.
#[derive(Debug)]
pub struct Stack {
    values: Vec<Value>,
    max_depth: usize,
}

impl Stack {
    /// Create a new empty stack
    pub fn new() -> Self {
        Self::with_capacity(256)
    }

    /// Create a stack with pre-allocated capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            max_depth: MAX_STACK_DEPTH,
        }
    }

    /// Push a value onto the stack
    pub fn push(&mut self, value: Value) -> Result<(), StackError> {
        if self.values.len() >= self.max_depth {
            return Err(StackError::Overflow);
        }
        self.values.push(value);
        Ok(())
    }

    /// Pop a value from the stack
    pub fn pop(&mut self) -> Result<Value, StackError> {
        self.values.pop().ok_or(StackError::Underflow)
    }

    /// Peek at the top value without removing it
    pub fn peek(&self) -> Result<&Value, StackError> {
        self.values.last().ok_or(StackError::Underflow)
    }

    /// Peek at a value at a specific depth (0 = top)
    pub fn peek_at(&self, depth: usize) -> Result<&Value, StackError> {
        if depth >= self.values.len() {
            return Err(StackError::InvalidAccess { index: depth });
        }
        Ok(&self.values[self.values.len() - 1 - depth])
    }

    /// Get a mutable reference to the top value
    pub fn peek_mut(&mut self) -> Result<&mut Value, StackError> {
        self.values.last_mut().ok_or(StackError::Underflow)
    }

    /// Duplicate the top value
    pub fn dup(&mut self) -> Result<(), StackError> {
        let value = self.peek()?.clone();
        self.push(value)
    }

    /// Swap the top two values
    pub fn swap(&mut self) -> Result<(), StackError> {
        let len = self.values.len();
        if len < 2 {
            return Err(StackError::Underflow);
        }
        self.values.swap(len - 1, len - 2);
        Ok(())
    }

    /// Rotate the top three values (a b c -> c a b)
    pub fn rotate(&mut self) -> Result<(), StackError> {
        let len = self.values.len();
        if len < 3 {
            return Err(StackError::Underflow);
        }
        // a b c -> c a b
        let c = self.values.remove(len - 1);
        self.values.insert(len - 3, c);
        Ok(())
    }

    /// Get the current stack depth
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if stack is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Clear the stack
    pub fn clear(&mut self) {
        self.values.clear();
    }

    /// Get all values (for debugging)
    pub fn values(&self) -> &[Value] {
        &self.values
    }

    /// Pop n values from the stack
    pub fn pop_n(&mut self, n: usize) -> Result<Vec<Value>, StackError> {
        if n > self.values.len() {
            return Err(StackError::Underflow);
        }
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            result.push(self.pop()?);
        }
        result.reverse(); // Maintain original order
        Ok(result)
    }

    /// Set the top value at a given depth
    pub fn set_at(&mut self, depth: usize, value: Value) -> Result<(), StackError> {
        if depth >= self.values.len() {
            return Err(StackError::InvalidAccess { index: depth });
        }
        let idx = self.values.len() - 1 - depth;
        self.values[idx] = value;
        Ok(())
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();

        stack.push(Value::Integer(1)).unwrap();
        stack.push(Value::Integer(2)).unwrap();
        stack.push(Value::Integer(3)).unwrap();

        assert_eq!(stack.pop().unwrap(), Value::Integer(3));
        assert_eq!(stack.pop().unwrap(), Value::Integer(2));
        assert_eq!(stack.pop().unwrap(), Value::Integer(1));
        assert!(stack.pop().is_err());
    }

    #[test]
    fn test_swap() {
        let mut stack = Stack::new();

        stack.push(Value::Integer(1)).unwrap();
        stack.push(Value::Integer(2)).unwrap();
        stack.swap().unwrap();

        assert_eq!(stack.pop().unwrap(), Value::Integer(1));
        assert_eq!(stack.pop().unwrap(), Value::Integer(2));
    }

    #[test]
    fn test_rotate() {
        let mut stack = Stack::new();

        stack.push(Value::Integer(1)).unwrap(); // a
        stack.push(Value::Integer(2)).unwrap(); // b
        stack.push(Value::Integer(3)).unwrap(); // c
        stack.rotate().unwrap(); // c a b

        assert_eq!(stack.pop().unwrap(), Value::Integer(2)); // b
        assert_eq!(stack.pop().unwrap(), Value::Integer(1)); // a
        assert_eq!(stack.pop().unwrap(), Value::Integer(3)); // c
    }
}
