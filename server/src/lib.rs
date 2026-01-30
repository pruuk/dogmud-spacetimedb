use spacetimedb::{reducer, table, Identity, ReducerContext, SpacetimeType, Table};

mod utils;

// Stats calculations (copied from dogmud-common to avoid getrandom in WASM)
mod combat_stats {

    use spacetimedb::rand::Rng;

    pub fn calculate_roll_base(stat: u8, skill: u8, modifier: f32) -> f32 {
        let stat_contribution = stat as f32 * 0.7;
        let skill_contribution = skill as f32 * 0.3;
        (stat_contribution + skill_contribution) * modifier
    }

    pub fn calculate_std_dev(mean: f32) -> f32 {
        mean * 0.15
    }

    // Simplified: just add some randomness without normal distribution
    pub fn random_variance(base: f32, ctx: &spacetimedb::ReducerContext) -> f32 {
        // use rand::Rng;
        let variance = calculate_std_dev(base);
        let min = base - variance;
        let max = base + variance;
        ctx.rng().gen_range(min..=max)
    }

    pub fn is_critical_hit(attacker_roll: f32, defender_roll: f32) -> bool {
        attacker_roll >= defender_roll * 1.2
    }

    pub fn is_critical_fail(attacker_roll: f32, defender_roll: f32) -> bool {
        attacker_roll <= defender_roll * 0.7
    }
}

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
    pub applied_at: i64,

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
    pub last_used: i64,
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
    pub current_volume: Option<f32>,
    pub max_volume: Option<f32>,
    // Fast common exits
    pub north_exit: Option<u64>,
    pub south_exit: Option<u64>,
    pub east_exit: Option<u64>,
    pub west_exit: Option<u64>,
    pub up_exit: Option<u64>,
    pub down_exit: Option<u64>,
    // NOTE: special case exits will require query, will be slower
    pub has_special_exits: bool, // Flag to check Exit table
    // Environment (relative to region)
    pub temperature_modifier: i16,  // +/- from region.base_temperature, use 0 for normal
    pub light_modifier: i16,        // +/- from region.base_light_level, use 0 for normal
    pub is_safe_zone: bool,
    pub allows_combat: bool,
    pub allows_magic: bool,
    last_player_visit: i32,
    item_count: i32,
    // index by region ID for fast queries like all rooms in a region
    #[index(btree)]
    pub region_id: u64,
    // Metadata
    pub is_active: bool,
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
    pub timestamp: i64,

    event_type: EventType,
    event_data: String,
    pub primary_actor: u64,
    pub secondary_actor: Option<u64>,

    pub requires_sight: bool,
    pub requires_hearing: bool,
    pub stealth_dc: Option<u8>,

    pub expires_at: i64,
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
    pub created_at: i64,
    pub last_transaction: i64,
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

    pub created_at: i64,
    pub last_login: i64,
    pub total_play_time: i64, // Seconds

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
    pub connected_at: i64,
    pub last_heartbeat: i64, // For timeout detection

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
    pub window_start: i64, // When this window started
    pub last_action: i64,
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
    pub last_updated: i64,
    pub updated_by: Option<Identity>,
}

// Reducers for testing
// TODO: Move these out to separate folders once we figure out how to get everything working
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


#[reducer]
pub fn create_test_room(ctx: &ReducerContext) {
    log::info!("Creating test room");
    
    let test_room = Room {
        id: 0,  // Auto-increment
        region_id: 1,  // We'll need a test region too
        name: "The Starting Chamber".to_string(),
        description: "A dimly lit stone chamber. Torches flicker on the walls, casting dancing shadows. You see exits to the north, south, east, and west.".to_string(),
        
        // No exits yet (we'll add more rooms later)
        north_exit: None,
        south_exit: None,
        east_exit: None,
        west_exit: None,
        up_exit: None,
        down_exit: None,
        has_special_exits: false,
        current_volume: None, // not tracking volume, room is effectively inifinite size for testing
        max_volume: None, // unlimited space
        item_count: 0,
        last_player_visit: 0,
        
        // Environment
        temperature_modifier: 2,  // Slightly warmer than region default
        light_modifier: 64,       // Brighter than region (torches on walls)
        is_safe_zone: true,     // Can't attack here
        allows_combat: false,
        allows_magic: true,
        
        // Metadata
        is_active: true,
    };
    
    match ctx.db.room().try_insert(test_room) {
        Ok(_) => log::info!("Test room created successfully!"),
        Err(err) => log::error!("Failed to create test room: {:?}", err),
    }
}

