use spacetimedb::rand::Rng;

pub fn calculate_roll_base(stat: u8, skill: u8, modifier: f32) -> f32 {
    let stat_contribution = stat as f32 * 0.7;
    let skill_contribution = skill as f32 * 0.3;
    (stat_contribution + skill_contribution) * modifier
}

pub fn calculate_std_dev(mean: f32) -> f32 {
    mean * 0.15
}

pub fn random_variance(base: f32, ctx: &spacetimedb::ReducerContext) -> f32 {
    let variance = calculate_std_dev(base);
    let min = base - variance;
    let max = base + variance;
    ctx.rng().gen_range(min..=max)
}

pub fn is_critical_hit(attacker_roll: f32, defender_roll: f32) -> bool {
    attacker_roll >= defender_roll * 1.2
}

pub fn is_critical_fail(attacker_roll: f32, defender_roll: f32) -> bool {
    attacker_roll <= defender_roll * 0.7
}
