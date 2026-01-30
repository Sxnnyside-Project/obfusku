//! # Bytecode System for Obfusku
//!
//! This module defines the bytecode representation used by the Obfusku VM.
//! Bytecode provides an efficient intermediate representation between
//! source code and execution.
//!
//! ## Design Philosophy
//!
//! The bytecode is designed to be:
//! - **Stack-oriented**: Most operations work on the stack
//! - **Symbol-derived**: Opcodes map closely to symbolic operations
//! - **Compact**: Minimal memory footprint
//! - **Extensible**: Easy to add new operations

use std::fmt;

/// Bytecode opcodes for the Obfusku VM
///
/// Each opcode represents a single operation that the VM can execute.
/// The opcodes are designed to map naturally from Obfusku symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    // ═══════════════════════════════════════════════════════════════
    // STACK OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Push a constant onto the stack (index follows)
    Const = 0x01,
    /// Push null onto the stack
    Null = 0x02,
    /// Push true onto the stack
    True = 0x03,
    /// Push false onto the stack
    False = 0x04,
    /// Pop the top value from the stack
    Pop = 0x05,
    /// Duplicate the top of the stack
    Dup = 0x06,
    /// Swap top two stack values
    Swap = 0x07,
    /// Rotate top three values (a b c -> c a b)
    Rot = 0x08,

    // ═══════════════════════════════════════════════════════════════
    // VARIABLE OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Declare a new variable (name index, type follows)
    DeclareVar = 0x10,
    /// Load variable value onto stack (name index follows)
    LoadVar = 0x11,
    /// Store stack top into variable (name index follows)
    StoreVar = 0x12,
    /// Load global variable
    LoadGlobal = 0x13,
    /// Store to global variable
    StoreGlobal = 0x14,

    // ═══════════════════════════════════════════════════════════════
    // ARITHMETIC OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Add top two stack values
    Add = 0x20,
    /// Subtract (second from top minus top)
    Sub = 0x21,
    /// Multiply top two values
    Mul = 0x22,
    /// Divide (second from top divided by top)
    Div = 0x23,
    /// Modulo
    Mod = 0x24,
    /// Power/Exponent
    Pow = 0x25,
    /// Negate top of stack
    Neg = 0x26,
    /// Increment top of stack
    Inc = 0x27,
    /// Decrement top of stack
    Dec = 0x28,

    // ═══════════════════════════════════════════════════════════════
    // COMPARISON OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Equal comparison
    Eq = 0x30,
    /// Not equal comparison
    Ne = 0x31,
    /// Less than
    Lt = 0x32,
    /// Greater than
    Gt = 0x33,
    /// Less than or equal
    Le = 0x34,
    /// Greater than or equal
    Ge = 0x35,

    // ═══════════════════════════════════════════════════════════════
    // LOGICAL OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Logical AND
    And = 0x40,
    /// Logical OR
    Or = 0x41,
    /// Logical NOT
    Not = 0x42,
    /// Logical XOR
    Xor = 0x43,

    // ═══════════════════════════════════════════════════════════════
    // CONTROL FLOW
    // ═══════════════════════════════════════════════════════════════

    /// Unconditional jump (offset follows)
    Jump = 0x50,
    /// Jump if top of stack is false
    JumpIfFalse = 0x51,
    /// Jump if top of stack is true
    JumpIfTrue = 0x52,
    /// Jump back (for loops)
    Loop = 0x53,
    /// Break from current loop
    Break = 0x54,
    /// Continue to next iteration
    Continue = 0x55,

    // ═══════════════════════════════════════════════════════════════
    // FUNCTION OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Call a function (function index + arity follows)
    Call = 0x60,
    /// Return from function
    Return = 0x61,
    /// Define a function (name index + arity + body length follows)
    DefineFunc = 0x62,
    /// Load function reference
    LoadFunc = 0x63,
    /// Call a closure value (arity follows, closure on stack)
    CallClosure = 0x5E,

    // ═══════════════════════════════════════════════════════════════
    // ARRAY OPERATIONS
    // ═══════════════════════════════════════════════════════════════

    /// Create array from N stack values (count follows)
    MakeArray = 0x64,
    /// Get array element (index on stack)
    ArrayGet = 0x65,
    /// Set array element (index and value on stack)
    ArraySet = 0x66,
    /// Get array length
    ArrayLen = 0x67,
    /// Push to array
    ArrayPush = 0x68,

    // ═══════════════════════════════════════════════════════════════
    // MAP OPERATIONS (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Create map from N key-value pairs on stack (count follows)
    MakeMap = 0x69,
    /// Get map value by key (key on stack)
    MapGet = 0x6A,
    /// Set map value (key and value on stack)
    MapSet = 0x6B,
    /// Check if map contains key
    MapHas = 0x6C,
    /// Remove key from map
    MapRemove = 0x6D,
    /// Get map keys as array
    MapKeys = 0x6E,
    /// Get map values as array
    MapValues = 0x6F,

    // ═══════════════════════════════════════════════════════════════
    // CLOSURE OPERATIONS (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Create closure capturing environment (func index + capture count follows)
    MakeClosure = 0xA0,
    /// Load captured variable (capture index follows)
    LoadCapture = 0xA1,
    /// Store to captured variable (capture index follows)
    StoreCapture = 0xA2,

    // ═══════════════════════════════════════════════════════════════
    // PATTERN MATCHING (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Begin match expression (arm count follows)
    MatchBegin = 0xA3,
    /// Test match arm pattern (jump offset if no match)
    MatchArm = 0xA4,
    /// End match expression
    MatchEnd = 0xA5,
    /// Wildcard pattern match (always succeeds)
    MatchWildcard = 0xA6,
    /// Bind matched value to variable
    MatchBind = 0xA7,

    // ═══════════════════════════════════════════════════════════════
    // MODULE OPERATIONS (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Import module (module name index follows)
    Import = 0xA8,
    /// Export symbol from current module
    Export = 0xA9,
    /// Load from imported module (module + symbol index)
    LoadModule = 0xAA,

    // ═══════════════════════════════════════════════════════════════
    // EXCEPTION HANDLING (v0.3.0)
    // ═══════════════════════════════════════════════════════════════

    /// Begin try block (handler offset follows)
    TryBegin = 0xB0,
    /// End try block
    TryEnd = 0xB1,
    /// Throw exception (value on stack)
    Throw = 0xB2,
    /// Catch exception (binds to variable)
    Catch = 0xB3,
    /// Finally block marker
    Finally = 0xB4,

    // ═══════════════════════════════════════════════════════════════
    // INPUT/OUTPUT
    // ═══════════════════════════════════════════════════════════════

    /// Print value from stack
    Print = 0x70,
    /// Print literal string (index follows)
    PrintLit = 0x71,
    /// Read input into variable
    Input = 0x72,
    /// Debug print
    Debug = 0x73,

    // ═══════════════════════════════════════════════════════════════
    // SPECIAL
    // ═══════════════════════════════════════════════════════════════

    /// Load accumulator value
    LoadAcc = 0x80,
    /// Store to accumulator
    StoreAcc = 0x81,
    /// Increment accumulator
    IncAcc = 0x82,
    /// Decrement accumulator
    DecAcc = 0x83,

    /// String concatenation
    Concat = 0x90,
    /// Type conversion (target type follows)
    Convert = 0x91,
    /// String interpolation (string index + var count follows)
    Interpolate = 0x92,

    /// No operation
    Nop = 0xFE,
    /// Halt execution
    Halt = 0xFF,
}

