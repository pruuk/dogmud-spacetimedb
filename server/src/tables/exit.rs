#[spacetimedb::table(name = exit)]
pub struct Exit {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub from_room: u64,

    pub to_room: u64,
    pub direction: String,
    pub name: String,
    pub is_hidden: bool,
    pub is_locked: bool,
}
