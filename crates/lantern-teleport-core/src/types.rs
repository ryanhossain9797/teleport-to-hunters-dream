//! Type definitions for lantern teleport functionality

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

/// Current position in the game world extracted from a save file
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentPosition {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
    /// Z coordinate
    pub z: f32,
    /// Map ID as a 4-byte array (save file format)
    pub map_id: [u8; 4],
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
