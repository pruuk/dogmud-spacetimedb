use spacetimedb::{Identity, SpacetimeType};

#[spacetimedb::table(name = player_session)]
pub struct PlayerSession {
    #[primary_key]
    pub identity: Identity,

    pub character_id: u64,
    pub connected_at: i64,
    pub last_heartbeat: i64,

    pub client_type: ClientType,
    pub client_version: String,

    pub is_active: bool,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ClientType {
    RatatuiTUI,
    LeptosWeb,
    TelnetBridge,
    Unknown,
}