impl OpCode {
    /// Get the number of operand bytes this opcode expects
    pub fn operand_count(&self) -> usize {
        match self {
            OpCode::Const => 2,        // 16-bit constant index
            OpCode::DeclareVar => 3,   // 16-bit name + 8-bit type
            OpCode::LoadVar => 2,      // 16-bit name index
            OpCode::StoreVar => 2,     // 16-bit name index
            OpCode::LoadGlobal => 2,   // 16-bit name index
            OpCode::StoreGlobal => 2,  // 16-bit name index
            OpCode::Jump => 2,         // 16-bit offset
            OpCode::JumpIfFalse => 2,  // 16-bit offset
            OpCode::JumpIfTrue => 2,   // 16-bit offset
            OpCode::Loop => 2,         // 16-bit offset (negative)
            OpCode::Call => 3,         // 16-bit function index + 8-bit arity
            OpCode::DefineFunc => 5,   // 16-bit name + 8-bit arity + 16-bit body length
            OpCode::LoadFunc => 2,     // 16-bit function index
            OpCode::PrintLit => 2,     // 16-bit string index
            OpCode::Input => 3,        // 16-bit name + 8-bit type
            OpCode::Convert => 1,      // 8-bit target type
            OpCode::MakeArray => 2,    // 16-bit element count
            OpCode::Interpolate => 3,  // 16-bit string index + 8-bit var count
            // v0.3.0 opcodes
            OpCode::MakeMap => 2,      // 16-bit pair count
            OpCode::MakeClosure => 3,  // 16-bit func index + 8-bit capture count
            OpCode::LoadCapture => 2,  // 16-bit capture index
            OpCode::StoreCapture => 2, // 16-bit capture index
            OpCode::CallClosure => 1,  // 8-bit arity (closure on stack)
            OpCode::MatchBegin => 1,   // 8-bit arm count
            OpCode::MatchArm => 2,     // 16-bit jump offset
            OpCode::MatchBind => 2,    // 16-bit name index
            OpCode::Import => 2,       // 16-bit module name index
            OpCode::Export => 2,       // 16-bit symbol name index
            OpCode::LoadModule => 4,   // 16-bit module + 16-bit symbol
            OpCode::TryBegin => 2,     // 16-bit handler offset
            OpCode::Catch => 2,        // 16-bit variable name index
            _ => 0,
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Value types in Obfusku
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ValueType {
    Integer = 0,
    Real = 1,
    String = 2,
    Boolean = 3,
    Rune = 4,
    Array = 5,
    Map = 6,
    Null = 7,
    Function = 8,
    Closure = 9,
    Module = 10,
}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValueType::Integer => write!(f, "⟁"),
            ValueType::Real => write!(f, "⧆"),
            ValueType::String => write!(f, "⌘"),
            ValueType::Boolean => write!(f, "☍"),
            ValueType::Rune => write!(f, "ᚱ"),
            ValueType::Array => write!(f, "⌬"),
            ValueType::Map => write!(f, "⌖"),
            ValueType::Null => write!(f, "∅"),
            ValueType::Function => write!(f, "λ"),
            ValueType::Closure => write!(f, "λ⊃"),
            ValueType::Module => write!(f, "📦"),
        }
    }
}

