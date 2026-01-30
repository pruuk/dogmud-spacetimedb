use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = npc_behavior)]
pub struct NPCBehavior {
    #[primary_key]
    pub entity_id: u64,

    pub ai_type: AIType,
    pub aggro_range: u8,
    pub wander_range: u8,
    pub home_room: u64,

    pub role: Option<NPCRole>,

    pub movement_type: MovementType,
    pub patrol_waypoints: String,
    pub movement_speed: f32,

    pub faction: Option<String>,
    pub assist_allies: bool,

    pub respawn_delay: u32,
    pub is_unique: bool,

    pub can_talk: bool,
    pub dialogue_tree_id: Option<u64>,

    pub loot_table_id: Option<u64>,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum AIType {
    Passive,
    Defensive,
    Aggressive,
    Territorial,
    Timid,
    Berserk,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum NPCRole {
    Shopkeeper,
    Banker,
    Questgiver,
    Guard,
    Trainer,
    Innkeeper,
    Blacksmith,
    Alchemist,
    Monster,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum MovementType {
    Stationary,
    Wander,
    Patrol,
    Chase,
}
