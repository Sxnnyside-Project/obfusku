//! # Bytecode Serialization for Obfusku v1.0.0
//!
//! Allows saving compiled spells to disk and reloading them.
//! The format is version-aware for future compatibility.

use crate::bytecode::{Chunk, FunctionInfo, Value, ValueType};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::fs::File;
use std::path::Path;
use thiserror::Error;

/// Magic number for Obfusku bytecode files
const MAGIC: &[u8; 4] = b"OBFK";

/// Current bytecode format version
const VERSION_MAJOR: u8 = 1;
const VERSION_MINOR: u8 = 0;
const VERSION_PATCH: u8 = 0;

/// Serialization errors
#[derive(Error, Debug)]
pub enum SerializeError {
    #[error("📜 I/O error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("🔮 Invalid bytecode file - missing magic number")]
    InvalidMagic,
    
    #[error("⚠️ Incompatible bytecode version: file is {file_version}, runtime is {runtime_version}")]
    IncompatibleVersion { file_version: String, runtime_version: String },
    
    #[error("📜 Corrupted bytecode at offset {offset}")]
    CorruptedData { offset: usize },
    
    #[error("📜 Unknown value type: {0}")]
    UnknownValueType(u8),
}

/// Bytecode file header
#[derive(Debug, Clone)]
pub struct BytecodeHeader {
    pub magic: [u8; 4],
    pub version_major: u8,
    pub version_minor: u8,
    pub version_patch: u8,
    pub flags: u8,
}

impl BytecodeHeader {
    pub fn new() -> Self {
        Self {
            magic: *MAGIC,
            version_major: VERSION_MAJOR,
            version_minor: VERSION_MINOR,
            version_patch: VERSION_PATCH,
            flags: 0,
        }
    }
    
    pub fn version_string(&self) -> String {
        format!("{}.{}.{}", self.version_major, self.version_minor, self.version_patch)
    }
    
    pub fn is_compatible(&self) -> bool {
        // Compatible if major version matches
        self.version_major == VERSION_MAJOR
    }
}

impl Default for BytecodeHeader {
    fn default() -> Self {
        Self::new()
    }
}

/// Serializer for bytecode
pub struct BytecodeSerializer;

