use kiparse::{pcb, symbol, Result};
use prettytable::{Table, row};
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <file.kicad_pcb|file.kicad_sym> [--json]", args[0]);
        eprintln!("       {} --version", args[0]);
        std::process::exit(1);
    }
    
    if args[1] == "--version" {
        println!("kiparse v{}", kiparse::VERSION);
        return Ok(());
    }
    
    let filename = &args[1];
    let json_output = args.get(2).map_or(false, |arg| arg == "--json");
    
    let content = fs::read_to_string(filename)?;
    
    if filename.ends_with(".kicad_pcb") {
        handle_pcb_file(&content, json_output)?;
    } else if filename.ends_with(".kicad_sym") {
        handle_symbol_file(&content, json_output)?;
    } else {
        eprintln!("Unsupported file type. Use .kicad_pcb or .kicad_sym files.");
        std::process::exit(1);
    }
    
    Ok(())
}

fn handle_pcb_file(content: &str, json_output: bool) -> Result<()> {
    let pcb = pcb::parse_layers_only(content)?;
    
    if json_output {
        #[cfg(feature = "json")]
        {
            println!("{}", serde_json::to_string_pretty(&pcb).unwrap());
        }
        #[cfg(not(feature = "json"))]
        {
            eprintln!("JSON output requires the 'json' feature to be enabled");
            std::process::exit(1);
        }
    } else {
        println!("PCB Information:");
        println!("================");
        println!("Version: {}", pcb.version);
        println!("Generator: {}", pcb.generator);
        println!("Layers: {}", pcb.layers.len());
        
        if !pcb.layers.is_empty() {
            println!("\nLayer Details:");
            let mut table = Table::new();
            table.add_row(row!["ID", "Name", "Type", "User Name"]);
            
            let mut sorted_layers: Vec<_> = pcb.layers.iter().collect();
            sorted_layers.sort_by_key(|(id, _)| *id);
            
            for (id, layer) in sorted_layers {
                table.add_row(row![
                    id, 
                    layer.name, 
                    layer.layer_type,
                    layer.user_name.as_deref().unwrap_or("-")
                ]);
            }
            table.printstd();
        }
    }
    
    Ok(())
}

fn handle_symbol_file(content: &str, json_output: bool) -> Result<()> {
    let symbols = symbol::symbol_parser::parse_symbol_lib(content)?;
    
    if json_output {
        #[cfg(feature = "json")]
        {
            println!("{}", serde_json::to_string_pretty(&symbols).unwrap());
        }
        #[cfg(not(feature = "json"))]
        {
            eprintln!("JSON output requires the 'json' feature to be enabled");
            std::process::exit(1);
        }
    } else {
        println!("Symbol Library Information:");
        println!("==========================");
        println!("Total symbols: {}", symbols.len());
        
        if !symbols.is_empty() {
            println!("\nSymbol Details:");
            let mut table = Table::new();
            table.add_row(row!["Symbol", "Description"]);
            
            for symbol in &symbols {
                table.add_row(row![
                    symbol.name,
                    if symbol.description.is_empty() { 
                        "-".to_string() 
                    } else { 
                        symbol.description.clone() 
                    }
                ]);
            }
            table.printstd();
        }
    }
    
    Ok(())
}