/// Closure captures environment snapshot
#[derive(Debug, Clone, PartialEq)]
pub struct Closure {
    /// Index into function table
    pub function_index: usize,
    /// Captured values from enclosing scope
    pub captures: Vec<Value>,
}

impl Closure {
    pub fn new(function_index: usize, captures: Vec<Value>) -> Self {
        Self { function_index, captures }
    }
}

/// A runtime value in the VM
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i64),
    Real(f64),
    String(String),
    Boolean(bool),
    Rune(char),
    Array(Vec<Value>),
    /// Map using Vec for ordered iteration, could optimize with HashMap later
    Map(Vec<(Value, Value)>),
    Null,
    /// Index into function table
    Function(usize),
    /// Closure with captured environment
    ClosureVal(Box<Closure>),
    /// Module reference (module index)
    Module(usize),
}

impl Value {
    /// Get the type of this value
    pub fn value_type(&self) -> ValueType {
        match self {
            Value::Integer(_) => ValueType::Integer,
            Value::Real(_) => ValueType::Real,
            Value::String(_) => ValueType::String,
            Value::Boolean(_) => ValueType::Boolean,
            Value::Rune(_) => ValueType::Rune,
            Value::Array(_) => ValueType::Array,
            Value::Map(_) => ValueType::Map,
            Value::Null => ValueType::Null,
            Value::Function(_) => ValueType::Function,
            Value::ClosureVal(_) => ValueType::Closure,
            Value::Module(_) => ValueType::Module,
        }
    }