#[reducer]
pub fn create_room_grid(
    ctx: &ReducerContext,
    center_x: i32,
    center_y: i32,
    size: u32,
) -> Result<(), String> {
    log::info!("Creating {}x{} room grid centered at ({}, {})", size, size, center_x, center_y);
    
    if size > 20 {
        return Err("Grid size too large (max 20x20)".to_string());
    }
    
    let half_size = (size / 2) as i32;
    let mut room_ids: std::collections::HashMap<(i32, i32), u64> = std::collections::HashMap::new();
    
    // First pass: Create all rooms
    for x in (center_x - half_size)..=(center_x + half_size) {
        for y in (center_y - half_size)..=(center_y + half_size) {
            let room = Room {
                id: 0,  // Auto-increment
                region_id: 1,
                name: format!("Room [{}, {}]", x, y),
                description: format!("A stone chamber at coordinates ({}, {}). Exits lead in the cardinal directions.", x, y),
                
                // Will link exits in second pass
                north_exit: None,
                south_exit: None,
                east_exit: None,
                west_exit: None,
                up_exit: None,
                down_exit: None,
                has_special_exits: false,
                
                // Environment
                temperature_modifier: 0,
                light_modifier: 50,
                is_safe_zone: false,
                allows_combat: true,
                allows_magic: true,
                
                // Volume (optional, so None)
                current_volume: None,
                max_volume: None,

                // additonal fields
                item_count: 0,
                last_player_visit: 0,
                
                // Metadata
                is_active: true,
            };
            
            let inserted = ctx.db.room().try_insert(room)
                .map_err(|e| format!("Failed to create room at ({}, {}): {:?}", x, y, e))?;
            
            room_ids.insert((x, y), inserted.id);
            log::info!("Created room {} at ({}, {})", inserted.id, x, y);
        }
    }
    
    // Second pass: Link rooms with exits
    for x in (center_x - half_size)..=(center_x + half_size) {
        for y in (center_y - half_size)..=(center_y + half_size) {
            let current_id = room_ids.get(&(x, y)).unwrap();
            let mut room = ctx.db.room().id().find(current_id).unwrap();
            
            // Link north (y+1)
            if let Some(&north_id) = room_ids.get(&(x, y + 1)) {
                room.north_exit = Some(north_id);
            }
            
            // Link south (y-1)
            if let Some(&south_id) = room_ids.get(&(x, y - 1)) {
                room.south_exit = Some(south_id);
            }
            
            // Link east (x+1)
            if let Some(&east_id) = room_ids.get(&(x + 1, y)) {
                room.east_exit = Some(east_id);
            }
            
            // Link west (x-1)
            if let Some(&west_id) = room_ids.get(&(x - 1, y)) {
                room.west_exit = Some(west_id);
            }
            
            // Update the room with exits
            ctx.db.room().id().update(room);
        }
    }
    
    log::info!("Room grid created successfully! Total rooms: {}", room_ids.len());
    Ok(())
}

#[reducer]
pub fn create_test_region(ctx: &ReducerContext) {
    log::info!("Creating test region");
    
    let test_region = Region {
        id: 0,  // Auto-increment
        name: "Test Dungeon".to_string(),
        description: "A small dungeon for testing game mechanics.".to_string(),
        
        biome: BiomeType::Dungeon,
        climate: ClimateType::Temperate,
        base_temperature: 18,
        base_light_level: 64,  // Dim
        
        default_spawn_room: 1,  // Will be the room we create
        
        is_active: true,
        tick_rate_fast: 1000,   // 1 second
        tick_rate_medium: 5000, // 5 seconds
        
        // No bounds for now
        min_x: None,
        max_x: None,
        min_y: None,
        max_y: None,
    };
    
    match ctx.db.region().try_insert(test_region) {
        Ok(_) => log::info!("Test region created successfully!"),
        Err(err) => log::error!("Failed to create test region: {:?}", err),
    }
}

