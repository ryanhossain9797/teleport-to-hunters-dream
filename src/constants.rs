// Save file parsing constants
pub const LCED_MARKER: [u8; 4] = [0x4C, 0x43, 0x45, 0x44]; // "LCED"

pub const COORD_PATTERN: [u8; 12] = [
    0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const COORD_OFFSET_AFTER_PATTERN: usize = 12;

/// Map ID constants (little-endian [u8; 4] format)
pub mod map_ids {
    pub const HUNTERS_DREAM: [u8; 4] = [0x00, 0x00, 0x00, 0x15]; // [21, 0]
    pub const CENTRAL_YHARNAM: [u8; 4] = [0x18, 0x01, 0x00, 0x00]; // [24, 1]
    pub const OLD_YHARNAM: [u8; 4] = [0x00, 0x00, 0x00, 0x17]; // [23, 0]
    pub const FORBIDDEN_WOODS: [u8; 4] = [0x00, 0x00, 0x00, 0x1B]; // [27, 0]
    pub const BYRGENWERTH: [u8; 4] = [0x00, 0x00, 0x00, 0x20]; // [32, 0]
    pub const YAHARGUL: [u8; 4] = [0x00, 0x00, 0x00, 0x1C]; // [28, 0]
    pub const CAINHURST: [u8; 4] = [0x00, 0x00, 0x00, 0x19]; // [25, 0]
    pub const NIGHTMARE_FRONTIER: [u8; 4] = [0x00, 0x00, 0x00, 0x21]; // [33, 0]
    pub const MERGOS_LOFT: [u8; 4] = [0x00, 0x00, 0x00, 0x1A]; // [26, 0]
    pub const HUNTERS_NIGHTMARE: [u8; 4] = [0x00, 0x00, 0x00, 0x22]; // [34, 0]
    pub const RESEARCH_HALL: [u8; 4] = [0x00, 0x00, 0x00, 0x23]; // [35, 0]
    pub const FISHING_HAMLET: [u8; 4] = [0x00, 0x00, 0x00, 0x24]; // [36, 0]
    pub const ABANDONED_WORKSHOP: [u8; 4] = [0x01, 0x01, 0x00, 0x00]; // [21, 1]
}

/// Coordinate constants
pub mod coords {
    use super::map_ids;

    pub const HUNTERS_DREAM: (f32, f32, f32, [u8; 4]) = (-8.0, -6.0, -18.0, map_ids::HUNTERS_DREAM);
    pub const FIRST_FLOOR_SICKROOM: (f32, f32, f32, [u8; 4]) =
        (-199.74, -50.759, 179.42, map_ids::CENTRAL_YHARNAM);
    pub const CENTRAL_YHARNAM: (f32, f32, f32, [u8; 4]) =
        (-193.4, -28.646, 68.5, map_ids::CENTRAL_YHARNAM);
    pub const GREAT_BRIDGE: (f32, f32, f32, [u8; 4]) =
        (-124.488, -27.021, 64.673, map_ids::CENTRAL_YHARNAM);
    pub const TOMB_OF_OEDON: (f32, f32, f32, [u8; 4]) =
        (-33.811, -40.722, 87.303, map_ids::CENTRAL_YHARNAM);
    pub const CATHEDRAL_WARD: (f32, f32, f32, [u8; 4]) =
        (16.775, -9.511, 103.27, map_ids::CENTRAL_YHARNAM);
    pub const GRAND_CATHEDRAL_WARD: (f32, f32, f32, [u8; 4]) =
        (67.808, 35.713, 339.689, map_ids::CENTRAL_YHARNAM);
    pub const UPPER_CATHEDRAL_WARD: (f32, f32, f32, [u8; 4]) =
        (-24.643, 40.621, 250.57, map_ids::CENTRAL_YHARNAM);
    pub const LUMENFLOWER_GARDENS: (f32, f32, f32, [u8; 4]) =
        (45.335, 51.403, 300.35, map_ids::CENTRAL_YHARNAM);
    pub const ALTAR_OF_DESPAIR: (f32, f32, f32, [u8; 4]) =
        (114.86, 4.443, 425.02, map_ids::CENTRAL_YHARNAM);
    pub const OLD_YHARNAM: (f32, f32, f32, [u8; 4]) = (126.4, -65.214, 36.0, map_ids::OLD_YHARNAM);
    pub const CHURCH_OF_THE_GOOD_CHALICE: (f32, f32, f32, [u8; 4]) =
        (-139.979, -126.664, 57.359, map_ids::OLD_YHARNAM);
    pub const GRAVEYARD_OF_THE_DARKBEAST: (f32, f32, f32, [u8; 4]) =
        (111.86, -120.783, -65.249, map_ids::OLD_YHARNAM);
    pub const HEMWICK_CHARNEL_LANE: (f32, f32, f32, [u8; 4]) =
        (-172.0, -22.0, 485.5, map_ids::OLD_YHARNAM);
    pub const WITCHS_ABODE: (f32, f32, f32, [u8; 4]) = (-336.3, 2.4, 733.0, map_ids::OLD_YHARNAM);
    pub const FORBIDDEN_WOODS: (f32, f32, f32, [u8; 4]) =
        (-190.0, -76.3, 252.0, map_ids::FORBIDDEN_WOODS);
    pub const FORBIDDEN_GRAVE: (f32, f32, f32, [u8; 4]) =
        (-335.0, -186.5, 479.0, map_ids::FORBIDDEN_WOODS);
    pub const BYRGENWERTH: (f32, f32, f32, [u8; 4]) = (-400.4, -180.8, 414.6, map_ids::BYRGENWERTH);
    pub const MOONSIDE_LAKE: (f32, f32, f32, [u8; 4]) =
        (-509.0, -271.0, 371.0, map_ids::BYRGENWERTH);
    pub const YAHARGUL_UNSEEN_VILLAGE: (f32, f32, f32, [u8; 4]) =
        (257.4, -51.4, 70.0, map_ids::YAHARGUL);
    pub const YAHARGUL_CHAPEL: (f32, f32, f32, [u8; 4]) = (260.4, -88.0, -55.6, map_ids::YAHARGUL);
    pub const ADVENT_PLAZA: (f32, f32, f32, [u8; 4]) = (418.8, -123.6, -253.4, map_ids::YAHARGUL);
    pub const HYPOGEAN_GAOL: (f32, f32, f32, [u8; 4]) = (219.6, -97.6, -78.8, map_ids::YAHARGUL);
    pub const FORSAKEN_CASTLE_CAINHURST: (f32, f32, f32, [u8; 4]) =
        (-4.5, 33.8, -187.9, map_ids::CAINHURST);
    pub const LOGARIUS_SEAT: (f32, f32, f32, [u8; 4]) = (47.8, 111.8, -350.4, map_ids::CAINHURST);
    pub const VILEBLOOD_QUEENS_CHAMBER: (f32, f32, f32, [u8; 4]) =
        (122.4, 129.0, -455.0, map_ids::CAINHURST);
    pub const ABANDONED_OLD_WORKSHOP: (f32, f32, f32, [u8; 4]) =
        (129.8, -19.9, 140.8, map_ids::ABANDONED_WORKSHOP);
    pub const LECTURE_BUILDING: (f32, f32, f32, [u8; 4]) =
        (-472.37, -185.25, 594.9, map_ids::BYRGENWERTH);
    pub const LECTURE_BUILDING_2ND_FLOOR: (f32, f32, f32, [u8; 4]) =
        (-444.22, -177.25, 514.19, map_ids::BYRGENWERTH);
    pub const NIGHTMARE_FRONTIER: (f32, f32, f32, [u8; 4]) =
        (0.35, 1500.0, 0.0, map_ids::NIGHTMARE_FRONTIER);
    pub const NIGHTMARE_OF_MENSIS: (f32, f32, f32, [u8; 4]) =
        (-104.65, 1462.28, -42.65, map_ids::NIGHTMARE_FRONTIER);
    pub const MERGOS_LOFT_BASE: (f32, f32, f32, [u8; 4]) =
        (84.58, 986.7, -0.37, map_ids::MERGOS_LOFT);
    pub const MERGOS_LOFT_MIDDLE: (f32, f32, f32, [u8; 4]) =
        (136.69, 1061.26, -14.86, map_ids::MERGOS_LOFT);
    pub const WET_NURSES_LUNARIUM: (f32, f32, f32, [u8; 4]) =
        (140.72, 1124.3, -37.98, map_ids::MERGOS_LOFT);
    pub const HUNTERS_NIGHTMARE: (f32, f32, f32, [u8; 4]) =
        (-481.68, 1490.49, -497.73, map_ids::HUNTERS_NIGHTMARE);
    pub const NIGHTMARE_CHURCH: (f32, f32, f32, [u8; 4]) =
        (-434.08, 1503.18, -594.52, map_ids::HUNTERS_NIGHTMARE);
    pub const NIGHTMARE_GRAND_CATHEDRAL: (f32, f32, f32, [u8; 4]) =
        (-433.09, 1535.71, -261.57, map_ids::HUNTERS_NIGHTMARE);
    pub const UNDERGROUND_CORPSE_PILE: (f32, f32, f32, [u8; 4]) =
        (-406.81, 1503.79, -743.0, map_ids::HUNTERS_NIGHTMARE);
    pub const RESEARCH_HALL: (f32, f32, f32, [u8; 4]) =
        (-318.67, 1553.02, -824.22, map_ids::RESEARCH_HALL);
    pub const LUMENWOOD_GARDEN: (f32, f32, f32, [u8; 4]) =
        (-432.15, 1593.0, -824.37, map_ids::RESEARCH_HALL);
    pub const ASTRAL_CLOCKTOWER: (f32, f32, f32, [u8; 4]) =
        (-454.88, 1595.57, -824.44, map_ids::RESEARCH_HALL);
    pub const FISHING_HAMLET: (f32, f32, f32, [u8; 4]) =
        (-619.2, 1594.3, -817.2, map_ids::FISHING_HAMLET);
    pub const LIGHTHOUSE_HUT: (f32, f32, f32, [u8; 4]) =
        (-645.2, 1614.66, -867.2, map_ids::FISHING_HAMLET);
    pub const COAST: (f32, f32, f32, [u8; 4]) = (-695.2, 1577.27, -943.2, map_ids::FISHING_HAMLET);
}
