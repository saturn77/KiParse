# KiParse

A comprehensive, high-performance KiCad file format parser written in Rust. KiParse provides robust parsing capabilities for KiCad's native file formats with a focus on performance, accuracy, and ease of use.

[![Crates.io](https://img.shields.io/crates/v/kiparse)](https://crates.io/crates/kiparse)
[![Documentation](https://docs.rs/kiparse/badge.svg)](https://docs.rs/kiparse)
[![Tests](https://github.com/saturn77/KiParse/workflows/Rust/badge.svg)](https://github.com/saturn77/KiParse/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.65.0-blue)](https://blog.rust-lang.org/2022/11/03/Rust-1.65.0.html)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)

## Features

- 🚀 **Dual Parser Architecture**: Fast layer-only parsing and complete S-expression parsing
- 🔍 **Comprehensive Type System**: Strongly typed data structures for all KiCad elements
- 🛡️ **Robust Error Handling**: Detailed error messages with parsing context
- ⚡ **Memory Efficient**: Optimized for large PCB files (~2x file size memory usage)
- ✅ **Well Tested**: 20+ unit tests with edge case validation and real-world examples
- 📦 **Zero-Copy Parsing**: Minimal memory allocations during parsing
- 🎯 **Actively Developed**: Suitable for development and testing workflows

## Supported File Formats

| Format | Extension | Parser Status | Description |
|--------|-----------|--------------|-------------|
| PCB Files | `.kicad_pcb` | ✅ Complete | Full board layout with tracks, footprints, layers |
| Symbol Libraries | `.kicad_sym` | ✅ Complete | Component symbol definitions and metadata |
| Schematics | `.kicad_sch` | 🚧 Planned | Schematic capture files |
| Footprint Libraries | `.kicad_mod` | 🚧 Planned | Footprint definitions |

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
kiparse = "0.1"
```

### Basic PCB Parsing

```rust
use kiparse::pcb::parse_layers_only;

fn main() -> kiparse::Result<()> {
    let pcb_content = std::fs::read_to_string("design.kicad_pcb")?;
    let pcb = parse_layers_only(&pcb_content)?;
    
    println!("Found {} layers", pcb.layers.len());
    for (id, layer) in &pcb.layers {
        println!("Layer {}: {} ({})", id, layer.name, layer.layer_type);
    }
    
    Ok(())
}
```

### Full PCB Analysis

```rust
use kiparse::pcb::PcbParser;

fn main() -> kiparse::Result<()> {
    let pcb_content = std::fs::read_to_string("design.kicad_pcb")?;
    let pcb = PcbParser::parse_from_str(&pcb_content)?;
    
    println!("PCB Statistics:");
    println!("  Tracks: {}", pcb.tracks.len());
    println!("  Footprints: {}", pcb.footprints.len());
    println!("  Vias: {}", pcb.vias.len());
    
    // Analyze tracks by layer
    for track in &pcb.tracks {
        println!("Track on {}: {:.2}mm wide", track.layer, track.width);
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

## Performance Characteristics

KiParse is designed for efficient parsing with large PCB files:

| Operation | Performance | Memory Usage |
|-----------|-------------|--------------|
| Simple layer parsing | ~10 MB/s | 1.5x file size |
| Full PCB parsing | ~2 MB/s | 2x file size |
| Symbol parsing | ~15 MB/s | 1.2x file size |

*Benchmarks on Intel i7-10750H with 16GB RAM*

## Architecture

### Parser Types

**Simple Parser** (`parse_layers_only`)
- Fast layer extraction for CAM workflows
- Minimal memory usage
- ~5x faster than full parsing
- Use for: Gerber generation, layer validation

**Full Parser** (`PcbParser::parse_from_str`)
- Complete S-expression parsing
- Full semantic analysis
- Rich data structures
- Use for: DRC analysis, component placement, routing analysis

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

### Design Rule Check (DRC)

```rust
use kiparse::pcb::PcbParser;

fn check_trace_widths(pcb_file: &str, min_width: f64) -> kiparse::Result<Vec<String>> {
    let content = std::fs::read_to_string(pcb_file)?;
    let pcb = PcbParser::parse_from_str(&content)?;
    
    let violations: Vec<String> = pcb.tracks
        .iter()
        .filter(|track| track.width < min_width)
        .map(|track| format!("Track on {} is {:.3}mm (min: {:.3}mm)", 
                           track.layer, track.width, min_width))
        .collect();
    
    Ok(violations)
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
- ✅ 20+ unit tests covering edge cases
- ✅ Real-world PCB file validation
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
```

## Feature Flags

| Feature | Description | Default |
|---------|-------------|---------|
| `cli` | Command-line interface | ❌ |
| `json` | JSON serialization support | ❌ |
| `serde` | Serde serialization for all types | ✅ |

```toml
[dependencies]
kiparse = { version = "0.1", features = ["cli", "json"] }
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