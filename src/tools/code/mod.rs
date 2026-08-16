//! Code analysis tools

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Code symbol information
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub path: PathBuf,
    pub line_number: usize,
    pub column: usize,
}

/// Kind of code symbol
#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
    Function,
    Class,
    Struct,
    Interface,
    Module,
    Variable,
    Constant,
    Type,
    Method,
    Field,
    Other,
}

impl SymbolKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            SymbolKind::Function => "function",
            SymbolKind::Class => "class",
            SymbolKind::Struct => "struct",
            SymbolKind::Interface => "interface",
            SymbolKind::Module => "module",
            SymbolKind::Variable => "variable",
            SymbolKind::Constant => "constant",
            SymbolKind::Type => "type",
            SymbolKind::Method => "method",
            SymbolKind::Field => "field",
            SymbolKind::Other => "other",
        }
    }
}

/// Analyze a source file for symbols (simplified)
pub fn analyze_symbols(path: &Path) -> Result<Vec<Symbol>> {
    let content = std::fs::read_to_string(path)?;
    let mut symbols = Vec::new();
    
    // Simple heuristic-based symbol detection
    // In production, this would use tree-sitter for proper parsing
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    
    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        
        // Detect functions (various languages)
        if trimmed.starts_with("fn ") 
            || trimmed.starts_with("function ")
            || trimmed.starts_with("def ")
            || (trimmed.contains("func ") && extension == "go")
        {
            if let Some(name) = extract_function_name(trimmed, extension) {
                symbols.push(Symbol {
                    name,
                    kind: SymbolKind::Function,
                    path: path.to_path_buf(),
                    line_number: line_num + 1,
                    column: 0,
                });
            }
        }
        
        // Detect classes/structs
        if trimmed.starts_with("class ")
            || trimmed.starts_with("struct ")
            || trimmed.starts_with("interface ")
        {
            if let Some(name) = extract_type_name(trimmed) {
                let kind = if trimmed.starts_with("class ") {
                    SymbolKind::Class
                } else if trimmed.starts_with("struct ") {
                    SymbolKind::Struct
                } else {
                    SymbolKind::Interface
                };
                
                symbols.push(Symbol {
                    name,
                    kind,
                    path: path.to_path_buf(),
                    line_number: line_num + 1,
                    column: 0,
                });
            }
        }
    }
    
    Ok(symbols)
}

fn extract_function_name(line: &str, extension: &str) -> Option<String> {
    // Very simplified extraction
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        let name_part = parts[1];
        // Remove parentheses and parameters
        let name = name_part.split('(').next()?.trim();
        Some(name.to_string())
    } else {
        None
    }
}

fn extract_type_name(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() >= 2 {
        Some(parts[1].to_string())
    } else {
        None
    }
}

/// Get the language of a file based on extension
pub fn detect_language(path: &Path) -> Option<&'static str> {
    path.extension().and_then(|ext| match ext.to_str()? {
        "rs" => Some("rust"),
        "py" => Some("python"),
        "js" | "mjs" => Some("javascript"),
        "ts" | "tsx" => Some("typescript"),
        "go" => Some("go"),
        "java" => Some("java"),
        "c" | "h" => Some("c"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("cpp"),
        "rb" => Some("ruby"),
        "php" => Some("php"),
        "cs" => Some("csharp"),
        "swift" => Some("swift"),
        "kt" => Some("kotlin"),
        "scala" => Some("scala"),
        "rs" => Some("rust"),
        _ => None,
    })
}

/// Count lines of code
pub fn count_lines(path: &Path) -> Result<LineCount> {
    let content = std::fs::read_to_string(path)?;
    let mut lines = LineCount::default();
    
    let mut in_multiline_comment = false;
    let mut in_multiline_string = false;
    
    for line in content.lines() {
        let trimmed = line.trim();
        
        // Track multiline comments
        if trimmed.starts_with("/*") && !trimmed.ends_with("*/") {
            in_multiline_comment = true;
        }
        if trimmed.contains("*/") {
            in_multiline_comment = false;
            continue;
        }
        
        // Skip empty lines and comments
        if trimmed.is_empty()
            || trimmed.starts_with("//")
            || trimmed.starts_with('#')
            || trimmed.starts_with("--")
            || in_multiline_comment
        {
            lines.blank += 1;
            continue;
        }
        
        lines.code += 1;
        
        // Count comments on same line
        if trimmed.contains("//") || trimmed.contains('#') {
            lines.comments += 1;
        }
    }
    
    lines.total = lines.code + lines.blank + lines.comments;
    Ok(lines)
}

/// Line count statistics
#[derive(Debug, Clone, Default)]
pub struct LineCount {
    pub total: usize,
    pub code: usize,
    pub comments: usize,
    pub blank: usize,
}

/// Code tools collection
pub struct CodeTools;

impl CodeTools {
    /// Analyze symbols in a file
    pub fn analyze(path: &Path) -> Result<Vec<Symbol>> {
        analyze_symbols(path)
    }
    
    /// Detect language
    pub fn language(path: &Path) -> Option<&'static str> {
        detect_language(path)
    }
    
    /// Count lines
    pub fn count_lines(path: &Path) -> Result<LineCount> {
        count_lines(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_detect_language() {
        assert_eq!(detect_language(Path::new("test.rs")), Some("rust"));
        assert_eq!(detect_language(Path::new("test.py")), Some("python"));
        assert_eq!(detect_language(Path::new("test.js")), Some("javascript"));
    }
    
    #[test]
    fn test_count_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.rs");
        std::fs::write(&file, "fn main() {\n    // comment\n    println!();\n}\n").unwrap();
        
        let count = count_lines(&file).unwrap();
        assert!(count.code > 0);
    }
}
