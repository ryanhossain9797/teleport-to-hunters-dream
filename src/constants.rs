// Teleport destination constants
pub const LANTERN_TELEPORT_MAP_ID: [u8; 4] = [0x00, 0x00, 0x00, 0x15]; // Little-endian [21, 0]
pub const LANTERN_TELEPORT_COORDS: (f32, f32, f32) = (-8.0, -6.0, -18.0);

// Save file parsing constants
pub const LCED_MARKER: [u8; 4] = [0x4C, 0x43, 0x45, 0x44]; // "LCED"

pub const COORD_PATTERN: [u8; 12] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const COORD_OFFSET_AFTER_PATTERN: usize = 12;
