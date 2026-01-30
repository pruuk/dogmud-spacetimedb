use spacetimedb::{reducer, table, Identity, ReducerContext, SpacetimeType, Table};

mod utils;

// Table definitions
#[table(name = entity)]
pub struct Entity {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub identity: Option<Identity>,

    // adding indexes - creates and index on entity_type for faster queries
    // of something like Get all entities in room #5
    // index by entity_type
    #[index(btree)]
    pub entity_type: EntityType,
    pub name: String,
    pub description: String,

    // indexing by room
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

#[table(name = condition)]
#[derive(Clone, Debug)]
pub struct Condition {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub condition_type: ConditionType,
    pub magnitude: f32,
    pub remaining_ticks: i32,
    pub source_id: Option<u64>,
    pub applied_at: u64,

    // index on entity ID
    #[index(btree)]
    pub entity_id: u64,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ConditionType {
    // Damage over time conditions
    Burning,
    Poisoned,
    Diseased,
    Bleeding,
    Choking,

    // healing over time conditions
    Regenerating,
    Meditating,
    WellRested,

    // Impairing Conditions
    Shocked,
    Frozen,
    Paralzed,
    Dazed,
    Inebriated,
    Comatose, // Used when Health or Stamina reaches zero
    Blinded,
    Encumbered,
    Exhausted,
    Bound,
    Grappled,

    // Positional Conditions
    Sitting,
    Prone,
    Supine,

    // Environmental States
    Wet,
    Muddy,
    Oiled,
}

#[table(name = skill)]
#[derive(Clone, Debug)]
pub struct Skill {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub skill_type: SkillType,
    pub level: u8,
    pub last_used: u64,
    pub times_used: i32,

    // indexing by entity ID
    #[index(btree)]
    pub entity_id: u64,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum SkillType {
    MeleeCombat,
    RangedCombat,
    MagicCasting,
    Tracking,
    Stealth,
    Blacksmithing,
    Hidemaking,
    Bowyery,
    Alchemy,
    Cooking,
    Haggling,
    Climbing,
    // TODO: Fill in more skills as we expand the game and think of ideas for more skills
}

#[table(name = room)]
#[derive(Clone, Debug)]
pub struct Room {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub name: String,
    description: String,
    max_volume: f32,
    current_volume: f32,
    // Fast common exits
    pub north_exit: Option<u64>,
    pub south_exit: Option<u64>,
    pub east_exit: Option<u64>,
    pub west_exit: Option<u64>,
    pub up_exit: Option<u64>,
    pub down_exit: Option<u64>,
    // NOTE: special case exits will require query, will be slower
    pub has_special_exits: bool, // Flag to check Exit table
    light_level: u8,
    temperature: i32,
    is_safe_zone: bool,
    last_player_visit: i32,
    item_count: i32,
    // index by region ID for fast queries like all rooms in a region
    #[index(btree)]
    pub region_id: u64,
}

// For special cases only
// this handles non-cardinal direction exits like enter portal/trapdoor
#[spacetimedb::table(name = exit)]
pub struct Exit {
    pub from_room: u64,
    pub to_room: u64,
    pub direction: String, // "portal", "trapdoor", "northwest"
    pub name: String,
    pub is_hidden: bool,
    pub is_locked: bool,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum TerrainType {
    Indoor,

    // Outdoor terrain types in ascending order or difficulty to pass through
    Flat,
    Sloped,
    Rugged,
    Difficult,
    Challenging,
    Impossible,
}

#[table(name = game_event)]
#[derive(Clone, Debug)]
pub struct GameEvent {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    // index by room_id and timestamp
    #[index(btree)]
    pub room_id: u64,
    #[index(btree)]
    pub timestamp: u64,

    event_type: EventType,
    event_data: String,
    pub primary_actor: u64,
    pub secondary_actor: Option<u64>,

    pub requires_sight: bool,
    pub requires_hearing: bool,
    pub stealth_dc: Option<u8>,

