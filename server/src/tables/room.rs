#[spacetimedb::table(name = room)]
pub struct Room {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub region_id: u64,

    pub name: String,
    pub description: String,

    pub north_exit: Option<u64>,
    pub south_exit: Option<u64>,
    pub east_exit: Option<u64>,
    pub west_exit: Option<u64>,
    pub up_exit: Option<u64>,
    pub down_exit: Option<u64>,
    pub has_special_exits: bool,

    pub temperature_modifier: i16,
    pub light_modifier: i16,
    pub is_safe_zone: bool,
    pub allows_combat: bool,
    pub allows_magic: bool,

    pub current_volume: Option<f32>,
    pub max_volume: Option<f32>,

    pub is_active: bool,
}