    /// Check if this value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Integer(i) => *i != 0,
            Value::Real(r) => *r != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Map(m) => !m.is_empty(),
            Value::Rune(c) => *c != '\0',
            Value::Function(_) => true,
            Value::ClosureVal(_) => true,
            Value::Module(_) => true,
        }
    }

    /// Convert to integer if possible
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            Value::Real(r) => Some(*r as i64),
            Value::Boolean(b) => Some(if *b { 1 } else { 0 }),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to real if possible
    pub fn to_real(&self) -> Option<f64> {
        match self {
            Value::Integer(i) => Some(*i as f64),
            Value::Real(r) => Some(*r),
            Value::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Convert to string
    pub fn to_string_value(&self) -> String {
        match self {
            Value::Integer(i) => i.to_string(),
            Value::Real(r) => r.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => if *b { "◉".to_string() } else { "◎".to_string() },
            Value::Rune(c) => c.to_string(),
            Value::Null => "∅".to_string(),
            Value::Array(a) => format!("⌬[{}]", a.len()),
            Value::Map(m) => format!("⌖{{{}}}", m.len()),
            Value::Function(i) => format!("λ#{}", i),
            Value::ClosureVal(c) => format!("λ⊃#{}", c.function_index),
            Value::Module(i) => format!("📦#{}", i),
        }
    }

    /// Check if value can be used as map key (hashable)
    pub fn is_hashable(&self) -> bool {
        matches!(self,
            Value::Integer(_) | Value::String(_) | Value::Boolean(_) |
            Value::Rune(_) | Value::Null
        )
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_value())
    }
}

/// Function information stored in the chunk
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    /// Name of the function
    pub name: String,
    /// Number of parameters
    pub arity: u8,
    /// Parameter names and types
    pub params: Vec<(String, ValueType)>,
    /// Start offset in bytecode
    pub start: usize,
    /// Length of function body
    pub length: usize,
    /// Names of captured variables (for closures)
    pub capture_names: Vec<String>,
}

impl FunctionInfo {
    pub fn new(name: String, arity: u8, params: Vec<(String, ValueType)>, start: usize, length: usize) -> Self {
        Self { name, arity, params, start, length, capture_names: Vec::new() }
    }

    /// Check if this function requires closure (has captures)
    pub fn is_closure(&self) -> bool {
        !self.capture_names.is_empty()
    }
}

/// A chunk of bytecode
///
/// Contains the bytecode instructions along with the constant pool
/// and debug information.
#[derive(Debug, Clone)]
pub struct Chunk {
    /// The bytecode instructions
    pub code: Vec<u8>,
    /// Constant pool
    pub constants: Vec<Value>,
    /// String pool for identifiers
    pub strings: Vec<String>,
    /// Line number information for debugging
    pub lines: Vec<usize>,
    /// Name of this chunk (for functions, modules)
    pub name: String,
    /// Function table
    pub functions: Vec<FunctionInfo>,
}

