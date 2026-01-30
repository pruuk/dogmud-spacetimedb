use spacetimedb::rand::Rng;

// Re-export the pure functions from common
pub use dogmud_common::{
    calculate_roll_base, calculate_std_dev, is_critical_fail, is_critical_hit,
};

// This function stays here because it needs SpacetimeDB context
pub fn random_variance(base: f32, ctx: &spacetimedb::ReducerContext) -> f32 {
    let variance = calculate_std_dev(base);
    let min = base - variance;
    let max = base + variance;
    ctx.rng().gen_range(min..=max)
}
