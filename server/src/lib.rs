use spacetimedb::{reducer, table, Identity, ReducerContext, SpacetimeType, Table};

mod utils;

// Table definitions
#[spacetimedb::table(name=entity)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    pub identity: Option<Identity>,
    pub entity_type: EntityType,
    pub name: String,
    pub description: String,

    // Spatial
    pub room_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,

    // Physical
    pub volume: f32,
    pub weight: f32,
    pub max_capacity: f32,

    // Resources
    pub hp: i32,
    pub max_hp: i32,
    pub stamina: i32,
    pub max_stamina: i32,
    pub mana: i32,
    pub max_mana: i32,

    // The Five Stats
    pub dexterity: u8,
    pub strength: u8,
    pub vitality: u8,
    pub perception: u8,
    pub willpower: u8,

    // State
    pub is_alive: bool,
    pub is_active: bool,

    // Metadata
    pub created_at: u64,
    pub last_action_at: u64,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    Player,
    NPC,
    Item,
    Container,
    Fixture,
}

#[reducer]
pub fn create_test_entity(ctx: &ReducerContext, name: String) {
    log::info!("Creating test entity: {}", name);

    let new_entity = Entity {
        id: 0,
        identity: None,
        entity_type: EntityType::Player,
        name,
        description: "A test entity".to_string(),
        room_id: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        volume: 1.0,
        weight: 70.0,
        max_capacity: 50.0,
        hp: 100,
        max_hp: 100,
        stamina: 100,
        max_stamina: 100,
        mana: 100,
        max_mana: 100,
        dexterity: 100,
        strength: 100,
        vitality: 100,
        perception: 100,
        willpower: 100,
        is_alive: true,
        is_active: true,
        created_at: 0,
        last_action_at: 0,
    };

    match ctx.db.entity().try_insert(new_entity) {
        Ok(_) => log::info!("Entity inserted successfully!"),
        Err(err) => log::error!("Failed to insert entity: {:?}", err),
    }

    log::info!("Entity inserted successfully!");
}
