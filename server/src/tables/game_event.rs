use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = game_event)]
pub struct GameEvent {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub room_id: u64,

    #[index(btree)]
    pub timestamp: i64,

    pub event_type: EventType,
    pub event_data: String,

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
}