#[reducer]
pub fn create_account(
    ctx: &ReducerContext,
    username: String,
    password: String,
) -> Result<(), String> {
    log::info!("Creating account: {}", username);
    
    // Validate password length
    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }
    
    // Check if username already exists
    let existing = ctx.db
        .account()
        .iter()
        .any(|acc| acc.username == username);
    
    if existing {
        return Err(format!("Username '{}' already taken", username));
    }
    
    // Hash password with Argon2
    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };
    
    // Generate deterministic salt from identity + timestamp
    // This is unique per account and deterministic for replication
    let salt_source = format!("{:?}{}", ctx.sender, ctx.timestamp.to_micros_since_unix_epoch());
    let salt_bytes: [u8; 16] = {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(salt_source.as_bytes());
        let result = hasher.finalize();
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&result[0..16]);
        bytes
    };
    
    // Convert to base64 for SaltString
    use base64::{Engine as _, engine::general_purpose};
    let salt_b64 = general_purpose::STANDARD_NO_PAD.encode(&salt_bytes);
    let salt = SaltString::from_b64(&salt_b64)
        .map_err(|e| format!("Failed to create salt: {}", e))?;
    
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Failed to hash password: {}", e))?
        .to_string();
    
    let account = Account {
        identity: ctx.sender,
        username,
        password_hash,
        email: None,
        created_at: ctx.timestamp.to_micros_since_unix_epoch(),
        last_login: ctx.timestamp.to_micros_since_unix_epoch(),
        total_play_time: 0,
        is_banned: false,
        is_admin: false,
        is_moderator: false,
        primary_character_id: None,
    };
    
    ctx.db.account().try_insert(account)
        .map_err(|e| format!("Failed to create account: {:?}", e))?;
    
    log::info!("Account created successfully");
    Ok(())
}

#[reducer]
pub fn login(
    ctx: &ReducerContext,
    username: String,
    password: String,
) -> Result<(), String> {
    log::info!("Login attempt: {}", username);
    
    // Find account
    let account = ctx.db
        .account()
        .iter()
        .find(|acc| acc.username == username)
        .ok_or("Invalid username or password")?;
    
    // Check if banned
    if account.is_banned {
        return Err("Account is banned".to_string());
    }
    
    // Verify password with Argon2
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    
    let parsed_hash = PasswordHash::new(&account.password_hash)
        .map_err(|e| format!("Invalid password hash: {}", e))?;
    
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "Invalid username or password".to_string())?;
    
    // Create player session
    let session = PlayerSession {
        identity: ctx.sender,
        character_id: account.primary_character_id.unwrap_or(0),  // 0 = no character yet
        connected_at: ctx.timestamp.to_micros_since_unix_epoch(),
        last_heartbeat: ctx.timestamp.to_micros_since_unix_epoch(),
        client_type: ClientType::Unknown,
        client_version: "0.1.0".to_string(),
        is_active: true,
    };
    
    ctx.db.player_session().try_insert(session)
        .map_err(|e| format!("Failed to create session: {:?}", e))?;
    
    log::info!("Login successful for: {}", username);
    Ok(())
}

#[reducer]
pub fn create_character(
    ctx: &ReducerContext,
    name: String,
) -> Result<(), String> {
    log::info!("Creating character: {}", name);
    
    // Check if account exists
    let account = ctx.db
        .account()
        .identity()
        .find(&ctx.sender)
        .ok_or("No account found. Create account first.")?;
    
    // Check if character name is taken
    let name_taken = ctx.db
        .entity()
        .iter()
        .any(|e| e.entity_type == EntityType::Player && e.name == name);
    
    if name_taken {
        return Err(format!("Character name '{}' already taken", name));
    }
    
    // Create the character entity
    let character = Entity {
        id: 0,
        identity: Some(ctx.sender),
        entity_type: EntityType::Player,
        name,
        description: "A brave adventurer.".to_string(),
        room_id: 1,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        volume: 70.0,
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
        created_at: ctx.timestamp.to_micros_since_unix_epoch(),
        last_action_at: ctx.timestamp.to_micros_since_unix_epoch(),
    };
    
    let inserted = ctx.db.entity().try_insert(character)
        .map_err(|e| format!("Failed to create character: {:?}", e))?;
    
    // Update account with primary character ID
    let mut updated_account = account;
    updated_account.primary_character_id = Some(inserted.id);
    ctx.db.account().identity().update(updated_account);
    
    log::info!("Character created successfully!");
    Ok(())
}

