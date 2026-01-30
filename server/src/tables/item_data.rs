use spacetimedb::SpacetimeType;

#[spacetimedb::table(name = item_data)]
pub struct ItemData {
    #[primary_key]
    pub entity_id: u64,

    pub item_type: ItemType,

    pub quantity: u32,
    pub max_stack: u32,

    pub base_damage: u16,
    pub damage_type: DamageType,
    pub attack_speed: f32,

    pub armor_rating: u16,
    pub armor_type: ArmorType,

    pub internal_volume: f32,
    pub weight_reduction: f32,

    pub durability: u16,
    pub max_durability: u16,
    pub is_equipped: bool,
    pub equipped_slot: Option<EquipSlot>,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ItemType {
    Weapon,
    Armor,
    Container,
    Consumable,
    Tool,
    QuestItem,
    Gold,
    Junk,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Fire,
    Ice,
    Lightning,
    Acid,
    Poison,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum ArmorType {
    Cloth,
    Leather,
    Chain,
    Plate,
}

#[derive(SpacetimeType, Clone, Copy, Debug, PartialEq)]
pub enum EquipSlot {
    Head,
    Torso,
    Legs,
    Feet,
    Hands,
    MainHand,
    OffHand,
    TwoHand,
    Neck,
    Ring,
}
