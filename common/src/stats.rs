use rand_distr::{Distribution, Normal};

/// Calculate the base roll value from stats and skills
/// Stats contribute 70%, skills contribute 30%
pub fn calculate_roll_base(stat: u8, skill: u8, modifier: f32) -> f32 {
    let stat_contribution = stat as f32 * 0.7;
    let skill_contribution = skill as f32 * 0.3;
    (stat_contribution + skill_contribution) * modifier
}

/// Calculate standard deviation (15% of mean)
pub fn calculate_std_dev(mean: f32) -> f32 {
    mean * 0.15
}

/// Sample from a normal distribution (this version uses rand, can't be used in WASM server)
pub fn normal_sample(mean: f32, std_dev: f32) -> f32 {
    let normal = Normal::new(mean, std_dev).unwrap();
    normal.sample(&mut rand::rng())
}

/// Check if attacker roll is a critical hit (>= defender * 1.2)
pub fn is_critical_hit(attacker_roll: f32, defender_roll: f32) -> bool {
    let threshold = defender_roll * 1.2;
    // Use small epsilon for floating point comparison
    attacker_roll >= threshold - 0.001
}

/// Check if attacker roll is a critical fail (<= defender * 0.7)
pub fn is_critical_fail(attacker_roll: f32, defender_roll: f32) -> bool {
    let threshold = defender_roll * 0.7;
    // Use small epsilon for floating point comparison
    attacker_roll <= threshold + 0.001
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_base() {
        // stat=100, skill=50: (100*0.7) + (50*0.3) = 70 + 15 = 85
        let roll = calculate_roll_base(100, 50, 1.0);
        assert!((roll - 85.0).abs() < 0.01);

        // stat=50, skill=100: (50*0.7) + (100*0.3) = 35 + 30 = 65
        let roll = calculate_roll_base(50, 100, 1.0);
        assert!((roll - 65.0).abs() < 0.01);

        // With modifier 0.9: 85 * 0.9 = 76.5
        let roll = calculate_roll_base(100, 50, 0.9);
        assert!((roll - 76.5).abs() < 0.01);
    }

    #[test]
    fn test_std_dev() {
        let std_dev = calculate_std_dev(100.0);
        assert!((std_dev - 15.0).abs() < 0.01);

        let std_dev = calculate_std_dev(50.0);
        assert!((std_dev - 7.5).abs() < 0.01);
    }

    #[test]
    fn test_critical_hit() {
        // 120 >= 100 * 1.2 (120 >= 120) = true
        assert!(is_critical_hit(120.0, 100.0));
        assert!(is_critical_hit(121.0, 100.0));

        // 119 < 120 = false
        assert!(!is_critical_hit(119.0, 100.0));
        assert!(!is_critical_hit(100.0, 100.0));
    }

    #[test]
    fn test_critical_fail() {
        // 70 <= 100 * 0.7 (70 <= 70) = true
        assert!(is_critical_fail(70.0, 100.0));
        assert!(is_critical_fail(69.0, 100.0));

        // 71 > 70 = false
        assert!(!is_critical_fail(71.0, 100.0));
        assert!(!is_critical_fail(100.0, 100.0));
    }

    #[test]
    fn test_stat_weighting() {
        // stat=100, skill=0: 100 * 0.7 = 70
        let high_stat = calculate_roll_base(100, 0, 1.0);
        assert!((high_stat - 70.0).abs() < 0.01);

        // stat=0, skill=100: 100 * 0.3 = 30
        let high_skill = calculate_roll_base(0, 100, 1.0);
        assert!((high_skill - 30.0).abs() < 0.01);

        // Stats are more important (70% vs 30%)
        assert!(high_stat > high_skill * 2.0);
    }
}
