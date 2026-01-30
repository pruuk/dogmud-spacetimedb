use crate::tables::{
    condition::condition,           // ← Add trait import
    entity::entity,                 // ← Add trait import
    game_event::game_event,         // ← Add trait import
    player_session::player_session, // ← Add trait import
    room::room,                     // ← Add trait import
    ConditionType,
    EventType,
    GameEvent,
};
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer]
pub fn move_player(ctx: &ReducerContext, direction: String) -> Result<(), String> {
    log::info!("Player attempting to move: {}", direction);

    let session = ctx
        .db
        .player_session()
        .identity()
        .find(&ctx.sender)
        .ok_or("Not logged in")?;

    if session.character_id == 0 {
        return Err("No character selected".to_string());
    }

    let mut player = ctx
        .db
        .entity()
        .id()
        .find(&session.character_id)
        .ok_or("Character not found")?;

    if !player.is_alive {
        return Err("You are dead and cannot move".to_string());
    }

    let is_comatose = ctx
        .db
        .condition()
        .entity_id()
        .filter(&player.id)
        .any(|c| c.condition_type == ConditionType::Comatose);

    if is_comatose {
        return Err("You are comatose and cannot move".to_string());
    }

    let current_room = ctx
        .db
        .room()
        .id()
        .find(&player.room_id)
        .ok_or("Current room not found")?;

    let target_room_id = match direction.to_lowercase().as_str() {
        "north" | "n" => current_room.north_exit,
        "south" | "s" => current_room.south_exit,
        "east" | "e" => current_room.east_exit,
        "west" | "w" => current_room.west_exit,
        "up" | "u" => current_room.up_exit,
        "down" | "d" => current_room.down_exit,
        _ => return Err(format!("Invalid direction: {}", direction)),
    };

    let target_room_id = target_room_id.ok_or(format!("There is no exit to the {}", direction))?;

    let target_room = ctx
        .db
        .room()
        .id()
        .find(&target_room_id)
        .ok_or("Target room not found")?;

    if !target_room.is_active {
        return Err("That passage is blocked".to_string());
    }

    let player_name = player.name.clone();
    let player_id = player.id;
    let old_room_id = player.room_id;

    player.room_id = target_room_id;
    player.last_action_at = ctx.timestamp.to_micros_since_unix_epoch();

    ctx.db.entity().id().update(player);

    let event = GameEvent {
        id: 0,
        room_id: old_room_id,
        timestamp: ctx.timestamp.to_micros_since_unix_epoch(),
        event_type: EventType::Movement,
        event_data: format!(
            "{{\"entity_id\": {}, \"direction\": \"{}\", \"from_room\": {}, \"to_room\": {}}}",
            player_id, direction, old_room_id, target_room_id
        ),
        primary_actor: player_id,
        secondary_actor: None,
        requires_sight: true,
        requires_hearing: false,
        stealth_dc: None,
        expires_at: ctx.timestamp.to_micros_since_unix_epoch() + 60_000_000,
    };

    ctx.db
        .game_event()
        .try_insert(event)
        .map_err(|e| format!("Failed to create event: {:?}", e))?;

    log::info!(
        "Player {} moved {} from room {} to room {}",
        player_name,
        direction,
        old_room_id,
        target_room_id
    );

    Ok(())
}
