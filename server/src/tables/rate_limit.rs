use spacetimedb::{Identity, SpacetimeType};

#[spacetimedb::table(name = rate_limit)]
pub struct RateLimit {
    #[primary_key]
    pub identity: Identity,

    pub action_type: ActionType,
    pub count: u32,
    pub window_start: i64,
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
