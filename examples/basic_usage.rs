//! Basic usage example for KiParse
//! 
//! This example demonstrates how to parse a KiCad PCB file and extract
//! basic information about layers, tracks, and components.

use kiparse::{pcb, Result};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        eprintln!("Usage: {} <file.kicad_pcb>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    let content = std::fs::read_to_string(filename)?;
    
    // Simple parsing for quick layer extraction
    println!("=== Simple Layer Parsing ===");
    let pcb_simple = pcb::parse_layers_only(&content)?;
    println!("Version: {}", pcb_simple.version);
    println!("Generator: {}", pcb_simple.generator);
    println!("Found {} layers:", pcb_simple.layers.len());
    
    for (id, layer) in &pcb_simple.layers {
        println!("  Layer {}: {} ({})", id, layer.name, layer.layer_type);
    }
    
    // Full parsing for detailed analysis
    println!("\n=== Full PCB Parsing ===");
    let pcb_full = pcb::PcbParser::parse_from_str(&content)?;
    
    println!("Detailed statistics:");
    println!("  Tracks: {}", pcb_full.tracks.len());
    println!("  Footprints: {}", pcb_full.footprints.len());
    println!("  Vias: {}", pcb_full.vias.len());
    println!("  Zones: {}", pcb_full.zones.len());
    println!("  Graphics: {}", pcb_full.graphics.len());
    println!("  Texts: {}", pcb_full.texts.len());
    
    // Analyze tracks by layer
    if !pcb_full.tracks.is_empty() {
        println!("\n=== Track Analysis ===");
        let mut layer_track_count = std::collections::HashMap::new();
        
        for track in &pcb_full.tracks {
            *layer_track_count.entry(&track.layer).or_insert(0) += 1;
        }
        
        for (layer, count) in layer_track_count {
            println!("  {}: {} tracks", layer, count);
        }
        
        // Find tracks with nets
        let tracks_with_nets = pcb_full.tracks.iter()
            .filter(|t| t.net.is_some())
            .count();
        println!("  Tracks with nets: {} / {}", tracks_with_nets, pcb_full.tracks.len());
    }
    
    // Analyze footprints
    if !pcb_full.footprints.is_empty() {
        println!("\n=== Footprint Analysis ===");
        let mut footprint_types = std::collections::HashMap::new();
        
        for footprint in &pcb_full.footprints {
            let base_name = footprint.name.split(':').next().unwrap_or(&footprint.name);
            *footprint_types.entry(base_name).or_insert(0) += 1;
        }
        
        for (footprint_type, count) in footprint_types {
            println!("  {}: {} instances", footprint_type, count);
        }
        
        // Count total pads
        let total_pads: usize = pcb_full.footprints.iter()
            .map(|f| f.pads.len())
            .sum();
        println!("  Total pads: {}", total_pads);
    }
    
    Ok(())
}