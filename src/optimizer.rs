//! # Bytecode Optimizer for Obfusku
//!
//! Basic optimization passes for compiled bytecode.
//! Keeps the magic efficient without breaking the ritual.

use crate::bytecode::{Chunk, OpCode, Value};

/// Optimization pass that can be applied to a chunk
pub trait OptimizationPass {
    fn name(&self) -> &'static str;
    fn optimize(&self, chunk: &mut Chunk) -> usize; // Returns number of optimizations made
}

/// Constant folding - evaluate constant expressions at compile time
pub struct ConstantFolding;

impl OptimizationPass for ConstantFolding {
    fn name(&self) -> &'static str {
        "Constant Folding"
    }
    
    fn optimize(&self, chunk: &mut Chunk) -> usize {
        let mut optimizations = 0;
        let mut i = 0;
        
        while i + 6 < chunk.code.len() {
            // Look for pattern: Const, Const, BinaryOp
            if chunk.code[i] == OpCode::Const as u8 {
                let idx1_low = chunk.code[i + 1] as u16;
                let idx1_high = chunk.code[i + 2] as u16;
                let idx1 = (idx1_high << 8) | idx1_low;
                
                if i + 6 < chunk.code.len() && chunk.code[i + 3] == OpCode::Const as u8 {
                    let idx2_low = chunk.code[i + 4] as u16;
                    let idx2_high = chunk.code[i + 5] as u16;
                    let idx2 = (idx2_high << 8) | idx2_low;
                    
                    let op = chunk.code[i + 6];
                    
                    // Try to fold
                    if let (Some(v1), Some(v2)) = (
                        chunk.constants.get(idx1 as usize).cloned(),
                        chunk.constants.get(idx2 as usize).cloned()
                    ) {
                        if let Some(result) = Self::fold_binary(op, &v1, &v2) {
                            // Replace with single constant
                            let new_idx = chunk.add_constant(result);
                            chunk.code[i] = OpCode::Const as u8;
                            chunk.code[i + 1] = (new_idx & 0xFF) as u8;
                            chunk.code[i + 2] = (new_idx >> 8) as u8;
                            
                            // Replace rest with Nop
                            for j in (i + 3)..=(i + 6) {
                                chunk.code[j] = OpCode::Nop as u8;
                            }
                            
                            optimizations += 1;
                        }
                    }
                }
            }
            i += 1;
        }
        
        optimizations
    }
}

impl ConstantFolding {
    fn fold_binary(op: u8, a: &Value, b: &Value) -> Option<Value> {
        match (op, a, b) {
            (op, Value::Integer(x), Value::Integer(y)) if op == OpCode::Add as u8 => {
                x.checked_add(*y).map(Value::Integer)
            }
            (op, Value::Integer(x), Value::Integer(y)) if op == OpCode::Sub as u8 => {
                x.checked_sub(*y).map(Value::Integer)
            }
            (op, Value::Integer(x), Value::Integer(y)) if op == OpCode::Mul as u8 => {
                x.checked_mul(*y).map(Value::Integer)
            }
            (op, Value::Integer(x), Value::Integer(y)) if op == OpCode::Div as u8 && *y != 0 => {
                x.checked_div(*y).map(Value::Integer)
            }
            (op, Value::Real(x), Value::Real(y)) if op == OpCode::Add as u8 => {
                Some(Value::Real(x + y))
            }
            (op, Value::Real(x), Value::Real(y)) if op == OpCode::Sub as u8 => {
                Some(Value::Real(x - y))
            }
            (op, Value::Real(x), Value::Real(y)) if op == OpCode::Mul as u8 => {
                Some(Value::Real(x * y))
            }
            (op, Value::String(x), Value::String(y)) if op == OpCode::Add as u8 => {
                Some(Value::String(format!("{}{}", x, y)))
            }
            _ => None,
        }
    }
}

/// Remove Nop instructions
pub struct NopRemoval;

impl OptimizationPass for NopRemoval {
    fn name(&self) -> &'static str {
        "Nop Removal"
    }
    
    fn optimize(&self, chunk: &mut Chunk) -> usize {
        let original_len = chunk.code.len();
        
        // Build mapping of old offsets to new offsets
        let mut offset_map: Vec<usize> = Vec::with_capacity(chunk.code.len());
        let mut new_offset = 0;
        
        for i in 0..chunk.code.len() {
            offset_map.push(new_offset);
            if chunk.code[i] != OpCode::Nop as u8 {
                new_offset += 1;
            }
        }
        
        // Remove Nops
        chunk.code.retain(|&b| b != OpCode::Nop as u8);
        chunk.lines.retain(|_| true); // Keep lines for now (approximate)
        
        original_len - chunk.code.len()
    }
}

/// The optimizer that runs all passes
pub struct Optimizer {
    passes: Vec<Box<dyn OptimizationPass>>,
}

impl Optimizer {
    pub fn new() -> Self {
        Self {
            passes: vec![
                Box::new(ConstantFolding),
                Box::new(NopRemoval),
            ],
        }
    }
    
    /// Add a custom optimization pass
    pub fn add_pass(&mut self, pass: Box<dyn OptimizationPass>) {
        self.passes.push(pass);
    }
    
    /// Run all optimization passes on a chunk
    pub fn optimize(&self, chunk: &mut Chunk, verbose: bool) -> usize {
        let mut total = 0;
        
        for pass in &self.passes {
            let count = pass.optimize(chunk);
            if verbose && count > 0 {
                eprintln!("🔧 {} made {} optimization(s)", pass.name(), count);
            }
            total += count;
        }
        
        total
    }
}

impl Default for Optimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constant_folding() {
        let mut chunk = Chunk::new("test");
        
        // 5 + 3
        let idx1 = chunk.add_constant(Value::Integer(5));
        let idx2 = chunk.add_constant(Value::Integer(3));
        
        chunk.write_op(OpCode::Const, 1);
        chunk.write_u16(idx1, 1);
        chunk.write_op(OpCode::Const, 1);
        chunk.write_u16(idx2, 1);
        chunk.write_op(OpCode::Add, 1);
        chunk.write_op(OpCode::Halt, 1);
        
        let pass = ConstantFolding;
        let count = pass.optimize(&mut chunk);
        
        assert!(count > 0);
    }
}
