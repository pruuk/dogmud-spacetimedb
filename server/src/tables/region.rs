use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = region)]
pub struct Region {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    pub name: String,
    pub description: String,

    pub biome: BiomeType,
    pub climate: ClimateType,
    pub base_temperature: i16,
    pub base_light_level: u8,

    pub default_spawn_room: u64,

    pub is_active: bool,
    pub tick_rate_fast: u32,
    pub tick_rate_medium: u32,

    pub min_x: Option<f32>,
    pub max_x: Option<f32>,
    pub min_y: Option<f32>,
    pub max_y: Option<f32>,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum BiomeType {
    Forest,
    Desert,
    Tundra,
    Swamp,
    Mountain,
    Plains,
    Ocean,
    Underground,
    City,
    Dungeon,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ClimateType {
    Tropical,
    Temperate,
    Arctic,
    Arid,
    Magical,
}
