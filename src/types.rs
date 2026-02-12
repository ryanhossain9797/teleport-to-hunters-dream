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
    // Hunter's Dream
    #[strum(serialize = "Hunter's Dream")]
    HuntersDream,

    // Yharnam Headstone
    #[strum(serialize = "1st Floor Sickroom")]
    FirstFloorSickroom,
    #[strum(serialize = "Central Yharnam")]
    CentralYharnam,
    #[strum(serialize = "Great Bridge")]
    GreatBridge,
    #[strum(serialize = "Tomb of Oedon")]
    TombOfOedon,
    #[strum(serialize = "Cathedral Ward")]
    CathedralWard,
    #[strum(serialize = "Grand Cathedral Ward")]
    GrandCathedralWard,
    #[strum(serialize = "Upper Cathedral Ward")]
    UpperCathedralWard,
    #[strum(serialize = "Lumenflower Gardens")]
    LumenflowerGardens,
    #[strum(serialize = "Altar of Despair")]
    AltarOfDespair,
    #[strum(serialize = "Old Yharnam")]
    OldYharnam,
    #[strum(serialize = "Church of the Good Chalice")]
    ChurchOfTheGoodChalice,
    #[strum(serialize = "Graveyard of the Darkbeast")]
    GraveyardOfTheDarkbeast,

    // Frontier Headstone
    #[strum(serialize = "Hemwick Charnel Lane")]
    HemwickCharnelLane,
    #[strum(serialize = "Witch's Abode")]
    WitchsAbode,
    #[strum(serialize = "Forbidden Woods")]
    ForbiddenWoods,
    #[strum(serialize = "Forbidden Grave")]
    ForbiddenGrave,
    #[strum(serialize = "Byrgenwerth")]
    Byrgenwerth,
    #[strum(serialize = "Moonside Lake")]
    MoonsideLake,

    // Unseen Headstone
    #[strum(serialize = "Yahar'gul, Unseen Village")]
    YahargulUnseenVillage,
    #[strum(serialize = "Yahar'gul Chapel")]
    YahargulChapel,
    #[strum(serialize = "Advent Plaza")]
    AdventPlaza,
    #[strum(serialize = "Hypogean Gaol")]
    HypogeanGaol,
    #[strum(serialize = "Forsaken Castle Cainhurst")]
    ForsakenCastleCainhurst,
    #[strum(serialize = "Logarius' Seat")]
    LogariusSeat,
    #[strum(serialize = "Vileblood Queen's Chamber")]
    VilebloodQueensChamber,
    #[strum(serialize = "Abandoned Old Workshop")]
    AbandonedOldWorkshop,

    // Nightmare Headstone
    #[strum(serialize = "Lecture Building")]
    LectureBuilding,
    #[strum(serialize = "Lecture Building 2nd Floor")]
    LectureBuilding2ndFloor,
    #[strum(serialize = "Nightmare Frontier")]
    NightmareFrontier,
    #[strum(serialize = "Nightmare of Mensis")]
    NightmareOfMensis,
    #[strum(serialize = "Mergo's Loft: Base")]
    MergosLoftBase,
    #[strum(serialize = "Mergo's Loft: Middle")]
    MergosLoftMiddle,
    #[strum(serialize = "Wet Nurse's Lunarium")]
    WetNursesLunarium,

    // Hunter's Nightmare Headstone
    #[strum(serialize = "Hunter's Nightmare")]
    HuntersNightmare,
    #[strum(serialize = "Nightmare Church")]
    NightmareChurch,
    #[strum(serialize = "Nightmare Grand Cathedral")]
    NightmareGrandCathedral,
    #[strum(serialize = "Underground Corpse Pile")]
    UndergroundCorpsePile,
    #[strum(serialize = "Research Hall")]
    ResearchHall,
    #[strum(serialize = "Lumenwood Garden")]
    LumenwoodGarden,
    #[strum(serialize = "Astral Clocktower")]
    AstralClocktower,
    #[strum(serialize = "Fishing Hamlet")]
    FishingHamlet,
    #[strum(serialize = "Lighthouse Hut")]
    LighthouseHut,
    #[strum(serialize = "Coast")]
    Coast,
}

