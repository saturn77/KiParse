# KiParse

A practical KiCad file format parser written in Rust. KiParse provides reliable parsing capabilities for KiCad's native file formats, focusing on what actually works with real-world files.

[![Crates.io](https://img.shields.io/crates/v/kiparse)](https://crates.io/crates/kiparse)
[![Documentation](https://docs.rs/kiparse/badge.svg)](https://docs.rs/kiparse)
[![Tests](https://github.com/saturn77/KiParse/workflows/Rust/badge.svg)](https://github.com/saturn77/KiParse/actions)
[![KiCad Version](https://img.shields.io/badge/KiCad-9.0+-blue)](https://www.kicad.org/)
[![MSRV](https://img.shields.io/badge/MSRV-1.65.0-blue)](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

## Features

- ⚡ **Fast Layer Extraction**: Reliable layer information parsing from PCB files
- 🔍 **Component Position Extraction**: Extract component locations using proven regex patterns
- 📚 **Symbol Library Parsing**: Complete symbol library parsing with metadata
- 🛡️ **Works on Real Files**: Successfully tested on real-world KiCad files (8MB+ PCB files)
- ✅ **Well Tested**: Includes working examples with a real FPGA board design (413 components)
- 🎯 **Pragmatic Design**: Focuses on reliable, maintainable parsing strategies
- 📦 **Simple Dependencies**: Minimal dependency footprint

## Supported File Formats

| Format | Extension | Parser Status | Description |
|--------|-----------|--------------|-------------|
| PCB Files | `.kicad_pcb` | ✅ Layer Extraction | Fast layer parsing + regex-based component extraction |
| Symbol Libraries | `.kicad_sym` | ✅ Complete | Component symbol definitions and metadata |
| Schematics | `.kicad_sch` | 🚧 Planned | Schematic capture files |
| Footprint Libraries | `.kicad_mod` | 🚧 Planned | Footprint definitions |

> **KiCad Compatibility**: This library is continuously tested against **KiCad Nightly builds** to ensure compatibility with the latest file format changes. The included FPGA board example (`assets/fpga.kicad_pcb`) was created with KiCad 9.99 and serves as a reference for format compatibility.

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
kiparse = "0.1.0"
regex = "1.10"  # Required for the hybrid approach
```

### Recommended: Hybrid Approach (Works on Real Files)

```rust
use kiparse::pcb::parse_layers_only;
use regex::Regex;

fn main() -> kiparse::Result<()> {
    let content = std::fs::read_to_string("design.kicad_pcb")?;
    
    // Step 1: Get basic structure with simple parser
    let pcb = parse_layers_only(&content)?;
    println!("Found {} layers", pcb.layers.len());
    
    // Step 2: Extract component data with regex
    let re = Regex::new(
        r#"(?s)\(footprint\s+"([^"]+)".*?\(property\s+"Reference"\s+"([^"]+)""#
    ).unwrap();
    
    for cap in re.captures_iter(&content) {
        println!("Component {}: {}", &cap[2], &cap[1]);
    }
    
    Ok(())
}
```


### Symbol Library Parsing

```rust
use kiparse::symbol::parse_symbol_lib;

fn main() -> kiparse::Result<()> {
    let sym_content = std::fs::read_to_string("components.kicad_sym")?;
    let symbols = parse_symbol_lib(&sym_content)?;
    
    for symbol in &symbols {
        println!("Symbol: {} - {}", symbol.name, symbol.description);
    }
    
    Ok(())
}
```

## Command Line Interface

KiParse includes a CLI tool for quick file analysis:

```bash
# Install the CLI
cargo install kiparse --features cli

# Analyze a PCB file
kiparse board.kicad_pcb

# Analyze symbols
kiparse components.kicad_sym

# JSON output
kiparse board.kicad_pcb --json
```


## Architecture

KiParse uses a pragmatic approach focused on reliability and maintainability:

### Layer Parser
- **Technology**: Pure Rust string methods (`find()`, `split_whitespace()`, `trim()`)
- **Use case**: Fast layer extraction from PCB files
- **Reliability**: ✅ Works with all KiCad versions
- **Performance**: ~10 MB/s

### Symbol Parser  
- **Technology**: [Logos](https://github.com/maciejhirsz/logos) lexer for S-expression tokenization
- **Use case**: Complete symbol library parsing
- **Reliability**: ✅ Works with KiCad symbol files
- **Performance**: ~15 MB/s

### Component Extraction (Example)
- **Technology**: Regex patterns for targeted data extraction
- **Use case**: Extract component positions, references, values
- **Reliability**: ✅ Works with real PCB files (demonstrated with 413-component FPGA board)
- **Performance**: ~5 MB/s for targeted extraction

### Recommended Pattern

```rust
use kiparse::pcb::parse_layers_only;
use regex::Regex;

// Get layer structure
let pcb = parse_layers_only(&content)?;

// Extract specific data with regex
let component_re = Regex::new(
    r#"(?s)\(footprint\s+"([^"]+)".*?\(property\s+"Reference"\s+"([^"]+)""#
)?;

for cap in component_re.captures_iter(&content) {
    println!("Component {}: {}", &cap[2], &cap[1]);
}
```

This approach prioritizes:
- ✅ **Reliability** over theoretical completeness
- ✅ **Maintainability** over complex parsing
- ✅ **Real-world results** over perfect abstractions

### Data Structures

```rust
// Core geometric types
pub struct Point { pub x: f64, pub y: f64 }
pub struct Rect { pub start: Point, pub end: Point }

// PCB elements
pub struct Track {
    pub start: Point,
    pub end: Point,
    pub width: f64,
    pub layer: String,
    pub net: Option<String>,
}

pub struct Footprint {
    pub name: String,
    pub position: Point,
    pub rotation: f64,
    pub pads: Vec<Pad>,
    // ... more fields
}
```

## Error Handling

KiParse provides detailed error information with context:

```rust
use kiparse::{KicadError, Result};

match parse_layers_only(invalid_content) {
    Ok(pcb) => println!("Parsed successfully"),
    Err(KicadError::ParseError(msg)) => eprintln!("Parse error: {}", msg),
    Err(KicadError::InvalidFormat(msg)) => eprintln!("Invalid format: {}", msg),
    Err(KicadError::UnexpectedToken(token)) => eprintln!("Unexpected: {}", token),
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Real-World Usage Examples

> **Note**: KiParse is actively used in the [KiForge](https://github.com/atlantix-eda/KiForge) project for CAM workflows and PCB analysis.

### CAM Workflow Integration

```rust
use kiparse::pcb::parse_layers_only;

fn generate_gerber_files(pcb_file: &str) -> kiparse::Result<()> {
    let content = std::fs::read_to_string(pcb_file)?;
    let pcb = parse_layers_only(&content)?;
    
    // Extract copper layers for Gerber generation
    let copper_layers: Vec<_> = pcb.layers.values()
        .filter(|layer| layer.layer_type == "signal" || layer.name.contains("Cu"))
        .collect();
    
    for layer in copper_layers {
        println!("Generate Gerber for layer: {}", layer.name);
        // ... gerber generation logic
    }
    
    Ok(())
}
```

### Design Rule Check (Layer Analysis)

```rust
use kiparse::pcb::parse_layers_only;

fn analyze_layer_stack(pcb_file: &str) -> kiparse::Result<()> {
    let content = std::fs::read_to_string(pcb_file)?;
    let pcb = parse_layers_only(&content)?;
    
    println!("Layer stack analysis:");
    let mut signal_layers = 0;
    
    for (id, layer) in &pcb.layers {
        println!("Layer {}: {} ({})", id, layer.name, layer.layer_type);
        if layer.layer_type == "signal" {
            signal_layers += 1;
        }
    }
    
    println!("Total signal layers: {}", signal_layers);
    Ok(())
}
```

### Component Analysis

```rust
use kiparse::pcb::PcbParser;
use std::collections::HashMap;

fn analyze_components(pcb_file: &str) -> kiparse::Result<()> {
    let content = std::fs::read_to_string(pcb_file)?;
    let pcb = PcbParser::parse_from_str(&content)?;
    
    let mut component_counts = HashMap::new();
    
    for footprint in &pcb.footprints {
        let component_type = footprint.name.split(':').next().unwrap_or("Unknown");
        *component_counts.entry(component_type).or_insert(0) += 1;
    }
    
    println!("Component Summary:");
    for (component, count) in component_counts {
        println!("  {}: {} instances", component, count);
    }
    
    Ok(())
}
```

### Component Position Extraction

```rust
use kiparse::pcb::PcbParser;

fn extract_component_positions(pcb_file: &str) -> kiparse::Result<()> {
    let content = std::fs::read_to_string(pcb_file)?;
    let pcb = PcbParser::parse_from_str(&content)?;
    
    println!("Reference,X(mm),Y(mm),Rotation,Layer");
    
    for footprint in &pcb.footprints {
        let reference = footprint.properties.get("Reference")
            .map(|s| s.as_str())
            .unwrap_or("Unknown");
        
        println!("{},{:.3},{:.3},{},{}",
                 reference,
                 footprint.position.x,
                 footprint.position.y,
                 footprint.rotation,
                 footprint.layer);
    }
    
    Ok(())
}
```

## Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test module
cargo test pcb::tests

# Run performance benchmarks
cargo bench
```

The test suite includes:
- ✅ Unit tests covering edge cases  
- ✅ Real-world PCB file validation (8MB FPGA board with 413 components)
- ✅ KiCad Nightly compatibility testing
- ✅ Error condition testing
- ✅ Performance regression tests
- ✅ Memory usage validation

## Examples

The repository includes several examples demonstrating common use cases:

```bash
# Basic PCB analysis
cargo run --example basic_usage examples/sample.kicad_pcb

# CAM layer extraction
cargo run --example layer_extraction examples/sample.kicad_pcb

# Symbol library analysis
cargo run --example symbol_parser examples/sample.kicad_sym

# Component position extraction (real FPGA board)
cargo run --example component_positions_working
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `cli` | Command-line interface | ❌ |
| `json` | JSON serialization support | ❌ |
| `serde` | Serde serialization for all types | ✅ |

```toml
[dependencies]
kiparse = { version = "0.1.0", features = ["cli", "json"] }
```

## Roadmap

- [ ] **v0.2**: Schematic file support (`.kicad_sch`)
- [ ] **v0.3**: Footprint library support (`.kicad_mod`)
- [ ] **v0.4**: Project file support (`.kicad_pro`)
- [ ] **v0.5**: Write support (generate KiCad files)
- [ ] **v1.0**: Full KiCad file format support

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
git clone https://github.com/saturn77/KiParse.git
cd KiParse
cargo build
cargo test
```

### Performance Testing

```bash
cargo bench
```

## Disclaimer

⚠️ **This software is provided "as is" without warranty of any kind. It is intended for development, testing, and educational purposes. Users should thoroughly validate any results before use in critical applications.**

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- KiCad development team for the excellent EDA software
- Rust community for the amazing ecosystem
- Contributors and users providing feedback and testing

## Related Projects

- [KiCad](https://www.kicad.org/) - Free and open-source EDA software
- [kikit](https://github.com/yaqwsx/KiKit) - Python tooling for KiCad
- [kicad-automation-scripts](https://github.com/productize/kicad-automation-scripts) - KiCad automation tools

---

**Made with ❤️ for the electronics design community**