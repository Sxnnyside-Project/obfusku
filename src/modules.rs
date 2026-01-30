//! # Module System for Obfusku v1.0.0
//!
//! Provides module loading, symbol importing/exporting, and namespace management.
//! Modules are the ritual scrolls that can be invoked into other spells.

use crate::bytecode::{Chunk, Value};
use rustc_hash::FxHashMap;
use std::path::PathBuf;
use std::fs;
use thiserror::Error;

/// Module-related errors
#[derive(Error, Debug)]
pub enum ModuleError {
    #[error("📜 Module '{name}' not found in the archives")]
    ModuleNotFound { name: String },

    #[error("🔄 Circular dependency detected: {chain}")]
    CircularDependency { chain: String },

    #[error("📖 Failed to read module '{name}': {reason}")]
    ReadError { name: String, reason: String },

    #[error("⚠️ Symbol '{symbol}' not exported from module '{module}'")]
    SymbolNotExported { symbol: String, module: String },

    #[error("🔒 Module '{name}' is already loaded")]
    AlreadyLoaded { name: String },
}

/// A loaded module containing compiled bytecode and exports
#[derive(Debug, Clone)]
pub struct Module {
    /// Module name/path
    pub name: String,
    /// Compiled bytecode for this module
    pub chunk: Chunk,
    /// Exported symbol names
    pub exports: Vec<String>,
    /// Exported values (evaluated at load time)
    pub export_values: FxHashMap<String, Value>,
}

impl Module {
    pub fn new(name: String, chunk: Chunk) -> Self {
        Self {
            name,
            chunk,
            exports: Vec::new(),
            export_values: FxHashMap::default(),
        }
    }

    /// Mark a symbol as exported
    pub fn export(&mut self, name: String) {
        if !self.exports.contains(&name) {
            self.exports.push(name);
        }
    }

    /// Set an exported value
    pub fn set_export_value(&mut self, name: String, value: Value) {
        self.export_values.insert(name, value);
    }

    /// Get an exported value
    pub fn get_export(&self, name: &str) -> Option<&Value> {
        if self.exports.contains(&name.to_string()) {
            self.export_values.get(name)
        } else {
            None
        }
    }

    /// Check if symbol is exported
    pub fn is_exported(&self, name: &str) -> bool {
        self.exports.contains(&name.to_string())
    }
}

/// Module loader and resolver
#[derive(Debug)]
pub struct ModuleLoader {
    /// Search paths for modules
    search_paths: Vec<PathBuf>,
    /// Loaded modules by name
    loaded: FxHashMap<String, usize>,
    /// Module storage
    modules: Vec<Module>,
    /// Loading stack for circular dependency detection
    loading_stack: Vec<String>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            search_paths: vec![PathBuf::from(".")],
            loaded: FxHashMap::default(),
            modules: Vec::new(),
            loading_stack: Vec::new(),
        }
    }

    /// Add a search path for modules
    pub fn add_search_path(&mut self, path: impl Into<PathBuf>) {
        self.search_paths.push(path.into());
    }

    /// Resolve module name to file path
    fn resolve(&self, name: &str) -> Option<PathBuf> {
        // Try direct path first
        let direct = PathBuf::from(name);
        if direct.exists() {
            return Some(direct);
        }

        // Add .obk extension if missing
        let with_ext = if name.ends_with(".obk") || name.ends_with(".obx") {
            name.to_string()
        } else {
            format!("{}.obk", name)
        };

        // Search in paths
        for search_path in &self.search_paths {
            let full_path = search_path.join(&with_ext);
            if full_path.exists() {
                return Some(full_path);
            }

            // Also try .obx extension for library modules
            let obx_path = search_path.join(format!("{}.obx", name.trim_end_matches(".obk")));
            if obx_path.exists() {
                return Some(obx_path);
            }
        }

        None
    }

    /// Check if module is already loaded
    pub fn is_loaded(&self, name: &str) -> bool {
        self.loaded.contains_key(name)
    }

    /// Get loaded module index
    pub fn get_module_index(&self, name: &str) -> Option<usize> {
        self.loaded.get(name).copied()
    }

    /// Get module by index
    pub fn get_module(&self, index: usize) -> Option<&Module> {
        self.modules.get(index)
    }

    /// Get mutable module by index
    pub fn get_module_mut(&mut self, index: usize) -> Option<&mut Module> {
        self.modules.get_mut(index)
    }

    /// Read module source from file
    pub fn read_source(&self, name: &str) -> Result<String, ModuleError> {
        let path = self.resolve(name)
            .ok_or_else(|| ModuleError::ModuleNotFound { name: name.to_string() })?;

        fs::read_to_string(&path)
            .map_err(|e| ModuleError::ReadError {
                name: name.to_string(),
                reason: e.to_string()
            })
    }

    /// Begin loading a module (for circular dependency detection)
    pub fn begin_load(&mut self, name: &str) -> Result<(), ModuleError> {
        if self.loading_stack.contains(&name.to_string()) {
            let chain = self.loading_stack.join(" → ") + " → " + name;
            return Err(ModuleError::CircularDependency { chain });
        }
        self.loading_stack.push(name.to_string());
        Ok(())
    }

    /// Complete loading a module
    pub fn complete_load(&mut self, name: &str, module: Module) -> usize {
        let index = self.modules.len();
        self.modules.push(module);
        self.loaded.insert(name.to_string(), index);
        self.loading_stack.retain(|n| n != name);
        index
    }

    /// Cancel loading (on error)
    pub fn cancel_load(&mut self, name: &str) {
        self.loading_stack.retain(|n| n != name);
    }

    /// Get all loaded module names
    pub fn loaded_modules(&self) -> Vec<&String> {
        self.loaded.keys().collect()
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        let mut module = Module::new("test".to_string(), Chunk::new("test"));
        module.export("foo".to_string());
        module.set_export_value("foo".to_string(), Value::Integer(42));

        assert!(module.is_exported("foo"));
        assert!(!module.is_exported("bar"));
        assert_eq!(module.get_export("foo"), Some(&Value::Integer(42)));
    }

    #[test]
    fn test_circular_detection() {
        let mut loader = ModuleLoader::new();

        loader.begin_load("a").unwrap();
        loader.begin_load("b").unwrap();

        let result = loader.begin_load("a");
        assert!(matches!(result, Err(ModuleError::CircularDependency { .. })));
    }
}
