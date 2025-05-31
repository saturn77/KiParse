# KiParse Examples

This directory contains examples demonstrating how to use KiParse for various KiCad file parsing tasks.

## Quick Start

```bash
# Run any example with:
cargo run --example <example_name> [arguments]

# For examples that need features:
cargo run --example <example_name> --features <feature>
```

## Available Examples

### 📋 Basic Usage

**File**: `basic_usage.rs`  
**Purpose**: Demonstrates basic layer extraction and file analysis  
**Usage**: 
```bash
cargo run --example basic_usage <file.kicad_pcb>
cargo run --example basic_usage assets/fpga.kicad_pcb
```
**What it shows**:
- Layer parsing and categorization (signal vs other layers)
- Basic file statistics and complexity estimation
- Error handling patterns

---

### 🔍 Layer Extraction

**File**: `layer_extraction.rs`  
**Purpose**: Focused layer parsing with detailed layer information  
**Usage**:
```bash
cargo run --example layer_extraction <file.kicad_pcb>
```
**What it shows**:
- Detailed layer stack analysis
- Layer type categorization
- CAM workflow preparation

---

### 📚 Symbol Library Parsing

**File**: `symbol_parser.rs`  
**Purpose**: Parse KiCad symbol library files  
**Usage**:
```bash
cargo run --example symbol_parser <file.kicad_sym>
```
**What it shows**:
- Symbol library parsing
- Symbol metadata extraction
- Component library analysis

---

### 🎯 Component Position Extraction (Working)

**File**: `component_positions_working.rs`  
**Purpose**: **Real-world component position extraction** from actual PCB files  
**Usage**:
```bash
cargo run --example component_positions_working
# Uses assets/fpga.kicad_pcb automatically
```
**What it shows**:
- ✅ **Recommended approach** for component data extraction
- Hybrid parsing (simple parser + regex)
- Component grouping by type (R, C, U, etc.)
- Natural sorting of references (R1, R2, R10)
- Board statistics and density analysis
- **Real results**: 413 components from FPGA board

**Dependencies**: Requires `regex = "1.10"` in your Cargo.toml

---

### 🧪 Component Position Extraction (Simple)

**File**: `component_positions_simple.rs`  
**Purpose**: Alternative regex-only approach for component extraction  
**Usage**:
```bash
cargo run --example component_positions_simple
```
**What it shows**:
- Pure regex-based component extraction
- Simplified pattern matching
- Alternative to the hybrid approach

**Dependencies**: Requires `regex = "1.10"` in your Cargo.toml

## Test Files

The `assets/` directory contains real KiCad files for testing:
- **`fpga.kicad_pcb`**: Real FPGA development board (8MB+, 413 components)
- Perfect for testing component extraction and performance

## Common Patterns

### Layer Analysis
```rust
use kiparse::pcb::parse_layers_only;

let content = std::fs::read_to_string("design.kicad_pcb")?;
let pcb = parse_layers_only(&content)?;

for (id, layer) in &pcb.layers {
    println!("Layer {}: {} ({})", id, layer.name, layer.layer_type);
}
```

### Component Extraction (Recommended)
```rust
use kiparse::pcb::parse_layers_only;
use regex::Regex;

// Get basic structure
let pcb = parse_layers_only(&content)?;

// Extract components with regex
let component_re = Regex::new(
    r#"(?s)\(footprint\s+"([^"]+)".*?\(property\s+"Reference"\s+"([^"]+)""#
)?;

for cap in component_re.captures_iter(&content) {
    println!("Component {}: {}", &cap[2], &cap[1]);
}
```

### Symbol Library Analysis
```rust
use kiparse::symbol::parse_symbol_lib;

let content = std::fs::read_to_string("library.kicad_sym")?;
let symbols = parse_symbol_lib(&content)?;

for symbol in &symbols {
    println!("Symbol: {}", symbol.name);
}
```

## Performance Notes

- **Layer parsing**: ~10 MB/s, works with all KiCad versions
- **Component extraction**: ~5 MB/s for targeted regex patterns
- **Symbol parsing**: ~15 MB/s with complete metadata

## Dependencies

Most examples only need the base `kiparse` crate. Component extraction examples require:

```toml
[dependencies]
kiparse = "0.1.0"
regex = "1.10"  # For component_positions_* examples
```

## Best Practices

1. **Start with layer parsing** - Always works, gives you basic file structure
2. **Use regex for specific data** - More reliable than complex parsing
3. **Handle errors gracefully** - KiCad files can be large and complex
4. **Test with real files** - Use the provided FPGA board example

## Need Help?

- Check the main README.md for architecture details
- Look at KNOWN_ISSUES.md for current limitations
- Submit issues at: https://github.com/saturn77/KiParse/issues