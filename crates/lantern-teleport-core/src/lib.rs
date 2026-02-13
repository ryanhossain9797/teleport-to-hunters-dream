//! Core library for lantern teleport functionality
//!
//! This library provides functions to work with Bloodborne lantern teleport locations.

use std::fs;
use std::path::Path;

/// Represents a teleport destination in Bloodborne
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    /// Display name of the location
    pub name: &'static str,
    /// Region where this location belongs
    pub region: &'static str,
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
    /// Map ID as a 2-byte array
    pub map_id: [u8; 2],
}

// ============================================================================
// Private constants - internal use only
// ============================================================================

const LCED_MARKER: [u8; 4] = [0x4C, 0x43, 0x45, 0x44]; // "LCED"

const COORD_PATTERN: [u8; 12] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

const COORD_OFFSET_AFTER_PATTERN: usize = 12;

mod map_ids {

    // Hunter's Dream
    pub const HUNTERS_DREAM: [u8; 2] = [0x00, 0x15];

    // Yharnam Headstone
    pub const CENTRAL_YHARNAM_1: [u8; 2] = [0x01, 0x18];
    pub const CENTRAL_YHARNAM_0: [u8; 2] = [0x00, 0x18];
    pub const CENTRAL_YHARNAM_2: [u8; 2] = [0x02, 0x18];
    pub const OLD_YHARNAM_0: [u8; 2] = [0x00, 0x17];

    // Frontier Headstone
    pub const HEMWICK_0: [u8; 2] = [0x00, 0x16];
    pub const FORBIDDEN_WOODS_0: [u8; 2] = [0x00, 0x1B];
    pub const BYRGENWERTH_0: [u8; 2] = [0x00, 0x20];
    pub const BYRGENWERTH_2: [u8; 2] = [0x02, 0x20];

    // Unseen Headstone
    pub const YAHARGUL_0: [u8; 2] = [0x00, 0x1C];
    pub const CAINHURST_0: [u8; 2] = [0x00, 0x19];
    pub const ABANDONED_WORKSHOP_1: [u8; 2] = [0x01, 0x15];

    // Nightmare Headstone
    pub const NIGHTMARE_FRONTIER_0: [u8; 2] = [0x00, 0x21];
    pub const MERGOS_LOFT_0: [u8; 2] = [0x00, 0x1A];

    // Hunter's Nightmare Headstone
    pub const HUNTERS_NIGHTMARE_0: [u8; 2] = [0x00, 0x22];
    pub const RESEARCH_HALL_0: [u8; 2] = [0x00, 0x23];
    pub const FISHING_HAMLET_0: [u8; 2] = [0x00, 0x24];

    /// Convert a map ID to the 4-byte format used in save files
    pub fn to_save_format(map_id: &[u8; 2]) -> [u8; 4] {
        [0x00, 0x00, map_id[1], map_id[0]]
    }
}

// ============================================================================
// Location data - all available teleport locations
// ============================================================================

