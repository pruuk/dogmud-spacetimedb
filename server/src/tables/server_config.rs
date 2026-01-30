use spacetimedb::Identity;

#[spacetimedb::table(name = server_config)]
pub struct ServerConfig {
    #[primary_key]
    pub key: String,

    pub value: String,
    pub last_updated: i64,
    pub updated_by: Option<Identity>,
}
