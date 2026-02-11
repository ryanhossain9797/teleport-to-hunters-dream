//! A minimal CLI tool to teleport to Hunter's Dream in Bloodborne save files
//!
//! Usage: bloodborne-teleport <save_file>

use clap::Parser;
use std::fs;
use std::path::PathBuf;

const HUNTERS_DREAM_MAP_ID: [u8; 4] = [0x00, 0x00, 0x00, 0x15]; // Little-endian [21, 0]
const HUNTERS_DREAM_COORDS: [f32; 3] = [-8.0, -6.0, -18.0];
const LCED_MARKER: [u8; 4] = [0x4C, 0x43, 0x45, 0x44];

const COORD_PATTERN: [u8; 12] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[derive(Parser, Debug)]
#[command(name = "bloodborne-teleport")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the decrypted Bloodborne save file
    save_file: PathBuf,
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Find the LCED marker in the file - this marks the section where coordinates are stored
fn find_lced_marker(bytes: &[u8]) -> Option<usize> {
    eprintln!("DEBUG: Searching for LCED marker (0x4C, 0x43, 0x45, 0x44)...");
    for i in 0..(bytes.len().saturating_sub(4)) {
        if bytes[i..i + 4] == LCED_MARKER {
            eprintln!("DEBUG: Found LCED marker at offset 0x{:X} ({})", i, i);
            return Some(i);
        }
    }
    eprintln!("DEBUG: LCED marker not found");
    None
}

/// Find the coordinate pattern starting from the LCED offset
fn find_coordinates_offset(bytes: &[u8], lced_offset: usize) -> Option<usize> {
    eprintln!(
        "DEBUG: Searching for coordinate pattern starting from offset {}...",
        lced_offset
    );
    for i in lced_offset..(bytes.len().saturating_sub(COORD_PATTERN.len())) {
        if bytes[i..i + COORD_PATTERN.len()] == COORD_PATTERN {
            // Coordinates start 12 bytes after the pattern
            let coord_offset = i + 12;
            eprintln!(
                "DEBUG: Found coordinate pattern at offset 0x{:X} ({}), coordinates at offset 0x{:X} ({})",
                i, i, coord_offset, coord_offset
            );
            return Some(coord_offset);
        }
    }
    eprintln!("DEBUG: Coordinate pattern not found after LCED marker");
    None
}

fn read_coordinates(bytes: &[u8], offset: usize) -> (f32, f32, f32) {
    let x_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
    let y_bytes: [u8; 4] = bytes[offset + 4..offset + 8].try_into().unwrap();
    let z_bytes: [u8; 4] = bytes[offset + 8..offset + 12].try_into().unwrap();

    let x = f32::from_le_bytes(x_bytes);
    let y = f32::from_le_bytes(y_bytes);
    let z = f32::from_le_bytes(z_bytes);

    (x, y, z)
}

fn read_map_id(bytes: &[u8]) -> [u8; 4] {
    [bytes[0x04], bytes[0x05], bytes[0x06], bytes[0x07]]
}

fn write_coordinates(bytes: &mut [u8], offset: usize, x: f32, y: f32, z: f32) {
    eprintln!(
        "DEBUG: Writing coordinates at offset {}: X={}, Y={}, Z={}",
        offset, x, y, z
    );
    bytes[offset..offset + 4].copy_from_slice(&f32::to_le_bytes(x));
    bytes[offset + 4..offset + 8].copy_from_slice(&f32::to_le_bytes(y));
    bytes[offset + 8..offset + 12].copy_from_slice(&f32::to_le_bytes(z));
}

fn write_map_id(bytes: &mut [u8], map_id: [u8; 4]) {
    eprintln!("DEBUG: Writing map ID at offset 0x04: {:?}", map_id);
    bytes[0x04] = map_id[0];
    bytes[0x05] = map_id[1];
    bytes[0x06] = map_id[2];
    bytes[0x07] = map_id[3];
}

fn format_map_id(map_id: [u8; 4]) -> String {
    let id = u16::from_le_bytes([map_id[0], map_id[1]]);
    format!("{}", id)
}

fn main() {
    let args = Args::parse();

    eprintln!("DEBUG: Reading file: {:?}", args.save_file);
    let mut bytes = match read_file(&args.save_file) {
        Ok(b) => {
            eprintln!("DEBUG: File size: {} bytes", b.len());
            b
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    // Step 1: Find LCED marker
    let lced_offset = match find_lced_marker(&bytes) {
        Some(offset) => offset,
        None => {
            eprintln!("\nError: Could not find LCED marker in save file.");
            eprintln!("This may not be a valid decrypted Bloodborne save.");
            eprintln!("The save file should contain the bytes 'LCED' (0x4C 0x43 0x45 0x44).");
            std::process::exit(1);
        }
    };

    // Step 2: Find coordinate pattern after LCED marker
    let coord_offset = match find_coordinates_offset(&bytes, lced_offset) {
        Some(offset) => offset,
        None => {
            eprintln!("\nError: Could not find coordinate pattern after LCED marker.");
            eprintln!("Expected pattern: [FF FF FF FF 00 00 00 00 00 00 00 00]");
            eprintln!("The save file may be corrupted or in an unexpected format.");
            std::process::exit(1);
        }
    };

    let (old_x, old_y, old_z) = read_coordinates(&bytes, coord_offset);
    let old_map_id = read_map_id(&bytes);

    println!("\nTeleporting to Hunter's Dream...");
    println!(
        "  From: X={:.3}, Y={:.3}, Z={:.3} (Map: {})",
        old_x,
        old_y,
        old_z,
        format_map_id(old_map_id)
    );
    println!(
        "  To Hunter's Dream:   X={:.3}, Y={:.3}, Z={:.3} (Map: 21)",
        HUNTERS_DREAM_COORDS[0], HUNTERS_DREAM_COORDS[1], HUNTERS_DREAM_COORDS[2]
    );

    write_map_id(&mut bytes, HUNTERS_DREAM_MAP_ID);

    write_coordinates(
        &mut bytes,
        coord_offset,
        HUNTERS_DREAM_COORDS[0],
        HUNTERS_DREAM_COORDS[1],
        HUNTERS_DREAM_COORDS[2],
    );

    eprintln!("DEBUG: Writing to file: {:?}", args.save_file);
    if let Err(e) = fs::write(&args.save_file, &bytes) {
        eprintln!("Error: Failed to write save file: {}", e);
        std::process::exit(1);
    }

    println!("\nSuccessfully teleported to Hunter's Dream!");
    println!("Save file updated: {:?}", args.save_file);
}
