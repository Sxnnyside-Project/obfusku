//! # Obfusku - The Magical Programming Language
//!
//! A symbol-driven esoteric language implemented in Rust.
//!
//! ## Overview
//!
//! Obfusku is an esoteric programming language where symbols carry semantic weight.
//! Visual and symbolic meaning matters - execution feels ritualistic and abstract.
//!
//! ## Architecture
//!
//! - **Lexer**: Unicode-aware tokenization with symbol priority
//! - **Compiler**: Single-pass compilation to bytecode
//! - **VM**: Stack-based virtual machine with symbolic execution
//! - **Symbol Table**: Central registry of all symbol meanings
//!
//! ## Example
//!
//! ```obfusku
//! // Declare an integer variable
//! ⟁greeting_count=5
//!
//! // Print a greeting
//! ✤"Welcome to Obfusku!"
//!
//! // Output the count
//! ⚡[greeting_count]
//!
//! // End the spell
//! ❧
//! ```

pub mod symbols;
pub mod lexer;
pub mod bytecode;
pub mod vm;
pub mod compiler;
pub mod source_map;
pub mod optimizer;
pub mod modules;
pub mod serialize;

use clap::{Parser, Subcommand};
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::compiler::Compiler;
use crate::symbols::SymbolTable;
use crate::vm::Runtime;
use crate::serialize::BytecodeSerializer;

/// Obfusku - The Magical Programming Language
#[derive(Parser)]
#[command(name = "obfusku")]
#[command(author = "Sxnnyside Project")]
#[command(version = "0.3.0")]
#[command(about = "A symbol-driven esoteric programming language", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Execute an Obfusku spell (.obk file)
    Run {
        /// Path to the .obk file to execute
        file: PathBuf,

        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },

    /// Compile a spell and show the bytecode
    Compile {
        /// Path to the .obk file to compile
        file: PathBuf,

        /// Show disassembled bytecode
        #[arg(short, long)]
        disassemble: bool,
        
        /// Save compiled bytecode to file
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Load and run compiled bytecode (.obc file)
    Load {
        /// Path to the .obc bytecode file
        file: PathBuf,

        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },

    /// Run Obfusku in interactive REPL mode
    Repl {
        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },

    /// Show information about Obfusku symbols
    Symbols {
        /// Filter by category (type, operator, control, io, special)
        #[arg(short, long)]
        category: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { file, debug } => {
            if let Err(e) = run_file(&file, debug) {
                print_error(&e.to_string());
                std::process::exit(1);
            }
        }

        Commands::Compile { file, disassemble, output } => {
            if let Err(e) = compile_file(&file, disassemble, output) {
                print_error(&e.to_string());
                std::process::exit(1);
            }
        }

        Commands::Load { file, debug } => {
            if let Err(e) = load_file(&file, debug) {
                print_error(&e.to_string());
                std::process::exit(1);
            }
        }

        Commands::Repl { debug } => {
            if let Err(e) = run_repl(debug) {
                print_error(&e.to_string());
                std::process::exit(1);
            }
        }

        Commands::Symbols { category } => {
            show_symbols(category);
        }
    }
}

/// Run an Obfusku file
fn run_file(path: &PathBuf, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Check file extension
    if path.extension().map(|e| e.to_str()) != Some(Some("obk")) {
        print_warning("File does not have .obk extension - proceeding anyway");
    }

    // Read the source
    let source = fs::read_to_string(path)?;

    print_header("🔮 Casting spell...");

    // Compile
    let symbol_table = SymbolTable::new();
    let mut compiler = Compiler::new(&symbol_table);
    let chunk = compiler.compile(&source)?;

    if debug {
        println!("{}", chunk.disassemble());
    }

    // Execute
    let mut runtime = Runtime::new();
    runtime.set_debug(debug);
    runtime.execute(chunk)?;

    print_success("✨ Spell complete!");

    Ok(())
}

/// Compile a file and optionally show bytecode
fn compile_file(path: &PathBuf, disassemble: bool, output: Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;

    print_header("📜 Compiling spell...");

    let symbol_table = SymbolTable::new();
    let mut compiler = Compiler::new(&symbol_table);
    let chunk = compiler.compile(&source)?;

    print_success(&format!("✅ Compilation successful! ({} bytes of bytecode)", chunk.code.len()));

    // Save bytecode if output path provided
    if let Some(output_path) = output {
        BytecodeSerializer::save_to_file(&chunk, &output_path)?;
        print_success(&format!("💾 Bytecode saved to: {}", output_path.display()));
    }

    if disassemble {
        println!("\n{}", "═".repeat(50).dimmed());
        println!("{}", chunk.disassemble());
    }

    Ok(())
}