    pub expires_at: u64,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum EventType {
    Combat,
    Movement,
    Speech,
    Emote,
    ItemInteraction,
    ConditionChange,
    Environmental,
    System,
    Economy,
    // TODO: Add more event types as we flesh this out
}

#[spacetimedb::table(name = bank_account)]
pub struct BankAccount {
    #[primary_key]
    pub identity: Identity,

    pub balance: u64,
    pub created_at: u64,
    pub last_transaction: u64,
}

#[spacetimedb::table(name = item_data)]
pub struct ItemData {
    #[primary_key]
    pub entity_id: u64,

    pub item_type: ItemType,

    pub quantity: u32,
    pub max_stack: u32,

    pub base_damage: u16,
    pub damage_type: DamageType,
    pub attack_speed: f32,

    pub armor_rating: u16,
    pub armor_type: ArmorType,

    pub internal_volume: f32,
    pub weight_reduction: f32,

    pub durability: u16,
    pub max_durability: u16,
    pub is_equipped: bool,
    pub equipped_slot: Option<EquipSlot>,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Container,
    Consumable,
    Tool,
    QuestItem,
    Gold,
    Junk,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Ice,
    Lightning,
    Acid,
    Poison,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ArmorType {
    Cloth,
    Leather,
    Chain,
    Plate,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum EquipSlot {
    Head,
    Torso,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
    TwoHand,
    Neck,
    Ring,
    Face,
}

#[spacetimedb::table(name = region)]
pub struct Region {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    pub name: String,
    pub description: String,

    // Environmental
    pub biome: BiomeType,
    pub climate: ClimateType,
    pub base_temperature: i16, // Celsius
    pub base_light_level: u8,  // 0-255

    // For spawning/respawning
    pub default_spawn_room: u64, // Where players spawn in this region

    // Metadata for shepherds
    pub is_active: bool,       // Is RegionShepherd running for this?
    pub tick_rate_fast: u32,   // Custom tick rate (default 1s)
    pub tick_rate_medium: u32, // Custom tick rate (default 5s)

    // Optional bounds (for world map display)
    pub min_x: Option<f32>,
    pub max_x: Option<f32>,
    pub min_y: Option<f32>,
    pub max_y: Option<f32>,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum BiomeType {
    Forest,
    Desert,
    Swamp,
    Mountain,
    Plains,
    Ocean,
    Freshwater,
    Underground,
    City,
    Dungeon,
    Cave,
    Canyon,
    Steppe,
    Grasslands,
    Shrublands,
    Aerial,
    Planar,
    Coastal,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ClimateType {
    Tropical,
    Temperate,
    Boreal,
    Montane,
    Tundra,
    Arid,
    Mangrove,
    Magical, // For weird zones
}

#[spacetimedb::table(name = npc_behavior)]
pub struct NPCBehavior {
    #[primary_key]
    pub entity_id: u64, // One-to-one with Entity

    // AI Behavior
    pub ai_type: AIType,
    pub aggro_range: u8,  // How far they detect players
    pub wander_range: u8, // How far from home they roam
    pub home_room: u64,   // Where they spawn/return

    // Role/Function
    pub role: Option<NPCRole>, // What they do (can be None for basic mobs)

    // Patrol/Movement
    pub movement_type: MovementType,
    pub patrol_waypoints: String, // JSON array of room IDs for patrol routes
    pub movement_speed: f32,

    // Combat
    pub faction: Option<String>, // "Goblin Tribe", "City Guard", etc.
    pub assist_allies: bool,     // Help nearby same-faction NPCs

    // Respawn
    pub respawn_delay: u32, // Seconds until respawn
    pub is_unique: bool,    // Unique NPCs don't respawn

    // Conversation
    pub can_talk: bool,
    pub dialogue_tree_id: Option<u64>, // Link to dialogue system (future)

    // Loot
    pub loot_table_id: Option<u64>, // What they drop (future)
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum AIType {
    Passive,     // Never attacks
    Defensive,   // Attacks if attacked
    Aggressive,  // Attacks on sight
    Territorial, // Attacks if you enter their area
    Timid,       // Runs away when attacked
    Berserk,     // Always attacks, never stops
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum NPCRole {
    Shopkeeper,
    Banker,
    Questgiver,
    Guard,
    Trainer, // Teaches skills
    Innkeeper,
    Blacksmith,
    Alchemist,
    Monster, // Just a mob
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum MovementType {
    Stationary, // Never moves
    Wander,     // Random movement in range
    Patrol,     // Follows waypoints
    Chase,      // Pursues target
}

/*
Example NPCs using the NPCBehavior Table

** AGGRESSIVE GOBLIN **
NPCBehavior {
    entity_id: goblin_id,
    ai_type: AIType::Aggressive,
    aggro_range: 3,
    wander_range: 5,
    home_room: goblin_cave_id,
    role: Some(NPCRole::Monster),
    movement_type: MovementType::Wander,
    // ...
}

** TOWN SHOPKEEPER **
NPCBehavior {
    entity_id: shopkeeper_id,
    ai_type: AIType::Passive,
    aggro_range: 0,
    wander_range: 0,
    home_room: shop_id,
    role: Some(NPCRole::Shopkeeper),
    movement_type: MovementType::Stationary,
    can_talk: true,
    // ...
}

** PATROLLING GUARD **
NPCBehavior {
    entity_id: guard_id,
    ai_type: AIType::Defensive,
    aggro_range: 5,
    home_room: barracks_id,
    role: Some(NPCRole::Guard),
    movement_type: MovementType::Patrol,
    patrol_waypoints: "[101, 102, 103, 104, 101]",  // Square patrol
    faction: Some("City Guard".to_string()),
    assist_allies: true,
    // ...
}

*/

// Table to track containment - what is inside what containers
#[spacetimedb::table(name = containment)]
pub struct Containment {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub container_id: u64, // The Entity that holds items (bag, chest, room)

    #[index(btree)]
    pub contained_id: u64, // The Entity being held (item, smaller container)

    pub depth: u8,              // Nesting depth (prevent infinite bags)
    pub slot_index: Option<u8>, // For organized containers (slot 1, 2, 3...)
}

/*

**Example**: Sword in backpack in room

container_id: backpack_id, contained_id: sword_id, depth: 2
container_id: room_id, contained_id: backpack_id, depth: 1
*/

// player account info table
#[spacetimedb::table(name = account)]
pub struct Account {
    #[primary_key]
    pub identity: Identity, // SpacetimeDB identity (unique per user)

    pub username: String,
    pub password_hash: String, // NEVER store plaintext!
    pub email: Option<String>,

    pub created_at: u64,
    pub last_login: u64,
    pub total_play_time: u64, // Seconds

    pub is_banned: bool,
    pub is_admin: bool,
    pub is_moderator: bool,

    pub primary_character_id: Option<u64>, // Their main Entity
}

// player session table - tracks active connections
#[spacetimedb::table(name = player_session)]
pub struct PlayerSession {
    #[primary_key]
    pub identity: Identity,

    pub character_id: u64, // Which Entity they're playing as
    pub connected_at: u64,
    pub last_heartbeat: u64, // For timeout detection

    pub client_type: ClientType,
    pub client_version: String,

    pub is_active: bool, // False = AFK/link-dead
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ClientType {
    RatatuiTUI,
    LeptosWeb,
    TelnetBridge,
    Unknown,
}

// various metadata tables for utility

// ratelimit table to help track spam/abuse
#[spacetimedb::table(name = rate_limit)]
pub struct RateLimit {
    #[primary_key]
    pub identity: Identity,

    pub action_type: ActionType,
    pub count: u32,        // Times used in current window
    pub window_start: u64, // When this window started
    pub last_action: u64,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum ActionType {
    Attack,
    Move,
    Speech,
    Command,
    Trade,
}

/*
Example usage:

// Check if player can attack (max 2/sec)
let can_attack = check_rate_limit(ctx, identity, ActionType::Attack, 2, 1);

*/

// config table for server settings
#[spacetimedb::table(name = server_config)]
pub struct ServerConfig {
    #[primary_key]
    pub key: String, // "max_players", "tick_rate_fast", "pvp_enabled"

    pub value: String, // Store as string, parse as needed
    pub last_updated: u64,
    pub updated_by: Option<Identity>,
}

// Reducers
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
