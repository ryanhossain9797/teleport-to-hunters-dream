//! A minimal CLI tool to teleport to Hunter's Dream in Bloodborne save files
//!
//! Usage: bloodborne-teleport <save_file>

use clap::Parser;
use std::fs;
use std::path::PathBuf;

const HUNTERS_DREAM_MAP_ID: [u8; 4] = [0x00, 0x00, 0x00, 0x15]; // Little-endian [21, 0]
const HUNTERS_DREAM_COORDS: [f32; 3] = [-8.0, -6.0, -18.0];

const LCED_PATTERN: [u8; 12] = [
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

fn find_lced_offset(bytes: &[u8]) -> Option<usize> {
    for i in 0..(bytes.len().saturating_sub(LCED_PATTERN.len())) {
        if bytes[i..i + LCED_PATTERN.len()] == LCED_PATTERN {
            // Coordinates start 12 bytes after the pattern
            return Some(i + 12);
        }
    }
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
    bytes[offset..offset + 4].copy_from_slice(&f32::to_le_bytes(x));
    bytes[offset + 4..offset + 8].copy_from_slice(&f32::to_le_bytes(y));
    bytes[offset + 8..offset + 12].copy_from_slice(&f32::to_le_bytes(z));
}

fn write_map_id(bytes: &mut [u8], map_id: [u8; 4]) {
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

    let mut bytes = match read_file(&args.save_file) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let lced_offset = match find_lced_offset(&bytes) {
        Some(offset) => offset,
        None => {
            eprintln!("Error: Could not find LCED pattern in save file.");
            eprintln!("This may not be a valid decrypted Bloodborne save.");
            std::process::exit(1);
        }
    };

    let (old_x, old_y, old_z) = read_coordinates(&bytes, lced_offset);
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
        lced_offset,
        HUNTERS_DREAM_COORDS[0],
        HUNTERS_DREAM_COORDS[1],
        HUNTERS_DREAM_COORDS[2],
    );

    if let Err(e) = fs::write(&args.save_file, &bytes) {
        eprintln!("Error: Failed to write save file: {}", e);
        std::process::exit(1);
    }

    println!("\nSuccessfully teleported to Hunter's Dream!");
    println!("Save file updated: {:?}", args.save_file);
}