/// Load and execute compiled bytecode
fn load_file(path: &PathBuf, debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    // Check file extension
    if path.extension().map(|e| e.to_str()) != Some(Some("obc")) {
        print_warning("File does not have .obc extension - proceeding anyway");
    }

    print_header("📜 Loading compiled spell...");

    // Load bytecode
    let chunk = BytecodeSerializer::load_from_file(path)?;

    print_success(&format!("✅ Loaded bytecode: {} ({} bytes)", chunk.name, chunk.code.len()));

    if debug {
        println!("{}", chunk.disassemble());
    }

    print_header("🔮 Casting spell...");

    // Execute
    let mut runtime = Runtime::new();
    runtime.set_debug(debug);
    runtime.execute(chunk)?;

    print_success("✨ Spell complete!");

    Ok(())
}

/// Run the interactive REPL
fn run_repl(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    print_header("🌙 Obfusku Interactive Grimoire v0.3.0");
    println!("{}", "Type your spells. End with ❧ to execute.".dimmed());
    println!("{}", "Commands: :help, :symbols, :history, :clear, :quit".dimmed());
    println!();

    let symbol_table = SymbolTable::new();
    let mut runtime = Runtime::new();
    let mut debug_mode = debug;
    runtime.set_debug(debug_mode);
    
    // Command history
    let mut history: Vec<String> = Vec::new();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{} ", "⚗️ ".cyan());
        stdout.flush()?;

        let mut input = String::new();

        // Read until we see ❧ or a command
        loop {
            let mut line = String::new();
            if stdin.read_line(&mut line)? == 0 {
                println!();
                return Ok(());
            }

            let trimmed = line.trim();

            // Check for commands
            if trimmed.starts_with(':') {
                match trimmed {
                    ":quit" | ":q" => {
                        print_success("🌟 May your spells always compile!");
                        return Ok(());
                    }
                    ":help" | ":h" => {
                        print_repl_help();
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    ":symbols" | ":s" => {
                        show_symbols(None);
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    ":debug" | ":d" => {
                        debug_mode = !debug_mode;
                        runtime.set_debug(debug_mode);
                        println!("{}", format!("Debug mode: {}", if debug_mode { "ON" } else { "OFF" }).yellow());
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    ":history" | ":hist" => {
                        println!("\n{}", "═══ Spell History ═══".cyan());
                        for (i, spell) in history.iter().enumerate() {
                            let preview: String = spell.chars().take(40).collect();
                            let preview = preview.replace('\n', " ");
                            println!("  {} {}", format!("[{}]", i + 1).dimmed(), preview);
                        }
                        println!();
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    ":clear" | ":c" => {
                        // Clear screen (ANSI escape)
                        print!("\x1B[2J\x1B[1;1H");
                        print_header("🌙 Obfusku Interactive Grimoire v1.0.0");
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    ":reset" | ":r" => {
                        runtime = Runtime::new();
                        runtime.set_debug(debug_mode);
                        println!("{}", "Runtime state reset".yellow());
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    cmd if cmd.starts_with(":!") => {
                        // Recall from history
                        if let Ok(num) = cmd[2..].trim().parse::<usize>() {
                            if num > 0 && num <= history.len() {
                                input = history[num - 1].clone();
                                println!("{}", format!("Recalling spell #{}", num).dimmed());
                                break; // Execute it
                            }
                        }
                        println!("{}", "Invalid history number".red());
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                    _ => {
                        println!("{}", format!("Unknown command: {}", trimmed).red());
                        input.clear();
                        print!("{} ", "⚗️ ".cyan());
                        stdout.flush()?;
                        continue;
                    }
                }
            }

            input.push_str(&line);

            // Check if we have a complete spell (ends with ❧)
            if input.contains('❧') {
                break;
            }

            // Continuation prompt
            print!("{} ", "   ".dimmed());
            stdout.flush()?;
        }
        
        // Add to history
        if !input.trim().is_empty() {
            history.push(input.clone());
            // Keep last 50 entries
            if history.len() > 50 {
                history.remove(0);
            }
        }

        // Compile and execute
        let mut compiler = Compiler::new(&symbol_table);
        match compiler.compile(&input) {
            Ok(chunk) => {
                if debug_mode {
                    println!("{}", chunk.disassemble());
                }

                if let Err(e) = runtime.execute(chunk) {
                    print_error(&format!("Runtime error: {}", e));
                }
            }
            Err(e) => {
                print_error(&format!("Compilation error: {}", e));
            }
        }

        // Reset runtime for next spell (in REPL we want fresh state)
        runtime = Runtime::new();
        runtime.set_debug(debug_mode);

        println!();
    }
}

/// Show information about symbols
fn show_symbols(category_filter: Option<String>) {
    use crate::symbols::SymbolCategory;

    let table = SymbolTable::new();

    println!();
    println!("{}", "═══════════════════════════════════════════════════════".cyan());
    println!("{}", "            📚 OBFUSKU SYMBOL GRIMOIRE                  ".cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════".cyan());

    let categories = if let Some(ref filter) = category_filter {
        match filter.to_lowercase().as_str() {
            "type" | "types" => vec![SymbolCategory::TypeDeclaration],
            "operator" | "operators" | "op" => vec![SymbolCategory::Operator],
            "control" | "flow" => vec![SymbolCategory::ControlFlow],
            "io" | "input" | "output" => vec![SymbolCategory::InputOutput],
            "special" => vec![SymbolCategory::SpecialValue],
            "comparison" | "cmp" => vec![SymbolCategory::Comparison],
            "logical" | "logic" => vec![SymbolCategory::Logical],
            "delimiter" | "delim" => vec![SymbolCategory::Delimiter],
            _ => {
                println!("{}", format!("Unknown category: {}", filter).red());
                println!("Available: type, operator, control, io, special, comparison, logical, delimiter");
                return;
            }
        }
    } else {
        vec![
            SymbolCategory::TypeDeclaration,
            SymbolCategory::SpecialValue,
            SymbolCategory::Operator,
            SymbolCategory::Comparison,
            SymbolCategory::Logical,
            SymbolCategory::InputOutput,
            SymbolCategory::ControlFlow,
            SymbolCategory::Delimiter,
        ]
    };

    for category in categories {
        let symbols = table.symbols_in_category(category);
        if symbols.is_empty() {
            continue;
        }

        println!();
        println!("{}", format!("  {:?}", category).yellow().bold());
        println!("{}", "  ─────────────────────────────────────".dimmed());

        for symbol in symbols {
            println!(
                "    {}  {}",
                symbol.glyph.cyan().bold(),
                symbol.description.dimmed()
            );
        }
    }

    println!();
    println!("{}", "═══════════════════════════════════════════════════════".cyan());
}

fn print_repl_help() {
    println!();
    println!("{}", "═══════════════════════════════════════════════════════".cyan());
    println!("{}", "                    🌙 REPL HELP                       ".cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════".cyan());
    println!();
    println!("{}", "Commands:".yellow());
    println!("  {}  - Show this help", ":help, :h".green());
    println!("  {}  - List all symbols", ":symbols, :s".green());
    println!("  {}  - Toggle debug mode", ":debug, :d".green());
    println!("  {}  - Show spell history", ":history, :hist".green());
    println!("  {}  - Recall spell #N from history", ":!N".green());
    println!("  {}  - Clear screen", ":clear, :c".green());
    println!("  {}  - Reset runtime state", ":reset, :r".green());
    println!("  {}  - Exit the REPL", ":quit, :q".green());
    println!();
    println!("{}", "Usage:".yellow());
    println!("{}", "  Type your spell and end it with ❧ to execute.".dimmed());
    println!("{}", "  Multi-line spells are supported.".dimmed());
    println!();
    println!("{}", "═══════════════════════════════════════════════════════".cyan());
}

fn print_header(msg: &str) {
    println!("{}", msg.cyan().bold());
}

fn print_success(msg: &str) {
    println!("{}", msg.green());
}

fn print_warning(msg: &str) {
    eprintln!("{}", format!("⚠️  {}", msg).yellow());
}

fn print_error(msg: &str) {
    eprintln!("{}", format!("❌ {}", msg).red());
}