impl LocationName {
    /// Get the Location data for this enum variant
    pub fn get_location(self) -> Location {
        use crate::constants::coords::*;

        match self {
            // Hunter's Dream
            LocationName::HuntersDream => Location {
                name: "Hunter's Dream",
                region: "Hunter's Dream",
                x: HUNTERS_DREAM.0,
                y: HUNTERS_DREAM.1,
                z: HUNTERS_DREAM.2,
                map_id: HUNTERS_DREAM.3,
            },

            // Yharnam Headstone
            LocationName::FirstFloorSickroom => Location {
                name: "1st Floor Sickroom",
                region: "Yharnam Headstone",
                x: FIRST_FLOOR_SICKROOM.0,
                y: FIRST_FLOOR_SICKROOM.1,
                z: FIRST_FLOOR_SICKROOM.2,
                map_id: FIRST_FLOOR_SICKROOM.3,
            },
            LocationName::CentralYharnam => Location {
                name: "Central Yharnam",
                region: "Yharnam Headstone",
                x: CENTRAL_YHARNAM.0,
                y: CENTRAL_YHARNAM.1,
                z: CENTRAL_YHARNAM.2,
                map_id: CENTRAL_YHARNAM.3,
            },
            LocationName::GreatBridge => Location {
                name: "Great Bridge",
                region: "Yharnam Headstone",
                x: GREAT_BRIDGE.0,
                y: GREAT_BRIDGE.1,
                z: GREAT_BRIDGE.2,
                map_id: GREAT_BRIDGE.3,
            },
            LocationName::TombOfOedon => Location {
                name: "Tomb of Oedon",
                region: "Yharnam Headstone",
                x: TOMB_OF_OEDON.0,
                y: TOMB_OF_OEDON.1,
                z: TOMB_OF_OEDON.2,
                map_id: TOMB_OF_OEDON.3,
            },
            LocationName::CathedralWard => Location {
                name: "Cathedral Ward",
                region: "Yharnam Headstone",
                x: CATHEDRAL_WARD.0,
                y: CATHEDRAL_WARD.1,
                z: CATHEDRAL_WARD.2,
                map_id: CATHEDRAL_WARD.3,
            },
            LocationName::GrandCathedralWard => Location {
                name: "Grand Cathedral Ward",
                region: "Yharnam Headstone",
                x: GRAND_CATHEDRAL_WARD.0,
                y: GRAND_CATHEDRAL_WARD.1,
                z: GRAND_CATHEDRAL_WARD.2,
                map_id: GRAND_CATHEDRAL_WARD.3,
            },
            LocationName::UpperCathedralWard => Location {
                name: "Upper Cathedral Ward",
                region: "Yharnam Headstone",
                x: UPPER_CATHEDRAL_WARD.0,
                y: UPPER_CATHEDRAL_WARD.1,
                z: UPPER_CATHEDRAL_WARD.2,
                map_id: UPPER_CATHEDRAL_WARD.3,
            },
            LocationName::LumenflowerGardens => Location {
                name: "Lumenflower Gardens",
                region: "Yharnam Headstone",
                x: LUMENFLOWER_GARDENS.0,
                y: LUMENFLOWER_GARDENS.1,
                z: LUMENFLOWER_GARDENS.2,
                map_id: LUMENFLOWER_GARDENS.3,
            },
            LocationName::AltarOfDespair => Location {
                name: "Altar of Despair",
                region: "Yharnam Headstone",
                x: ALTAR_OF_DESPAIR.0,
                y: ALTAR_OF_DESPAIR.1,
                z: ALTAR_OF_DESPAIR.2,
                map_id: ALTAR_OF_DESPAIR.3,
            },
            LocationName::OldYharnam => Location {
                name: "Old Yharnam",
                region: "Yharnam Headstone",
                x: OLD_YHARNAM.0,
                y: OLD_YHARNAM.1,
                z: OLD_YHARNAM.2,
                map_id: OLD_YHARNAM.3,
            },
            LocationName::ChurchOfTheGoodChalice => Location {
                name: "Church of the Good Chalice",
                region: "Yharnam Headstone",
                x: CHURCH_OF_THE_GOOD_CHALICE.0,
                y: CHURCH_OF_THE_GOOD_CHALICE.1,
                z: CHURCH_OF_THE_GOOD_CHALICE.2,
                map_id: CHURCH_OF_THE_GOOD_CHALICE.3,
            },
            LocationName::GraveyardOfTheDarkbeast => Location {
                name: "Graveyard of the Darkbeast",
                region: "Yharnam Headstone",
                x: GRAVEYARD_OF_THE_DARKBEAST.0,
                y: GRAVEYARD_OF_THE_DARKBEAST.1,
                z: GRAVEYARD_OF_THE_DARKBEAST.2,
                map_id: GRAVEYARD_OF_THE_DARKBEAST.3,
            },

            // Frontier Headstone
            LocationName::HemwickCharnelLane => Location {
                name: "Hemwick Charnel Lane",
                region: "Frontier Headstone",
                x: HEMWICK_CHARNEL_LANE.0,
                y: HEMWICK_CHARNEL_LANE.1,
                z: HEMWICK_CHARNEL_LANE.2,
                map_id: HEMWICK_CHARNEL_LANE.3,
            },
            LocationName::WitchsAbode => Location {
                name: "Witch's Abode",
                region: "Frontier Headstone",
                x: WITCHS_ABODE.0,
                y: WITCHS_ABODE.1,
                z: WITCHS_ABODE.2,
                map_id: WITCHS_ABODE.3,
            },
            LocationName::ForbiddenWoods => Location {
                name: "Forbidden Woods",
                region: "Frontier Headstone",
                x: FORBIDDEN_WOODS.0,
                y: FORBIDDEN_WOODS.1,
                z: FORBIDDEN_WOODS.2,
                map_id: FORBIDDEN_WOODS.3,
            },
            LocationName::ForbiddenGrave => Location {
                name: "Forbidden Grave",
                region: "Frontier Headstone",
                x: FORBIDDEN_GRAVE.0,
                y: FORBIDDEN_GRAVE.1,
                z: FORBIDDEN_GRAVE.2,
                map_id: FORBIDDEN_GRAVE.3,
            },
            LocationName::Byrgenwerth => Location {
                name: "Byrgenwerth",
                region: "Frontier Headstone",
                x: BYRGENWERTH.0,
                y: BYRGENWERTH.1,
                z: BYRGENWERTH.2,
                map_id: BYRGENWERTH.3,
            },
            LocationName::MoonsideLake => Location {
                name: "Moonside Lake",
                region: "Frontier Headstone",
                x: MOONSIDE_LAKE.0,
                y: MOONSIDE_LAKE.1,
                z: MOONSIDE_LAKE.2,
                map_id: MOONSIDE_LAKE.3,
            },

            // Unseen Headstone
            LocationName::YahargulUnseenVillage => Location {
                name: "Yahar'gul, Unseen Village",
                region: "Unseen Headstone",
                x: YAHARGUL_UNSEEN_VILLAGE.0,
                y: YAHARGUL_UNSEEN_VILLAGE.1,
                z: YAHARGUL_UNSEEN_VILLAGE.2,
                map_id: YAHARGUL_UNSEEN_VILLAGE.3,
            },
            LocationName::YahargulChapel => Location {
                name: "Yahar'gul Chapel",
                region: "Unseen Headstone",
                x: YAHARGUL_CHAPEL.0,
                y: YAHARGUL_CHAPEL.1,
                z: YAHARGUL_CHAPEL.2,
                map_id: YAHARGUL_CHAPEL.3,
            },
            LocationName::AdventPlaza => Location {
                name: "Advent Plaza",
                region: "Unseen Headstone",
                x: ADVENT_PLAZA.0,
                y: ADVENT_PLAZA.1,
                z: ADVENT_PLAZA.2,
                map_id: ADVENT_PLAZA.3,
            },
            LocationName::HypogeanGaol => Location {
                name: "Hypogean Gaol",
                region: "Unseen Headstone",
                x: HYPOGEAN_GAOL.0,
                y: HYPOGEAN_GAOL.1,
                z: HYPOGEAN_GAOL.2,
                map_id: HYPOGEAN_GAOL.3,
            },
            LocationName::ForsakenCastleCainhurst => Location {
                name: "Forsaken Castle Cainhurst",
                region: "Unseen Headstone",
                x: FORSAKEN_CASTLE_CAINHURST.0,
                y: FORSAKEN_CASTLE_CAINHURST.1,
                z: FORSAKEN_CASTLE_CAINHURST.2,
                map_id: FORSAKEN_CASTLE_CAINHURST.3,
            },
            LocationName::LogariusSeat => Location {
                name: "Logarius' Seat",
                region: "Unseen Headstone",
                x: LOGARIUS_SEAT.0,
                y: LOGARIUS_SEAT.1,
                z: LOGARIUS_SEAT.2,
                map_id: LOGARIUS_SEAT.3,
            },
            LocationName::VilebloodQueensChamber => Location {
                name: "Vileblood Queen's Chamber",
                region: "Unseen Headstone",
                x: VILEBLOOD_QUEENS_CHAMBER.0,
                y: VILEBLOOD_QUEENS_CHAMBER.1,
                z: VILEBLOOD_QUEENS_CHAMBER.2,
                map_id: VILEBLOOD_QUEENS_CHAMBER.3,
            },
            LocationName::AbandonedOldWorkshop => Location {
                name: "Abandoned Old Workshop",
                region: "Unseen Headstone",
                x: ABANDONED_OLD_WORKSHOP.0,
                y: ABANDONED_OLD_WORKSHOP.1,
                z: ABANDONED_OLD_WORKSHOP.2,
                map_id: ABANDONED_OLD_WORKSHOP.3,
            },

            // Nightmare Headstone
            LocationName::LectureBuilding => Location {
                name: "Lecture Building",
                region: "Nightmare Headstone",
                x: LECTURE_BUILDING.0,
                y: LECTURE_BUILDING.1,
                z: LECTURE_BUILDING.2,
                map_id: LECTURE_BUILDING.3,
            },
            LocationName::LectureBuilding2ndFloor => Location {
                name: "Lecture Building 2nd Floor",
                region: "Nightmare Headstone",
                x: LECTURE_BUILDING_2ND_FLOOR.0,
                y: LECTURE_BUILDING_2ND_FLOOR.1,
                z: LECTURE_BUILDING_2ND_FLOOR.2,
                map_id: LECTURE_BUILDING_2ND_FLOOR.3,
            },
            LocationName::NightmareFrontier => Location {
                name: "Nightmare Frontier",
                region: "Nightmare Headstone",
                x: NIGHTMARE_FRONTIER.0,
                y: NIGHTMARE_FRONTIER.1,
                z: NIGHTMARE_FRONTIER.2,
                map_id: NIGHTMARE_FRONTIER.3,
            },
            LocationName::NightmareOfMensis => Location {
                name: "Nightmare of Mensis",
                region: "Nightmare Headstone",
                x: NIGHTMARE_OF_MENSIS.0,
                y: NIGHTMARE_OF_MENSIS.1,
                z: NIGHTMARE_OF_MENSIS.2,
                map_id: NIGHTMARE_OF_MENSIS.3,
            },
            LocationName::MergosLoftBase => Location {
                name: "Mergo's Loft: Base",
                region: "Nightmare Headstone",
                x: MERGOS_LOFT_BASE.0,
                y: MERGOS_LOFT_BASE.1,
                z: MERGOS_LOFT_BASE.2,
                map_id: MERGOS_LOFT_BASE.3,
            },
            LocationName::MergosLoftMiddle => Location {
                name: "Mergo's Loft: Middle",
                region: "Nightmare Headstone",
                x: MERGOS_LOFT_MIDDLE.0,
                y: MERGOS_LOFT_MIDDLE.1,
                z: MERGOS_LOFT_MIDDLE.2,
                map_id: MERGOS_LOFT_MIDDLE.3,
            },
            LocationName::WetNursesLunarium => Location {
                name: "Wet Nurse's Lunarium",
                region: "Nightmare Headstone",
                x: WET_NURSES_LUNARIUM.0,
                y: WET_NURSES_LUNARIUM.1,
                z: WET_NURSES_LUNARIUM.2,
                map_id: WET_NURSES_LUNARIUM.3,
            },

            // Hunter's Nightmare Headstone
            LocationName::HuntersNightmare => Location {
                name: "Hunter's Nightmare",
                region: "Hunter's Nightmare Headstone",
                x: HUNTERS_NIGHTMARE.0,
                y: HUNTERS_NIGHTMARE.1,
                z: HUNTERS_NIGHTMARE.2,
                map_id: HUNTERS_NIGHTMARE.3,
            },
            LocationName::NightmareChurch => Location {
                name: "Nightmare Church",
                region: "Hunter's Nightmare Headstone",
                x: NIGHTMARE_CHURCH.0,
                y: NIGHTMARE_CHURCH.1,
                z: NIGHTMARE_CHURCH.2,
                map_id: NIGHTMARE_CHURCH.3,
            },
            LocationName::NightmareGrandCathedral => Location {
                name: "Nightmare Grand Cathedral",
                region: "Hunter's Nightmare Headstone",
                x: NIGHTMARE_GRAND_CATHEDRAL.0,
                y: NIGHTMARE_GRAND_CATHEDRAL.1,
                z: NIGHTMARE_GRAND_CATHEDRAL.2,
                map_id: NIGHTMARE_GRAND_CATHEDRAL.3,
            },
            LocationName::UndergroundCorpsePile => Location {
                name: "Underground Corpse Pile",
                region: "Hunter's Nightmare Headstone",
                x: UNDERGROUND_CORPSE_PILE.0,
                y: UNDERGROUND_CORPSE_PILE.1,
                z: UNDERGROUND_CORPSE_PILE.2,
                map_id: UNDERGROUND_CORPSE_PILE.3,
            },
            LocationName::ResearchHall => Location {
                name: "Research Hall",
                region: "Hunter's Nightmare Headstone",
                x: RESEARCH_HALL.0,
                y: RESEARCH_HALL.1,
                z: RESEARCH_HALL.2,
                map_id: RESEARCH_HALL.3,
            },
            LocationName::LumenwoodGarden => Location {
                name: "Lumenwood Garden",
                region: "Hunter's Nightmare Headstone",
                x: LUMENWOOD_GARDEN.0,
                y: LUMENWOOD_GARDEN.1,
                z: LUMENWOOD_GARDEN.2,
                map_id: LUMENWOOD_GARDEN.3,
            },
            LocationName::AstralClocktower => Location {
                name: "Astral Clocktower",
                region: "Hunter's Nightmare Headstone",
                x: ASTRAL_CLOCKTOWER.0,
                y: ASTRAL_CLOCKTOWER.1,
                z: ASTRAL_CLOCKTOWER.2,
                map_id: ASTRAL_CLOCKTOWER.3,
            },
            LocationName::FishingHamlet => Location {
                name: "Fishing Hamlet",
                region: "Hunter's Nightmare Headstone",
                x: FISHING_HAMLET.0,
                y: FISHING_HAMLET.1,
                z: FISHING_HAMLET.2,
                map_id: FISHING_HAMLET.3,
            },
            LocationName::LighthouseHut => Location {
                name: "Lighthouse Hut",
                region: "Hunter's Nightmare Headstone",
                x: LIGHTHOUSE_HUT.0,
                y: LIGHTHOUSE_HUT.1,
                z: LIGHTHOUSE_HUT.2,
                map_id: LIGHTHOUSE_HUT.3,
            },
            LocationName::Coast => Location {
                name: "Coast",
                region: "Hunter's Nightmare Headstone",
                x: COAST.0,
                y: COAST.1,
                z: COAST.2,
                map_id: COAST.3,
            },
        }
    }

