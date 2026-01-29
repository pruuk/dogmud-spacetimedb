use spacetimedb::table;

#[table(name = condition)]
#[derive(Clone, Debug)]
pub struct Condition {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub entity_id: u64,
    // TODO: Add remaining fields
}
