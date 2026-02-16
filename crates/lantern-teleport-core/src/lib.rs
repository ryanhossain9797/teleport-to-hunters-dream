//! Core library for lantern teleport functionality
//!
//! This library provides functions to work with Bloodborne lantern teleport locations.

use std::fs;
use std::path::Path;

// ============================================================================
// Module declarations
// ============================================================================

mod constants;
mod types;

// Re-export public types
pub use types::{CurrentPosition, Location, TeleportError};

// Re-export public constants
pub use constants::LOCATIONS;

// ============================================================================
// Public API
// ============================================================================

/// Get a reference to all available lantern locations.
///
/// Returns a fixed-size slice of all teleport locations.
#[inline]
pub fn get_all_locations() -> &'static [Location] {
    &constants::LOCATIONS
}

/// Search for locations matching the given query string.
///
/// The search is case-insensitive and performs partial matching against location names.
///
/// # Arguments
///
/// * `query` - The search string to match against location names
///
/// # Returns
///
/// A vector of references to all locations that match the query
pub fn search_locations(query: &str) -> Vec<&'static Location> {
    let lower_query = query.to_lowercase();
    constants::LOCATIONS
        .iter()
        .filter(|loc| loc.name.to_lowercase().contains(&lower_query))
        .collect()
}

/// Teleport to the specified location in a Bloodborne save file.
///
/// This function modifies the save file at the given path to teleport
/// the character to the specified location.
///
/// # Arguments
///
/// * `save_path` - Path to the Bloodborne save file
/// * `location` - The destination location
///
/// # Returns
///
/// `Ok(())` on success, or a `TeleportError` if the operation fails
pub fn teleport<P: AsRef<Path>>(save_path: P, location: &Location) -> Result<(), TeleportError> {
    // Read the save file
    let path = save_path.as_ref();
    let mut bytes = fs::read(path).map_err(|e| TeleportError::ReadError(e.to_string()))?;

    // Find LCED marker
    let lced_offset = find_lced_marker(&bytes).ok_or(TeleportError::LcedMarkerNotFound)?;

    // Find coordinates offset
    let coord_offset =
        find_coordinates_offset(&bytes, lced_offset).ok_or(TeleportError::CoordPatternNotFound)?;

    // Write the new coordinates and map ID
    write_map_id(&mut bytes, location);
    write_coordinates(&mut bytes, coord_offset, location);

    // Write the modified save file back
    fs::write(path, &bytes).map_err(|e| TeleportError::WriteError(e.to_string()))?;

    Ok(())
}

/// Validate a save file and extract the current position.
///
/// This function reads a Bloodborne save file and validates that it contains
/// the expected data structures (LCED marker and coordinate pattern).
///
/// # Arguments
///
/// * `save_path` - Path to the Bloodborne save file
///
/// # Returns
///
/// `Ok(CurrentPosition)` with the current coordinates and map ID if valid,
/// or a `TeleportError` if the file is invalid or cannot be read.
pub fn validate_save_file<P: AsRef<Path>>(save_path: P) -> Result<CurrentPosition, TeleportError> {
    // Read the save file
    let path = save_path.as_ref();
    let bytes = fs::read(path).map_err(|e| TeleportError::ReadError(e.to_string()))?;

    // Find LCED marker
    let lced_offset = find_lced_marker(&bytes).ok_or(TeleportError::LcedMarkerNotFound)?;

    // Find coordinates offset
    let coord_offset =
        find_coordinates_offset(&bytes, lced_offset).ok_or(TeleportError::CoordPatternNotFound)?;

    // Extract current position
    let x = f32::from_le_bytes(
        bytes[coord_offset..coord_offset + 4]
            .try_into()
            .map_err(|_| TeleportError::InvalidOffset)?,
    );
    let y = f32::from_le_bytes(
        bytes[coord_offset + 4..coord_offset + 8]
            .try_into()
            .map_err(|_| TeleportError::InvalidOffset)?,
    );
    let z = f32::from_le_bytes(
        bytes[coord_offset + 8..coord_offset + 12]
            .try_into()
            .map_err(|_| TeleportError::InvalidOffset)?,
    );
    let map_id: [u8; 4] = bytes[0x04..0x08]
        .try_into()
        .map_err(|_| TeleportError::InvalidOffset)?;

    Ok(CurrentPosition { x, y, z, map_id })
}

// ============================================================================
// Private helper functions
// ============================================================================

#[inline]
fn find_lced_marker(bytes: &[u8]) -> Option<usize> {
    for i in 0..(bytes.len().saturating_sub(4)) {
        if bytes[i..i + 4] == constants::LCED_MARKER_BYTES {
            return Some(i);
        }
    }
    None
}

#[inline]
fn find_coordinates_offset(bytes: &[u8], lced_offset: usize) -> Option<usize> {
    let search_start = lced_offset;
    let search_end = bytes
        .len()
        .saturating_sub(constants::COORD_PATTERN_BYTES.len());

    for i in search_start..search_end {
        if bytes[i..i + constants::COORD_PATTERN_BYTES.len()] == constants::COORD_PATTERN_BYTES {
            return Some(i + constants::COORD_OFFSET_AFTER_PATTERN_BYTES);
        }
    }
    None
}

#[inline]
fn write_coordinates(bytes: &mut [u8], offset: usize, location: &Location) {
    bytes[offset..offset + 4].copy_from_slice(&f32::to_le_bytes(location.x));
    bytes[offset + 4..offset + 8].copy_from_slice(&f32::to_le_bytes(location.y));
    bytes[offset + 8..offset + 12].copy_from_slice(&f32::to_le_bytes(location.z));
}

#[inline]
fn write_map_id(bytes: &mut [u8], location: &Location) {
    let save_format = constants::map_ids::to_save_format(&location.map_id);
    bytes[0x04] = save_format[0];
    bytes[0x05] = save_format[1];
    bytes[0x06] = save_format[2];
    bytes[0x07] = save_format[3];
}