impl BytecodeSerializer {
    /// Save a chunk to a file
    pub fn save_to_file(chunk: &Chunk, path: impl AsRef<Path>) -> Result<(), SerializeError> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);
        Self::serialize(chunk, &mut writer)
    }
    
    /// Load a chunk from a file
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Chunk, SerializeError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        Self::deserialize(&mut reader)
    }
    
    /// Serialize chunk to writer
    pub fn serialize<W: Write>(chunk: &Chunk, writer: &mut W) -> Result<(), SerializeError> {
        let header = BytecodeHeader::new();
        
        // Write header
        writer.write_all(&header.magic)?;
        writer.write_all(&[header.version_major, header.version_minor, header.version_patch, header.flags])?;
        
        // Write chunk name
        Self::write_string(writer, &chunk.name)?;
        
        // Write code section
        Self::write_u32(writer, chunk.code.len() as u32)?;
        writer.write_all(&chunk.code)?;
        
        // Write constants
        Self::write_u16(writer, chunk.constants.len() as u16)?;
        for constant in &chunk.constants {
            Self::write_value(writer, constant)?;
        }
        
        // Write strings
        Self::write_u16(writer, chunk.strings.len() as u16)?;
        for s in &chunk.strings {
            Self::write_string(writer, s)?;
        }
        
        // Write line numbers
        Self::write_u32(writer, chunk.lines.len() as u32)?;
        for line in &chunk.lines {
            Self::write_u32(writer, *line as u32)?;
        }
        
        // Write functions
        Self::write_u16(writer, chunk.functions.len() as u16)?;
        for func in &chunk.functions {
            Self::write_function(writer, func)?;
        }
        
        Ok(())
    }
    
    /// Deserialize chunk from reader
    pub fn deserialize<R: Read>(reader: &mut R) -> Result<Chunk, SerializeError> {
        // Read and verify header
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if magic != *MAGIC {
            return Err(SerializeError::InvalidMagic);
        }
        
        let mut version = [0u8; 4];
        reader.read_exact(&mut version)?;
        let header = BytecodeHeader {
            magic,
            version_major: version[0],
            version_minor: version[1],
            version_patch: version[2],
            flags: version[3],
        };
        
        if !header.is_compatible() {
            return Err(SerializeError::IncompatibleVersion {
                file_version: header.version_string(),
                runtime_version: format!("{}.{}.{}", VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH),
            });
        }
        
        // Read chunk name
        let name = Self::read_string(reader)?;
        let mut chunk = Chunk::new(name);
        
        // Read code section
        let code_len = Self::read_u32(reader)? as usize;
        chunk.code = vec![0u8; code_len];
        reader.read_exact(&mut chunk.code)?;
        
        // Read constants
        let const_count = Self::read_u16(reader)? as usize;
        for _ in 0..const_count {
            let value = Self::read_value(reader)?;
            chunk.constants.push(value);
        }
        
        // Read strings
        let string_count = Self::read_u16(reader)? as usize;
        for _ in 0..string_count {
            let s = Self::read_string(reader)?;
            chunk.strings.push(s);
        }
        
        // Read line numbers
        let lines_count = Self::read_u32(reader)? as usize;
        for _ in 0..lines_count {
            let line = Self::read_u32(reader)? as usize;
            chunk.lines.push(line);
        }
        
        // Read functions
        let func_count = Self::read_u16(reader)? as usize;
        for _ in 0..func_count {
            let func = Self::read_function(reader)?;
            chunk.functions.push(func);
        }
        
        Ok(chunk)
    }
    
    // Helper methods for writing
    
    fn write_u16<W: Write>(writer: &mut W, value: u16) -> Result<(), SerializeError> {
        writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }
    
    fn write_u32<W: Write>(writer: &mut W, value: u32) -> Result<(), SerializeError> {
        writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }
    
    fn write_i64<W: Write>(writer: &mut W, value: i64) -> Result<(), SerializeError> {
        writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }
    
    fn write_f64<W: Write>(writer: &mut W, value: f64) -> Result<(), SerializeError> {
        writer.write_all(&value.to_le_bytes())?;
        Ok(())
    }
    
    fn write_string<W: Write>(writer: &mut W, s: &str) -> Result<(), SerializeError> {
        let bytes = s.as_bytes();
        Self::write_u16(writer, bytes.len() as u16)?;
        writer.write_all(bytes)?;
        Ok(())
    }
    
    fn write_value<W: Write>(writer: &mut W, value: &Value) -> Result<(), SerializeError> {
        match value {
            Value::Null => {
                writer.write_all(&[ValueType::Null as u8])?;
            }
            Value::Integer(i) => {
                writer.write_all(&[ValueType::Integer as u8])?;
                Self::write_i64(writer, *i)?;
            }
            Value::Real(r) => {
                writer.write_all(&[ValueType::Real as u8])?;
                Self::write_f64(writer, *r)?;
            }
            Value::Boolean(b) => {
                writer.write_all(&[ValueType::Boolean as u8, if *b { 1 } else { 0 }])?;
            }
            Value::String(s) => {
                writer.write_all(&[ValueType::String as u8])?;
                Self::write_string(writer, s)?;
            }
            Value::Rune(c) => {
                writer.write_all(&[ValueType::Rune as u8])?;
                Self::write_u32(writer, *c as u32)?;
            }
            Value::Array(arr) => {
                writer.write_all(&[ValueType::Array as u8])?;
                Self::write_u16(writer, arr.len() as u16)?;
                for v in arr {
                    Self::write_value(writer, v)?;
                }
            }
            Value::Map(map) => {
                writer.write_all(&[ValueType::Map as u8])?;
                Self::write_u16(writer, map.len() as u16)?;
                for (k, v) in map {
                    Self::write_value(writer, k)?;
                    Self::write_value(writer, v)?;
                }
            }
            Value::Function(idx) => {
                writer.write_all(&[ValueType::Function as u8])?;
                Self::write_u16(writer, *idx as u16)?;
            }
            Value::ClosureVal(c) => {
                writer.write_all(&[ValueType::Closure as u8])?;
                Self::write_u16(writer, c.function_index as u16)?;
                Self::write_u16(writer, c.captures.len() as u16)?;
                for cap in &c.captures {
                    Self::write_value(writer, cap)?;
                }
            }
            Value::Module(idx) => {
                writer.write_all(&[ValueType::Module as u8])?;
                Self::write_u16(writer, *idx as u16)?;
            }
        }
        Ok(())
    }
    
    fn write_function<W: Write>(writer: &mut W, func: &FunctionInfo) -> Result<(), SerializeError> {
        Self::write_string(writer, &func.name)?;
        writer.write_all(&[func.arity])?;
        Self::write_u16(writer, func.params.len() as u16)?;
        for (name, ty) in &func.params {
            Self::write_string(writer, name)?;
            writer.write_all(&[*ty as u8])?;
        }
        Self::write_u32(writer, func.start as u32)?;
        Self::write_u32(writer, func.length as u32)?;
        Ok(())
    }
    
    // Helper methods for reading
    
    fn read_u16<R: Read>(reader: &mut R) -> Result<u16, SerializeError> {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }
    
    fn read_u32<R: Read>(reader: &mut R) -> Result<u32, SerializeError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }
    
    fn read_i64<R: Read>(reader: &mut R) -> Result<i64, SerializeError> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(i64::from_le_bytes(buf))
    }
    
    fn read_f64<R: Read>(reader: &mut R) -> Result<f64, SerializeError> {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf)?;
        Ok(f64::from_le_bytes(buf))
    }
    
    fn read_string<R: Read>(reader: &mut R) -> Result<String, SerializeError> {
        let len = Self::read_u16(reader)? as usize;
        let mut buf = vec![0u8; len];
        reader.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|_| SerializeError::CorruptedData { offset: 0 })
    }
    
    fn read_value<R: Read>(reader: &mut R) -> Result<Value, SerializeError> {
        let mut type_byte = [0u8; 1];
        reader.read_exact(&mut type_byte)?;
        
        match type_byte[0] {
            x if x == ValueType::Null as u8 => Ok(Value::Null),
            x if x == ValueType::Integer as u8 => {
                let i = Self::read_i64(reader)?;
                Ok(Value::Integer(i))
            }
            x if x == ValueType::Real as u8 => {
                let r = Self::read_f64(reader)?;
                Ok(Value::Real(r))
            }
            x if x == ValueType::Boolean as u8 => {
                let mut b = [0u8; 1];
                reader.read_exact(&mut b)?;
                Ok(Value::Boolean(b[0] != 0))
            }
            x if x == ValueType::String as u8 => {
                let s = Self::read_string(reader)?;
                Ok(Value::String(s))
            }
            x if x == ValueType::Rune as u8 => {
                let c = Self::read_u32(reader)?;
                Ok(Value::Rune(char::from_u32(c).unwrap_or('\0')))
            }
            x if x == ValueType::Array as u8 => {
                let len = Self::read_u16(reader)? as usize;
                let mut arr = Vec::with_capacity(len);
                for _ in 0..len {
                    arr.push(Self::read_value(reader)?);
                }
                Ok(Value::Array(arr))
            }
            x if x == ValueType::Map as u8 => {
                let len = Self::read_u16(reader)? as usize;
                let mut map = Vec::with_capacity(len);
                for _ in 0..len {
                    let k = Self::read_value(reader)?;
                    let v = Self::read_value(reader)?;
                    map.push((k, v));
                }
                Ok(Value::Map(map))
            }
            x if x == ValueType::Function as u8 => {
                let idx = Self::read_u16(reader)? as usize;
                Ok(Value::Function(idx))
            }
            x if x == ValueType::Closure as u8 => {
                let func_idx = Self::read_u16(reader)? as usize;
                let cap_count = Self::read_u16(reader)? as usize;
                let mut captures = Vec::with_capacity(cap_count);
                for _ in 0..cap_count {
                    captures.push(Self::read_value(reader)?);
                }
                Ok(Value::ClosureVal(Box::new(crate::bytecode::Closure::new(func_idx, captures))))
            }
            x if x == ValueType::Module as u8 => {
                let idx = Self::read_u16(reader)? as usize;
                Ok(Value::Module(idx))
            }
            x => Err(SerializeError::UnknownValueType(x)),
        }
    }
    
    fn read_function<R: Read>(reader: &mut R) -> Result<FunctionInfo, SerializeError> {
        let name = Self::read_string(reader)?;
        let mut arity = [0u8; 1];
        reader.read_exact(&mut arity)?;
        let arity = arity[0];
        
        let param_count = Self::read_u16(reader)? as usize;
        let mut params = Vec::with_capacity(param_count);
        for _ in 0..param_count {
            let param_name = Self::read_string(reader)?;
            let mut type_byte = [0u8; 1];
            reader.read_exact(&mut type_byte)?;
            let param_type: ValueType = unsafe { std::mem::transmute(type_byte[0]) };
            params.push((param_name, param_type));
        }
        
        let start = Self::read_u32(reader)? as usize;
        let length = Self::read_u32(reader)? as usize;
        
        Ok(FunctionInfo::new(name, arity, params, start, length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_serialize_deserialize() {
        let mut chunk = Chunk::new("test");
        chunk.constants.push(Value::Integer(42));
        chunk.constants.push(Value::String("hello".to_string()));
        chunk.strings.push("var_name".to_string());
        chunk.code = vec![0x01, 0x00, 0x00, 0xFF];
        chunk.lines = vec![1, 1, 1, 1];
        
        let mut buffer = Vec::new();
        BytecodeSerializer::serialize(&chunk, &mut buffer).unwrap();
        
        let mut cursor = std::io::Cursor::new(buffer);
        let loaded = BytecodeSerializer::deserialize(&mut cursor).unwrap();
        
        assert_eq!(loaded.name, "test");
        assert_eq!(loaded.constants.len(), 2);
        assert_eq!(loaded.strings.len(), 1);
        assert_eq!(loaded.code.len(), 4);
    }
}
