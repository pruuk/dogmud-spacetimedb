// Combat formulas from TDD

/// Calculate damage multiplier from strength + skill
/// Formula: (strength + skill) / 100
pub fn calculate_damage_multiplier(strength: u8, skill: u8) -> f32 {
    (strength as f32 + skill as f32) / 100.0
}

/// Calculate base damage before armor
pub fn calculate_base_damage(weapon_damage: u16, strength: u8, skill: u8) -> u16 {
    let multiplier = calculate_damage_multiplier(strength, skill);
    (weapon_damage as f32 * multiplier) as u16
}
