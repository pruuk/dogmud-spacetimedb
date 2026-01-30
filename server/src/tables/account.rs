use spacetimedb::Identity;

#[spacetimedb::table(name = account)]
pub struct Account {
    #[primary_key]
    pub identity: Identity,

    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,

    pub created_at: i64,
    pub last_login: i64,
    pub total_play_time: u64,

    pub is_banned: bool,
    pub is_admin: bool,
    pub is_moderator: bool,

    pub primary_character_id: Option<u64>,
}
