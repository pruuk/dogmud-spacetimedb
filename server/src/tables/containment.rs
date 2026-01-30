#[spacetimedb::table(name = containment)]
pub struct Containment {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub container_id: u64,

    #[index(btree)]
    pub contained_id: u64,

    pub depth: u8,
    pub slot_index: Option<u8>,
}