#[reducer]
pub fn move_player(
    ctx: &ReducerContext,
    direction: String,
) -> Result<(), String> {
    log::info!("Player attempting to move: {}", direction);
    
    // Get the player's session
    let session = ctx.db
        .player_session()
        .identity()
        .find(&ctx.sender)
        .ok_or("Not logged in")?;
    
    if session.character_id == 0 {
        return Err("No character selected".to_string());
    }
    
    // Get the player's entity
    let mut player = ctx.db
        .entity()
        .id()
        .find(&session.character_id)
        .ok_or("Character not found")?;
    
    // Check if player is alive
    if !player.is_alive {
        return Err("You are dead and cannot move".to_string());
    }
    
    // Check if player is in comatose condition
    let is_comatose = ctx.db
        .condition()
        .entity_id()
        .filter(&player.id)
        .any(|c| c.condition_type == ConditionType::Comatose);
    
    if is_comatose {
        return Err("You are comatose and cannot move".to_string());
    }
    
    // Get current room
    let current_room = ctx.db
        .room()
        .id()
        .find(&player.room_id)
        .ok_or("Current room not found")?;
    
    // Determine target room based on direction
    let target_room_id = match direction.to_lowercase().as_str() {
        "north" | "n" => current_room.north_exit,
        "south" | "s" => current_room.south_exit,
        "east" | "e" => current_room.east_exit,
        "west" | "w" => current_room.west_exit,
        "up" | "u" => current_room.up_exit,
        "down" | "d" => current_room.down_exit,
        _ => return Err(format!("Invalid direction: {}", direction)),
    };
    
    let target_room_id = target_room_id
        .ok_or(format!("There is no exit to the {}", direction))?;
    
    // Get target room (to verify it exists and is active)
    let target_room = ctx.db
        .room()
        .id()
        .find(&target_room_id)
        .ok_or("Target room not found")?;
    
    if !target_room.is_active {
        return Err("That passage is blocked".to_string());
    }
    
    // Save data for logging before moving player
    let player_name = player.name.clone();  // Clone the name before update
    let player_id = player.id;
    let old_room_id = player.room_id;
    
    // Move the player
    player.room_id = target_room_id;
    player.last_action_at = ctx.timestamp.to_micros_since_unix_epoch();
    
    ctx.db.entity().id().update(player);  // player is moved here
    
    // Create movement event
    let event = GameEvent {
        id: 0,
        room_id: old_room_id,
        timestamp: ctx.timestamp.to_micros_since_unix_epoch(),
        event_type: EventType::Movement,
        event_data: format!("{{\"entity_id\": {}, \"direction\": \"{}\", \"from_room\": {}, \"to_room\": {}}}", 
            player_id, direction, old_room_id, target_room_id),
        primary_actor: player_id,
        secondary_actor: None,
        requires_sight: true,
        requires_hearing: false,
        stealth_dc: None,
        expires_at: ctx.timestamp.to_micros_since_unix_epoch() + 60_000_000,
    };
    
    ctx.db.game_event().try_insert(event)
        .map_err(|e| format!("Failed to create event: {:?}", e))?;
    
    log::info!("Player {} moved {} from room {} to room {}", 
        player_name, direction, old_room_id, target_room_id);
    
    Ok(())
}

