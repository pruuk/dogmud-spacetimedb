use spacetimedb::{Identity, SpacetimeType};

#[spacetimedb::table(name = entity)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    pub identity: Option<Identity>,

    #[index(btree)]
    pub entity_type: EntityType,

    pub name: String,
    pub description: String,

    #[index(btree)]
    pub room_id: u64,

    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub volume: f32,
    pub weight: f32,
    pub max_capacity: f32,
    pub hp: i32,
    pub max_hp: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub mana: i32,
    pub max_mana: i32,
    pub dexterity: u8,
    pub strength: u8,
    pub vitality: u8,
    pub perception: u8,
    pub willpower: u8,
    pub is_alive: bool,
    pub is_active: bool,
    pub created_at: i64,
    pub last_action_at: i64,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    Player,
    NPC,
    Item,
    Container,
    Fixture,
}
