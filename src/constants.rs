// Save file parsing constants
pub const LCED_MARKER: [u8; 4] = [0x4C, 0x43, 0x45, 0x44]; // "LCED"

pub const COORD_PATTERN: [u8; 12] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const COORD_OFFSET_AFTER_PATTERN: usize = 12;

/// Map ID constants (little-endian [u8; 4] format)
pub mod map_ids {
    // Hunter's Dream
    pub const HUNTERS_DREAM: [u8; 4] = [0x00, 0x00, 0x00, 0x15]; // [21, 0]

    // Yharnam Headstone
    pub const CENTRAL_YHARNAM_1: [u8; 4] = [0x18, 0x01, 0x00, 0x00]; // [24, 1]
    pub const CENTRAL_YHARNAM_0: [u8; 4] = [0x00, 0x00, 0x00, 0x15]; // [24, 0] (same as 21,0 but different usage)
    pub const CENTRAL_YHARNAM_2: [u8; 4] = [0x02, 0x00, 0x00, 0x00]; // [24, 2]
    pub const OLD_YHARNAM_0: [u8; 4] = [0x00, 0x00, 0x00, 0x17]; // [23, 0]

    // Frontier Headstone
    pub const HEMWICK_0: [u8; 4] = [0x00, 0x00, 0x00, 0x16]; // [22, 0]
    pub const FORBIDDEN_WOODS_0: [u8; 4] = [0x00, 0x00, 0x00, 0x1B]; // [27, 0]
    pub const BYRGENWERTH_0: [u8; 4] = [0x00, 0x00, 0x00, 0x20]; // [32, 0]
    pub const BYRGENWERTH_2: [u8; 4] = [0x02, 0x00, 0x00, 0x20]; // [32, 2]

    // Unseen Headstone
    pub const YAHARGUL_0: [u8; 4] = [0x00, 0x00, 0x00, 0x1C]; // [28, 0]
    pub const CAINHURST_0: [u8; 4] = [0x00, 0x00, 0x00, 0x19]; // [25, 0]
    pub const ABANDONED_WORKSHOP_1: [u8; 4] = [0x01, 0x01, 0x00, 0x00]; // [21, 1]

    // Nightmare Headstone
    pub const NIGHTMARE_FRONTIER_0: [u8; 4] = [0x00, 0x00, 0x00, 0x21]; // [33, 0]
    pub const MERGOS_LOFT_0: [u8; 4] = [0x00, 0x00, 0x00, 0x1A]; // [26, 0]

    // Hunter's Nightmare Headstone
    pub const HUNTERS_NIGHTMARE_0: [u8; 4] = [0x00, 0x00, 0x00, 0x22]; // [34, 0]
    pub const RESEARCH_HALL_0: [u8; 4] = [0x00, 0x00, 0x00, 0x23]; // [35, 0]
    pub const FISHING_HAMLET_0: [u8; 4] = [0x00, 0x00, 0x00, 0x24]; // [36, 0]
}