impl Chunk {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
            strings: Vec::new(),
            lines: Vec::new(),
            name: name.into(),
            functions: Vec::new(),
        }
    }

    /// Write a single byte
    pub fn write(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    /// Write an opcode
    pub fn write_op(&mut self, op: OpCode, line: usize) {
        self.write(op as u8, line);
    }

    /// Write a 16-bit value (little-endian)
    pub fn write_u16(&mut self, value: u16, line: usize) {
        self.write((value & 0xFF) as u8, line);
        self.write((value >> 8) as u8, line);
    }

    /// Add a constant and return its index
    pub fn add_constant(&mut self, value: Value) -> u16 {
        self.constants.push(value);
        (self.constants.len() - 1) as u16
    }

    /// Add a string and return its index
    pub fn add_string(&mut self, s: impl Into<String>) -> u16 {
        let s = s.into();
        // Check if string already exists
        if let Some(idx) = self.strings.iter().position(|x| x == &s) {
            return idx as u16;
        }
        self.strings.push(s);
        (self.strings.len() - 1) as u16
    }

    /// Add a function and return its index
    pub fn add_function(&mut self, func: FunctionInfo) -> u16 {
        self.functions.push(func);
        (self.functions.len() - 1) as u16
    }

    /// Get a function by index
    pub fn get_function(&self, index: usize) -> Option<&FunctionInfo> {
        self.functions.get(index)
    }

    /// Get the current instruction offset
    pub fn current_offset(&self) -> usize {
        self.code.len()
    }

    /// Patch a jump instruction at the given offset
    pub fn patch_jump(&mut self, offset: usize) {
        let jump = self.code.len() - offset - 2;
        self.code[offset] = (jump & 0xFF) as u8;
        self.code[offset + 1] = (jump >> 8) as u8;
    }

    /// Disassemble the chunk for debugging
    pub fn disassemble(&self) -> String {
        let mut output = format!("=== {} ===\n", self.name);
        let mut offset = 0;

        while offset < self.code.len() {
            let (instruction, new_offset) = self.disassemble_instruction(offset);
            output.push_str(&format!("{:04} {}\n", offset, instruction));
            offset = new_offset;
        }

        output
    }

    fn disassemble_instruction(&self, offset: usize) -> (String, usize) {
        let op = self.code[offset];
        let opcode = unsafe { std::mem::transmute::<u8, OpCode>(op) };

        let operand_count = opcode.operand_count();
        let mut offset = offset + 1;

        let instruction = match opcode {
            OpCode::Const => {
                let idx = self.read_u16(offset);
                offset += 2;
                format!("{} #{} ({})", opcode, idx, self.constants.get(idx as usize).map(|v| v.to_string()).unwrap_or_default())
            }
            OpCode::LoadVar | OpCode::StoreVar | OpCode::LoadGlobal | OpCode::StoreGlobal => {
                let idx = self.read_u16(offset);
                offset += 2;
                format!("{} #{} ({})", opcode, idx, self.strings.get(idx as usize).cloned().unwrap_or_default())
            }
            OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue | OpCode::Loop => {
                let target = self.read_u16(offset);
                offset += 2;
                format!("{} -> {}", opcode, target)
            }
            _ => {
                offset += operand_count;
                format!("{}", opcode)
            }
        };

        (instruction, offset)
    }

    fn read_u16(&self, offset: usize) -> u16 {
        let low = self.code[offset] as u16;
        let high = self.code[offset + 1] as u16;
        (high << 8) | low
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new("main")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_creation() {
        let mut chunk = Chunk::new("test");

        let idx = chunk.add_constant(Value::Integer(42));
        chunk.write_op(OpCode::Const, 1);
        chunk.write_u16(idx, 1);
        chunk.write_op(OpCode::Halt, 1);

        assert_eq!(chunk.code.len(), 4);
        assert_eq!(chunk.constants[0], Value::Integer(42));
    }

    #[test]
    fn test_value_truthiness() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(!Value::Null.is_truthy());
        assert!(Value::Integer(1).is_truthy());
        assert!(!Value::Integer(0).is_truthy());
    }
}
