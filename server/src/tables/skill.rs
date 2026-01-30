use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = skill)]
pub struct Skill {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub entity_id: u64,

    pub skill_type: SkillType,
    pub level: u8,
    pub experience: u32,
    pub last_used: i64,
    pub times_used: u32,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum SkillType {
    MeleeCombat,
    RangedCombat,
    MagicCasting,
    Tracking,
    Stealth,
    Blacksmithing,
    Hidemaking,
    Bowyery,
    Alchemy,
    Cooking,
    Haggling,
}
