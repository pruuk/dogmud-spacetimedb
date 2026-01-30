use crate::tables::{
    entity::entity,                 // ← Add trait import
    game_event::game_event,         // ← Add trait import
    player_session::player_session, // ← Add trait import
    room::room,                     // ← Add trait import
    EventType,
    GameEvent,
};
use crate::utils::combat_stats;
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer]
pub fn attack(ctx: &ReducerContext, target_id: u64) -> Result<(), String> {
    log::info!("Attack initiated against target {}", target_id);

    let session = ctx
        .db
        .player_session()
        .identity()
        .find(&ctx.sender)
        .ok_or("Not logged in")?;

    if session.character_id == 0 {
        return Err("No character selected".to_string());
    }

    let mut attacker = ctx
        .db
        .entity()
        .id()
        .find(&session.character_id)
        .ok_or("Attacker not found")?;

    let mut target = ctx
        .db
        .entity()
        .id()
        .find(&target_id)
        .ok_or("Target not found")?;

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

    let room = ctx
        .db
        .room()
        .id()
        .find(&attacker.room_id)
        .ok_or("Room not found")?;

    if !room.allows_combat {
        return Err("Combat is not allowed here".to_string());
    }

    if attacker.stamina < 10 {
        return Err("Not enough stamina to attack".to_string());
    }

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
        let base_damage = (attacker.strength as f32 / 10.0) as i32;
        damage = base_damage;

        if is_crit {
            damage = (damage as f32 * 1.3) as i32;
            result_message = format!(
                "{} critically hits {} for {} damage!",
                attacker.name, target.name, damage
            );
        } else {
            result_message = format!(
                "{} hits {} for {} damage",
                attacker.name, target.name, damage
            );
        }

        target.hp = (target.hp - damage).max(0);

        if target.hp == 0 {
            target.is_alive = false;
            result_message.push_str(&format!(" {} has died!", target.name));
        }
    }

    attacker.stamina = (attacker.stamina - 10).max(0);
    attacker.last_action_at = ctx.timestamp.to_micros_since_unix_epoch();

    let attacker_id = attacker.id;
    ctx.db.entity().id().update(attacker);
    ctx.db.entity().id().update(target);

    let event = GameEvent {
        id: 0,
        room_id: room.id,
        timestamp: ctx.timestamp.to_micros_since_unix_epoch(),
        event_type: EventType::Combat,
        event_data: format!(
            "{{\"attacker\": {}, \"target\": {}, \"damage\": {}, \"hit\": {}, \"critical\": {}}}",
            attacker_id, target_id, damage, hit, is_crit
        ),
        primary_actor: attacker_id,
        secondary_actor: Some(target_id),
        requires_sight: true,
        requires_hearing: true,
        stealth_dc: None,
        expires_at: ctx.timestamp.to_micros_since_unix_epoch() + 60_000_000,
    };

    ctx.db
        .game_event()
        .try_insert(event)
        .map_err(|e| format!("Failed to create event: {:?}", e))?;

    log::info!("Combat: {}", result_message);

    Ok(())
}
