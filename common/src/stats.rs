use rand_distr::{Distribution, Normal};

/// Calculate roll base from stat + skill with modifiers
pub fn calculate_roll_base(stat: u8, skill: u8, modifier: f32) -> f32 {
    ((stat as f32 + skill as f32) * modifier).max(1.0)
}

/// Calculate standard deviation (proportional to base)
/// Formula: σ = μ × 0.15
pub fn calculate_std_dev(base: f32) -> f32 {
    base * 0.15
}

/// Sample from normal distribution N(mean, std_dev)
pub fn normal_sample(mean: f32, std_dev: f32) -> f32 {
    let normal = Normal::new(mean, std_dev).unwrap();
    normal.sample(&mut rand::rng())
}

/// Check if roll is a critical hit
/// Formula: attacker_roll ≥ defender_roll × 1.3
pub fn is_critical_hit(attacker_roll: f32, defender_roll: f32) -> bool {
    attacker_roll >= (defender_roll * 1.3)
}

/// Check if roll is a critical failure
/// Formula: roll ≤ base × 0.7
pub fn is_critical_fail(roll: f32, base: f32) -> bool {
    roll <= (base * 0.7)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_base() {
        let base = calculate_roll_base(100, 12, 0.9);
        // Use approximate equality for floats
        assert!((base - 100.8).abs() < 0.01, "Expected ~100.8, got {}", base);
    }

    #[test]
    fn test_std_dev() {
        let std = calculate_std_dev(100.0);
        assert!((std - 15.0).abs() < 0.01, "Expected ~15.0, got {}", std);
    }

    #[test]
    fn test_critical_hit() {
        assert!(is_critical_hit(118.0, 81.0));
        assert!(!is_critical_hit(100.0, 95.0));
    }

    #[test]
    fn test_critical_fail() {
        assert!(is_critical_fail(70.0, 100.0));
        assert!(!is_critical_fail(90.0, 100.0));
    }
}