    /// Get all available location names
    pub fn all() -> &'static [LocationName] {
        &[
            // Hunter's Dream
            LocationName::HuntersDream,
            // Yharnam Headstone
            LocationName::FirstFloorSickroom,
            LocationName::CentralYharnam,
            LocationName::GreatBridge,
            LocationName::TombOfOedon,
            LocationName::CathedralWard,
            LocationName::GrandCathedralWard,
            LocationName::UpperCathedralWard,
            LocationName::LumenflowerGardens,
            LocationName::AltarOfDespair,
            LocationName::OldYharnam,
            LocationName::ChurchOfTheGoodChalice,
            LocationName::GraveyardOfTheDarkbeast,
            // Frontier Headstone
            LocationName::HemwickCharnelLane,
            LocationName::WitchsAbode,
            LocationName::ForbiddenWoods,
            LocationName::ForbiddenGrave,
            LocationName::Byrgenwerth,
            LocationName::MoonsideLake,
            // Unseen Headstone
            LocationName::YahargulUnseenVillage,
            LocationName::YahargulChapel,
            LocationName::AdventPlaza,
            LocationName::HypogeanGaol,
            LocationName::ForsakenCastleCainhurst,
            LocationName::LogariusSeat,
            LocationName::VilebloodQueensChamber,
            LocationName::AbandonedOldWorkshop,
            // Nightmare Headstone
            LocationName::LectureBuilding,
            LocationName::LectureBuilding2ndFloor,
            LocationName::NightmareFrontier,
            LocationName::NightmareOfMensis,
            LocationName::MergosLoftBase,
            LocationName::MergosLoftMiddle,
            LocationName::WetNursesLunarium,
            // Hunter's Nightmare Headstone
            LocationName::HuntersNightmare,
            LocationName::NightmareChurch,
            LocationName::NightmareGrandCathedral,
            LocationName::UndergroundCorpsePile,
            LocationName::ResearchHall,
            LocationName::LumenwoodGarden,
            LocationName::AstralClocktower,
            LocationName::FishingHamlet,
            LocationName::LighthouseHut,
            LocationName::Coast,
        ]
    }

    /// Get locations grouped by region
    pub fn by_region() -> &'static [(&'static str, &'static [LocationName])] {
        &[
            ("Hunter's Dream", &[LocationName::HuntersDream]),
            (
                "Yharnam Headstone",
                &[
                    LocationName::FirstFloorSickroom,
                    LocationName::CentralYharnam,
                    LocationName::GreatBridge,
                    LocationName::TombOfOedon,
                    LocationName::CathedralWard,
                    LocationName::GrandCathedralWard,
                    LocationName::UpperCathedralWard,
                    LocationName::LumenflowerGardens,
                    LocationName::AltarOfDespair,
                    LocationName::OldYharnam,
                    LocationName::ChurchOfTheGoodChalice,
                    LocationName::GraveyardOfTheDarkbeast,
                ],
            ),
            (
                "Frontier Headstone",
                &[
                    LocationName::HemwickCharnelLane,
                    LocationName::WitchsAbode,
                    LocationName::ForbiddenWoods,
                    LocationName::ForbiddenGrave,
                    LocationName::Byrgenwerth,
                    LocationName::MoonsideLake,
                ],
            ),
            (
                "Unseen Headstone",
                &[
                    LocationName::YahargulUnseenVillage,
                    LocationName::YahargulChapel,
                    LocationName::AdventPlaza,
                    LocationName::HypogeanGaol,
                    LocationName::ForsakenCastleCainhurst,
                    LocationName::LogariusSeat,
                    LocationName::VilebloodQueensChamber,
                    LocationName::AbandonedOldWorkshop,
                ],
            ),
            (
                "Nightmare Headstone",
                &[
                    LocationName::LectureBuilding,
                    LocationName::LectureBuilding2ndFloor,
                    LocationName::NightmareFrontier,
                    LocationName::NightmareOfMensis,
                    LocationName::MergosLoftBase,
                    LocationName::MergosLoftMiddle,
                    LocationName::WetNursesLunarium,
                ],
            ),
            (
                "Hunter's Nightmare Headstone",
                &[
                    LocationName::HuntersNightmare,
                    LocationName::NightmareChurch,
                    LocationName::NightmareGrandCathedral,
                    LocationName::UndergroundCorpsePile,
                    LocationName::ResearchHall,
                    LocationName::LumenwoodGarden,
                    LocationName::AstralClocktower,
                    LocationName::FishingHamlet,
                    LocationName::LighthouseHut,
                    LocationName::Coast,
                ],
            ),
        ]
    }
}