/// All available lantern teleport locations
pub const LOCATIONS: [Location; 44] = [
    // Hunter's Dream
    Location {
        name: "Hunter's Dream",
        region: "Hunter's Dream",
        x: -8.0,
        y: -6.0,
        z: -18.0,
        map_id: map_ids::HUNTERS_DREAM,
    },
    // Yharnam Headstone
    Location {
        name: "1st Floor Sickroom",
        region: "Yharnam Headstone",
        x: -199.74,
        y: -50.759,
        z: 179.42,
        map_id: map_ids::CENTRAL_YHARNAM_1,
    },
    Location {
        name: "Central Yharnam",
        region: "Yharnam Headstone",
        x: -193.4,
        y: -28.646,
        z: 68.5,
        map_id: map_ids::CENTRAL_YHARNAM_1,
    },
    Location {
        name: "Great Bridge",
        region: "Yharnam Headstone",
        x: -124.488,
        y: -27.021,
        z: 64.673,
        map_id: map_ids::CENTRAL_YHARNAM_1,
    },
    Location {
        name: "Tomb of Oedon",
        region: "Yharnam Headstone",
        x: -33.811,
        y: -40.722,
        z: 87.303,
        map_id: map_ids::CENTRAL_YHARNAM_1,
    },
    Location {
        name: "Cathedral Ward",
        region: "Yharnam Headstone",
        x: 16.775,
        y: -9.511,
        z: 103.27,
        map_id: map_ids::CENTRAL_YHARNAM_0,
    },
    Location {
        name: "Grand Cathedral Ward",
        region: "Yharnam Headstone",
        x: 67.808,
        y: 35.713,
        z: 339.689,
        map_id: map_ids::CENTRAL_YHARNAM_0,
    },
    Location {
        name: "Upper Cathedral Ward",
        region: "Yharnam Headstone",
        x: -24.643,
        y: 40.621,
        z: 250.57,
        map_id: map_ids::CENTRAL_YHARNAM_2,
    },
    Location {
        name: "Lumenflower Gardens",
        region: "Yharnam Headstone",
        x: 45.335,
        y: 51.403,
        z: 300.35,
        map_id: map_ids::CENTRAL_YHARNAM_2,
    },
    Location {
        name: "Altar of Despair",
        region: "Yharnam Headstone",
        x: 114.86,
        y: 4.443,
        z: 425.02,
        map_id: map_ids::CENTRAL_YHARNAM_2,
    },
    Location {
        name: "Old Yharnam",
        region: "Yharnam Headstone",
        x: 126.4,
        y: -65.214,
        z: 36.0,
        map_id: map_ids::OLD_YHARNAM_0,
    },
    Location {
        name: "Church of the Good Chalice",
        region: "Yharnam Headstone",
        x: -139.979,
        y: -126.664,
        z: 57.359,
        map_id: map_ids::OLD_YHARNAM_0,
    },
    Location {
        name: "Graveyard of the Darkbeast",
        region: "Yharnam Headstone",
        x: 111.86,
        y: -120.783,
        z: -65.249,
        map_id: map_ids::OLD_YHARNAM_0,
    },
    // Frontier Headstone
    Location {
        name: "Hemwick Charnel Lane",
        region: "Frontier Headstone",
        x: -172.0,
        y: -22.0,
        z: 485.5,
        map_id: map_ids::HEMWICK_0,
    },
    Location {
        name: "Witch's Abode",
        region: "Frontier Headstone",
        x: -336.3,
        y: 2.4,
        z: 733.0,
        map_id: map_ids::HEMWICK_0,
    },
    Location {
        name: "Forbidden Woods",
        region: "Frontier Headstone",
        x: -190.0,
        y: -76.3,
        z: 252.0,
        map_id: map_ids::FORBIDDEN_WOODS_0,
    },
    Location {
        name: "Forbidden Grave",
        region: "Frontier Headstone",
        x: -335.0,
        y: -186.5,
        z: 479.0,
        map_id: map_ids::FORBIDDEN_WOODS_0,
    },
    Location {
        name: "Byrgenwerth",
        region: "Frontier Headstone",
        x: -400.4,
        y: -180.8,
        z: 414.6,
        map_id: map_ids::BYRGENWERTH_0,
    },
    Location {
        name: "Moonside Lake",
        region: "Frontier Headstone",
        x: -509.0,
        y: -271.0,
        z: 371.0,
        map_id: map_ids::BYRGENWERTH_2,
    },
    // Unseen Headstone
    Location {
        name: "Yahar'gul, Unseen Village",
        region: "Unseen Headstone",
        x: 257.4,
        y: -51.4,
        z: 70.0,
        map_id: map_ids::YAHARGUL_0,
    },
    Location {
        name: "Yahar'gul Chapel",
        region: "Unseen Headstone",
        x: 260.4,
        y: -88.0,
        z: -55.6,
        map_id: map_ids::YAHARGUL_0,
    },
    Location {
        name: "Advent Plaza",
        region: "Unseen Headstone",
        x: 418.8,
        y: -123.6,
        z: -253.4,
        map_id: map_ids::YAHARGUL_0,
    },
    Location {
        name: "Hypogean Gaol",
        region: "Unseen Headstone",
        x: 219.6,
        y: -97.6,
        z: -78.8,
        map_id: map_ids::YAHARGUL_0,
    },
    Location {
        name: "Forsaken Castle Cainhurst",
        region: "Unseen Headstone",
        x: -4.5,
        y: 33.8,
        z: -187.9,
        map_id: map_ids::CAINHURST_0,
    },
    Location {
        name: "Logarius' Seat",
        region: "Unseen Headstone",
        x: 47.8,
        y: 111.8,
        z: -350.4,
        map_id: map_ids::CAINHURST_0,
    },
    Location {
        name: "Vileblood Queen's Chamber",
        region: "Unseen Headstone",
        x: 122.4,
        y: 129.0,
        z: -455.0,
        map_id: map_ids::CAINHURST_0,
    },
    Location {
        name: "Abandoned Old Workshop",
        region: "Unseen Headstone",
        x: 129.8,
        y: -19.9,
        z: 140.8,
        map_id: map_ids::ABANDONED_WORKSHOP_1,
    },
    // Nightmare Headstone
    Location {
        name: "Lecture Building",
        region: "Nightmare Headstone",
        x: -472.37,
        y: -185.25,
        z: 594.9,
        map_id: map_ids::BYRGENWERTH_0,
    },
    Location {
        name: "Lecture Building 2nd Floor",
        region: "Nightmare Headstone",
        x: -444.22,
        y: -177.25,
        z: 514.19,
        map_id: map_ids::BYRGENWERTH_0,
    },
    Location {
        name: "Nightmare Frontier",
        region: "Nightmare Headstone",
        x: 0.35,
        y: 1500.0,
        z: 0.0,
        map_id: map_ids::NIGHTMARE_FRONTIER_0,
    },
    Location {
        name: "Nightmare of Mensis",
        region: "Nightmare Headstone",
        x: -104.65,
        y: 1462.28,
        z: -42.65,
        map_id: map_ids::NIGHTMARE_FRONTIER_0,
    },
    Location {
        name: "Mergo's Loft: Base",
        region: "Nightmare Headstone",
        x: 84.58,
        y: 986.7,
        z: -0.37,
        map_id: map_ids::MERGOS_LOFT_0,
    },
    Location {
        name: "Mergo's Loft: Middle",
        region: "Nightmare Headstone",
        x: 136.69,
        y: 1061.26,
        z: -14.86,
        map_id: map_ids::MERGOS_LOFT_0,
    },
    Location {
        name: "Wet Nurse's Lunarium",
        region: "Nightmare Headstone",
        x: 140.72,
        y: 1124.3,
        z: -37.98,
        map_id: map_ids::MERGOS_LOFT_0,
    },
    // Hunter's Nightmare Headstone
    Location {
        name: "Hunter's Nightmare",
        region: "Hunter's Nightmare Headstone",
        x: -481.68,
        y: 1490.49,
        z: -497.73,
        map_id: map_ids::HUNTERS_NIGHTMARE_0,
    },
    Location {
        name: "Nightmare Church",
        region: "Hunter's Nightmare Headstone",
        x: -434.08,
        y: 1503.18,
        z: -594.52,
        map_id: map_ids::HUNTERS_NIGHTMARE_0,
    },
    Location {
        name: "Nightmare Grand Cathedral",
        region: "Hunter's Nightmare Headstone",
        x: -433.09,
        y: 1535.71,
        z: -261.57,
        map_id: map_ids::HUNTERS_NIGHTMARE_0,
    },
    Location {
        name: "Underground Corpse Pile",
        region: "Hunter's Nightmare Headstone",
        x: -406.81,
        y: 1503.79,
        z: -743.0,
        map_id: map_ids::HUNTERS_NIGHTMARE_0,
    },
    Location {
        name: "Research Hall",
        region: "Hunter's Nightmare Headstone",
        x: -318.67,
        y: 1553.02,
        z: -824.22,
        map_id: map_ids::RESEARCH_HALL_0,
    },
    Location {
        name: "Lumenwood Garden",
        region: "Hunter's Nightmare Headstone",
        x: -432.15,
        y: 1593.0,
        z: -824.37,
        map_id: map_ids::RESEARCH_HALL_0,
    },
    Location {
        name: "Astral Clocktower",
        region: "Hunter's Nightmare Headstone",
        x: -454.88,
        y: 1595.57,
        z: -824.44,
        map_id: map_ids::RESEARCH_HALL_0,
    },
    Location {
        name: "Fishing Hamlet",
        region: "Hunter's Nightmare Headstone",
        x: -619.2,
        y: 1594.3,
        z: -817.2,
        map_id: map_ids::FISHING_HAMLET_0,
    },
    Location {
        name: "Lighthouse Hut",
        region: "Hunter's Nightmare Headstone",
        x: -645.2,
        y: 1614.66,
        z: -867.2,
        map_id: map_ids::FISHING_HAMLET_0,
    },
    Location {
        name: "Coast",
        region: "Hunter's Nightmare Headstone",
        x: -695.2,
        y: 1577.27,
        z: -943.2,
        map_id: map_ids::FISHING_HAMLET_0,
    },
];