#[reducer]
pub fn attack(
    ctx: &ReducerContext,
    target_id: u64,
) -> Result<(), String> {
    log::info!("Attack initiated against target {}", target_id);
    
    // Get attacker's session
    let session = ctx.db
        .player_session()
        .identity()
        .find(&ctx.sender)
        .ok_or("Not logged in")?;
    
    if session.character_id == 0 {
        return Err("No character selected".to_string());
    }
    
    // Get attacker
    let mut attacker = ctx.db
        .entity()
        .id()
        .find(&session.character_id)
        .ok_or("Attacker not found")?;
    
    // Get target
    let mut target = ctx.db
        .entity()
        .id()
        .find(&target_id)
        .ok_or("Target not found")?;
    
    // Validation checks
    if !attacker.is_alive {
        return Err("You are dead".to_string());
    }
    
    if !target.is_alive {
        return Err("Target is already dead".to_string());
    }
    
    if attacker.id == target.id {
        return Err("You cannot attack yourself".to_string());
    }
    
    if attacker.room_id != target.room_id {
        return Err("Target is not in the same room".to_string());
    }
    
    // Check if room allows combat
    let room = ctx.db
        .room()
        .id()
        .find(&attacker.room_id)
        .ok_or("Room not found")?;
    
    if !room.allows_combat {
        return Err("Combat is not allowed here".to_string());
    }
    
    // Check stamina
    if attacker.stamina < 10 {
        return Err("Not enough stamina to attack".to_string());
    }
    
    // Use the stats system from dogmud-common
    // Use our local combat_stats module
    // In attack reducer:
    // In attack reducer, replace the sampling:
    let attack_stat = ((attacker.dexterity as u16 + attacker.strength as u16) / 2) as u8;
    let attack_skill = 50u8;
    let attack_roll = combat_stats::calculate_roll_base(attack_stat, attack_skill, 1.0);
    let attack_sample = combat_stats::random_variance(attack_roll, ctx);

    let defense_stat = ((target.dexterity as u16 + target.perception as u16) / 2) as u8;
    let defense_skill = 50u8;
    let defense_roll = combat_stats::calculate_roll_base(defense_stat, defense_skill, 0.9);
    let defense_sample = combat_stats::random_variance(defense_roll, ctx);

    let hit = attack_sample > defense_sample;
    let is_crit = combat_stats::is_critical_hit(attack_sample, defense_sample);
    let is_fumble = combat_stats::is_critical_fail(attack_sample, defense_sample);
        
    let mut damage = 0;
    let mut result_message = String::new();
    
    if is_fumble {
        result_message = format!("{} fumbles the attack!", attacker.name);
    } else if !hit {
        result_message = format!("{} misses {}", attacker.name, target.name);
    } else {
        // Calculate base damage (Strength-based)
        let base_damage = (attacker.strength as f32 / 10.0) as i32;
        damage = base_damage;
        
        if is_crit {
            damage = (damage as f32 * 1.3) as i32;
            result_message = format!("{} critically hits {} for {} damage!", 
                attacker.name, target.name, damage);
        } else {
            result_message = format!("{} hits {} for {} damage", 
                attacker.name, target.name, damage);
        }
        
        // Apply damage
        target.hp = (target.hp - damage).max(0);
        
        if target.hp == 0 {
            target.is_alive = false;
            result_message.push_str(&format!(" {} has died!", target.name));
        }
    }
    
    // Consume stamina
    attacker.stamina = (attacker.stamina - 10).max(0);
    attacker.last_action_at = ctx.timestamp.to_micros_since_unix_epoch();
    
    // Save entities
    let attacker_id = attacker.id;
    ctx.db.entity().id().update(attacker);
    ctx.db.entity().id().update(target);
    
    // Create combat event
    let event = GameEvent {
        id: 0,
        room_id: room.id,
        timestamp: ctx.timestamp.to_micros_since_unix_epoch(),
        event_type: EventType::Combat,
        event_data: format!("{{\"attacker\": {}, \"target\": {}, \"damage\": {}, \"hit\": {}, \"critical\": {}}}", 
            attacker_id, target_id, damage, hit, is_crit),
        primary_actor: attacker_id,
        secondary_actor: Some(target_id),
        requires_sight: true,
        requires_hearing: true,
        stealth_dc: None,
        expires_at: ctx.timestamp.to_micros_since_unix_epoch() + 60_000_000,
    };
    
    ctx.db.game_event().try_insert(event)
        .map_err(|e| format!("Failed to create event: {:?}", e))?;
    
    log::info!("Combat: {}", result_message);
    
    Ok(())
}