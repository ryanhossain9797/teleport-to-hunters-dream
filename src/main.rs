//! A CLI tool to teleport to Lantern Teleport in Bloodborne save files
//!
//! Usage: lantern-teleport <save_file> [--location <LOCATION>]

use clap::Parser;
use std::array::TryFromSliceError;
use std::fs;
use std::path::PathBuf;

mod constants;
mod types;

use constants::{COORD_OFFSET_AFTER_PATTERN, COORD_PATTERN, LCED_MARKER};
use types::{LOCATIONS, Location, get_locations_by_region};

#[derive(Parser, Debug)]
#[command(
    name = "lantern-teleport",
    author,
    version,
    about = "A CLI tool to teleport to any Lantern in Bloodborne save files",
    long_about = "A CLI tool to teleport to any Lantern in Bloodborne save files.\n\n\
                  This tool is meant to be run on userdata0000, userdata0001, etc. files \
                  found in your Bloodborne save directory.\
                  userdata0000 is your first character, userdata0001 is your second character, and so on."
)]
struct Args {
    /// Path to the save file (e.g., userdata0000, userdata0001)
    save_file: PathBuf,

    /// Teleport to a specific location (supports fuzzy matching)
    #[arg(short, long)]
    location: Option<String>,

    /// List all available locations
    #[arg(short, long)]
    list: bool,
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("Failed to read file: {}", e))
}

fn find_lced_marker(bytes: &[u8]) -> Option<usize> {
    println!("DEBUG: Searching for LCED marker (0x4C, 0x43, 0x45, 0x44)...");
    for i in 0..(bytes.len().saturating_sub(4)) {
        if bytes[i..i + 4] == LCED_MARKER {
            println!("DEBUG: Found LCED marker at offset 0x{:X} ({})", i, i);
            return Some(i);
        }
    }
    println!("DEBUG: LCED marker not found");
    None
}

fn find_coordinates_offset(bytes: &[u8], lced_offset: usize) -> Option<usize> {
    println!(
        "DEBUG: Searching for coordinate pattern starting from offset {}...",
        lced_offset
    );
    for i in lced_offset..(bytes.len().saturating_sub(COORD_PATTERN.len())) {
        if bytes[i..i + COORD_PATTERN.len()] == COORD_PATTERN {
            let coord_offset = i + COORD_OFFSET_AFTER_PATTERN;
            println!(
                "DEBUG: Found coordinate pattern at offset 0x{:X} ({}), coordinates at offset 0x{:X} ({})",
                i, i, coord_offset, coord_offset
            );
            return Some(coord_offset);
        }
    }
    println!("DEBUG: Coordinate pattern not found after LCED marker");
    None
}

fn read_coordinates(bytes: &[u8], offset: usize) -> Result<(f32, f32, f32), TryFromSliceError> {
    let x_bytes: [u8; 4] = bytes[offset..offset + 4].try_into()?;
    let y_bytes: [u8; 4] = bytes[offset + 4..offset + 8].try_into()?;
    let z_bytes: [u8; 4] = bytes[offset + 8..offset + 12].try_into()?;

    let x = f32::from_le_bytes(x_bytes);
    let y = f32::from_le_bytes(y_bytes);
    let z = f32::from_le_bytes(z_bytes);

    Ok((x, y, z))
}

fn write_coordinates(bytes: &mut [u8], offset: usize, x: f32, y: f32, z: f32) {
    println!(
        "DEBUG: Writing coordinates at offset {}: X={}, Y={}, Z={}",
        offset, x, y, z
    );
    bytes[offset..offset + 4].copy_from_slice(&f32::to_le_bytes(x));
    bytes[offset + 4..offset + 8].copy_from_slice(&f32::to_le_bytes(y));
    bytes[offset + 8..offset + 12].copy_from_slice(&f32::to_le_bytes(z));
}

fn write_map_id(bytes: &mut [u8], map_id: [u8; 4]) {
    println!("DEBUG: Writing map ID at offset 0x04: {:?}", map_id);
    bytes[0x04] = map_id[0];
    bytes[0x05] = map_id[1];
    bytes[0x06] = map_id[2];
    bytes[0x07] = map_id[3];
}

fn find_location(query: &str) -> Option<&'static Location> {
    let lower_query = query.to_lowercase();

    // Try exact match (case-insensitive)
    for loc in &LOCATIONS {
        if loc.name.to_lowercase() == lower_query {
            return Some(loc);
        }
    }

    // Try partial match
    for loc in &LOCATIONS {
        if loc.name.to_lowercase().contains(&lower_query) {
            return Some(loc);
        }
    }

    None
}

fn list_locations() {
    println!("\nAvailable teleport locations:");
    println!("============================");

    for (region, locations) in get_locations_by_region() {
        println!("\n{}", region);
        println!("----------------------------");
        for location in locations {
            println!(
                "  - {} (X: {:.2}, Y: {:.2}, Z: {:.2})",
                location.name, location.x, location.y, location.z
            );
        }
    }

    println!("\n============================");
    println!(
        "Total: {} locations across {} regions",
        LOCATIONS.len(),
        get_locations_by_region().len()
    );
}

fn teleport_to_location(bytes: &mut [u8], coord_offset: usize, location: &Location) {
    println!("\nTeleporting to {}...", location.name);

    write_map_id(bytes, [0x00, 0x00, location.map_id[0], location.map_id[1]]);
    write_coordinates(bytes, coord_offset, location.x, location.y, location.z);
}

fn main() {
    let args = Args::parse();

    if args.list {
        list_locations();
        return;
    }

    let location = &args
        .location
        .map(|name| match find_location(&name) {
            Some(loc) => loc,
            None => {
                println!(
                    "Error: Unknown location '{}'\nUse --list to see available locations",
                    name
                );
                std::process::exit(1);
            }
        })
        .unwrap_or_else(|| &LOCATIONS[0]);

    println!(
        "DEBUG: Selected location: {} (X: {:.2}, Y: {:.2}, Z: {:.2})",
        location.name, location.x, location.y, location.z
    );

    println!("DEBUG: Reading file: {:?}", args.save_file);
    let Ok(mut bytes) = read_file(&args.save_file) else {
        println!("Error: Failed to read file");
        std::process::exit(1);
    };
    println!("DEBUG: File size: {} bytes", bytes.len());

    let Some(lced_offset) = find_lced_marker(&bytes) else {
        println!(
            "\nError: Could not find LCED (0x4C 0x43 0x45 0x44) marker in save file.\n\
            This may not be a valid decrypted Bloodborne save."
        );
        std::process::exit(1);
    };

    let Some(coord_offset) = find_coordinates_offset(&bytes, lced_offset) else {
        println!(
            "\nError: Could not find coordinate pattern [FF FF FF FF 00 00 00 00 00 00 00 00] after LCED marker.\n\
            This may indicate the save file is corrupted or in an unexpected format."
        );
        std::process::exit(1);
    };

    let Ok(_) = read_coordinates(&bytes, coord_offset) else {
        println!(
            "Error: Failed to read coordinates from offset 0x{:X}",
            coord_offset
        );
        std::process::exit(1);
    };

    teleport_to_location(&mut bytes, coord_offset, location);

    println!("DEBUG: Writing to file: {:?}", args.save_file);
    if let Err(e) = fs::write(&args.save_file, &bytes) {
        println!("Error: Failed to write save file: {}", e);
        std::process::exit(1);
    }

    println!("\nSuccessfully teleported to {}!", location.name);
    println!("Save file updated: {:?}", args.save_file);
}