// ============================================================================
// Public API
// ============================================================================

/// Get a reference to all available lantern locations.
///
/// Returns a fixed-size slice of all teleport locations.
#[inline]
pub fn get_all_locations() -> &'static [Location] {
    &LOCATIONS
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
    LOCATIONS
        .iter()
        .filter(|loc| loc.name.to_lowercase().contains(&lower_query))
        .collect()
}

/// Error type for teleport operations
#[derive(Debug)]
pub enum TeleportError {
    /// Failed to read the save file
    ReadError(String),
    /// Failed to write the save file
    WriteError(String),
    /// Could not find LCED marker in save file
    LcedMarkerNotFound,
    /// Could not find coordinate pattern in save file
    CoordPatternNotFound,
    /// Invalid coordinate offset
    InvalidOffset,
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

// ============================================================================
// Private helper functions
// ============================================================================

#[inline]
fn find_lced_marker(bytes: &[u8]) -> Option<usize> {
    for i in 0..(bytes.len().saturating_sub(4)) {
        if bytes[i..i + 4] == LCED_MARKER {
            return Some(i);
        }
    }
    None
}

#[inline]
fn find_coordinates_offset(bytes: &[u8], lced_offset: usize) -> Option<usize> {
    let search_start = lced_offset;
    let search_end = bytes.len().saturating_sub(COORD_PATTERN.len());

    for i in search_start..search_end {
        if bytes[i..i + COORD_PATTERN.len()] == COORD_PATTERN {
            return Some(i + COORD_OFFSET_AFTER_PATTERN);
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
    let save_format = map_ids::to_save_format(&location.map_id);
    bytes[0x04] = save_format[0];
    bytes[0x05] = save_format[1];
    bytes[0x06] = save_format[2];
    bytes[0x07] = save_format[3];
}
