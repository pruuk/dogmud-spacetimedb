use spacetimedb::table;

#[table(name = skill)]
#[derive(Clone, Debug)]
pub struct Skill {
    #[primary_key]
    #[auto_inc]
    pub id: u64,
    pub entity_id: u64,
    // TODO: Add remaining fields
}
