use crate::tables::{
    entity::entity, // ← Add trait import (for create_test_entity)
    region::region, // ← Add trait import
    room::room,     // ← Add trait import
    BiomeType,
    ClimateType,
    Entity,
    EntityType, // ← Add these for create_test_entity
    Region,
    Room,
};
use spacetimedb::{reducer, ReducerContext, Table};
use std::collections::HashMap;

#[reducer]
pub fn create_test_region(ctx: &ReducerContext) {
    log::info!("Creating test region");

    let test_region = Region {
        id: 0,
        name: "Test Dungeon".to_string(),
        description: "A small dungeon for testing game mechanics.".to_string(),
        biome: BiomeType::Dungeon,
        climate: ClimateType::Temperate,
        base_temperature: 18,
        base_light_level: 64,
        default_spawn_room: 1,
        is_active: true,
        tick_rate_fast: 1000,
        tick_rate_medium: 5000,
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
pub fn create_test_room(ctx: &ReducerContext) {
    log::info!("Creating test room");

    let test_room = Room {
        id: 0,
        region_id: 1,
        name: "The Starting Chamber".to_string(),
        description: "A dimly lit stone chamber. Torches flicker on the walls, casting dancing shadows. You see exits to the north, south, east, and west.".to_string(),
        north_exit: None,
        south_exit: None,
        east_exit: None,
        west_exit: None,
        up_exit: None,
        down_exit: None,
        has_special_exits: false,
        temperature_modifier: 2,
        light_modifier: 64,
        is_safe_zone: true,
        allows_combat: false,
        allows_magic: true,
        current_volume: None,
        max_volume: None,
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
    log::info!(
        "Creating {}x{} room grid centered at ({}, {})",
        size,
        size,
        center_x,
        center_y
    );

    if size > 20 {
        return Err("Grid size too large (max 20x20)".to_string());
    }

    let half_size = (size / 2) as i32;
    let mut room_ids: HashMap<(i32, i32), u64> = HashMap::new();

    // First pass: Create all rooms
    for x in (center_x - half_size)..=(center_x + half_size) {
        for y in (center_y - half_size)..=(center_y + half_size) {
            let room = Room {
                id: 0,
                region_id: 1,
                name: format!("Room [{}, {}]", x, y),
                description: format!("A stone chamber at coordinates ({}, {}). Exits lead in the cardinal directions.", x, y),
                north_exit: None,
                south_exit: None,
                east_exit: None,
                west_exit: None,
                up_exit: None,
                down_exit: None,
                has_special_exits: false,
                temperature_modifier: 0,
                light_modifier: 50,
                is_safe_zone: false,
                allows_combat: true,
                allows_magic: true,
                current_volume: None,
                max_volume: None,
                is_active: true,
            };

            let inserted = ctx
                .db
                .room()
                .try_insert(room)
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

            if let Some(&north_id) = room_ids.get(&(x, y + 1)) {
                room.north_exit = Some(north_id);
            }

            if let Some(&south_id) = room_ids.get(&(x, y - 1)) {
                room.south_exit = Some(south_id);
            }

            if let Some(&east_id) = room_ids.get(&(x + 1, y)) {
                room.east_exit = Some(east_id);
            }

            if let Some(&west_id) = room_ids.get(&(x - 1, y)) {
                room.west_exit = Some(west_id);
            }

            ctx.db.room().id().update(room);
        }
    }

    log::info!(
        "Room grid created successfully! Total rooms: {}",
        room_ids.len()
    );
    Ok(())
}

#[reducer]
pub fn create_test_entity(ctx: &ReducerContext, name: String) {
    use crate::tables::{Entity, EntityType};

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
}
