use crate::tables::{
    account::account,               // ← Add trait import
    entity::entity,                 // ← Add trait import
    player_session::player_session, // ← Add trait import
    Account,
    ClientType,
    Entity,
    EntityType,
    PlayerSession,
};
use spacetimedb::{reducer, ReducerContext, Table};

#[reducer]
pub fn create_account(
    ctx: &ReducerContext,
    username: String,
    password: String,
) -> Result<(), String> {
    log::info!("Creating account: {}", username);

    if password.len() < 8 {
        return Err("Password must be at least 8 characters".to_string());
    }

    let existing = ctx.db.account().iter().any(|acc| acc.username == username);

    if existing {
        return Err(format!("Username '{}' already taken", username));
    }

    use argon2::{
        password_hash::{PasswordHasher, SaltString},
        Argon2,
    };

    let salt_source = format!(
        "{:?}{}",
        ctx.sender,
        ctx.timestamp.to_micros_since_unix_epoch()
    );
    let salt_bytes: [u8; 16] = {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(salt_source.as_bytes());
        let result = hasher.finalize();
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(&result[0..16]);
        bytes
    };

    use base64::{engine::general_purpose, Engine as _};
    let salt_b64 = general_purpose::STANDARD_NO_PAD.encode(&salt_bytes);
    let salt =
        SaltString::from_b64(&salt_b64).map_err(|e| format!("Failed to create salt: {}", e))?;

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

    ctx.db
        .account()
        .try_insert(account)
        .map_err(|e| format!("Failed to create account: {:?}", e))?;

    log::info!("Account created successfully");
    Ok(())
}

#[reducer]
pub fn create_character(ctx: &ReducerContext, name: String) -> Result<(), String> {
    log::info!("Creating character: {}", name);

    let account = ctx
        .db
        .account()
        .identity()
        .find(&ctx.sender)
        .ok_or("No account found. Create account first.")?;

    let name_taken = ctx
        .db
        .entity()
        .iter()
        .any(|e| e.entity_type == EntityType::Player && e.name == name);

    if name_taken {
        return Err(format!("Character name '{}' already taken", name));
    }

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

    let inserted = ctx
        .db
        .entity()
        .try_insert(character)
        .map_err(|e| format!("Failed to create character: {:?}", e))?;

    let mut updated_account = account;
    updated_account.primary_character_id = Some(inserted.id);
    ctx.db.account().identity().update(updated_account);

    log::info!("Character created successfully!");
    Ok(())
}

#[reducer]
pub fn login(ctx: &ReducerContext, username: String, password: String) -> Result<(), String> {
    log::info!("Login attempt: {}", username);

    let account = ctx
        .db
        .account()
        .iter()
        .find(|acc| acc.username == username)
        .ok_or("Invalid username or password")?;

    if account.is_banned {
        return Err("Account is banned".to_string());
    }

    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let parsed_hash = PasswordHash::new(&account.password_hash)
        .map_err(|e| format!("Invalid password hash: {}", e))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| "Invalid username or password".to_string())?;

    let session = PlayerSession {
        identity: ctx.sender,
        character_id: account.primary_character_id.unwrap_or(0),
        connected_at: ctx.timestamp.to_micros_since_unix_epoch(),
        last_heartbeat: ctx.timestamp.to_micros_since_unix_epoch(),
        client_type: ClientType::Unknown,
        client_version: "0.1.0".to_string(),
        is_active: true,
    };

    ctx.db
        .player_session()
        .try_insert(session)
        .map_err(|e| format!("Failed to create session: {:?}", e))?;

    log::info!("Login successful for: {}", username);
    Ok(())
}
