use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = condition)]
pub struct Condition {
    #[primary_key]
    #[auto_inc]
    pub id: u64,

    #[index(btree)]
    pub entity_id: u64,

    pub condition_type: ConditionType,
    pub magnitude: f32,
    pub remaining_ticks: u32,
    pub source_id: Option<u64>,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ConditionType {
    Burning,
    Poisoned,
    Bleeding,
    Regenerating,
    Hasted,
    Blessed,
    Wet,
    Muddy,
    Frozen,
    Oiled,
    Blinded,
    Stunned,
    Comatose,
    Encumbered,
}
