use strum::{Display, EnumString};

/// Represents a teleport destination in Bloodborne
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub name: &'static str,
    pub region: &'static str,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub map_id: [u8; 4],
}

/// Two-way mapping between enum variants and their string representations
#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString)]
pub enum LocationName {
    #[strum(serialize = "Hunter's Dream")]
    HuntersDream,
}

impl LocationName {
    /// Get the Location data for this enum variant
    pub fn get_location(self) -> Location {
        match self {
            LocationName::HuntersDream => Location {
                name: "Hunter's Dream",
                region: "Hunter's Dream",
                x: crate::constants::LANTERN_TELEPORT_COORDS.0,
                y: crate::constants::LANTERN_TELEPORT_COORDS.1,
                z: crate::constants::LANTERN_TELEPORT_COORDS.2,
                map_id: crate::constants::LANTERN_TELEPORT_MAP_ID,
            },
        }
    }

    /// Get all available location names
    pub fn all() -> &'static [LocationName] {
        &[LocationName::HuntersDream]
    }
}
