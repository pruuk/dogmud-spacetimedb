use spacetimedb::table;

#[table(name = room)]
#[derive(Clone, Debug)]
pub struct Room {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub region_id: u64,
    // TODO: Add remaining fields
}
