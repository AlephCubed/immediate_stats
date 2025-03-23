pub trait StatContainer {
    /// Resets all stat bonuses to zero, and stat multipliers to one.
    fn reset_modifiers(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use immediate_stats_macros::StatContainer;

    #[derive(StatContainer, PartialEq, Debug)]
    struct Health {
        #[base(health)]
        health_base: i32,
        #[bonus(health)]
        health_bonus: i32,
        #[multiplier(health)]
        health_multiplier: f32,
    }

    #[test]
    fn default() {
        let mut h = Health {
            health_base: 10,
            health_bonus: 3,
            health_multiplier: 1.5,
        };

        h.reset_modifiers();

        assert_eq!(
            h,
            Health {
                health_base: 10,
                health_bonus: 0,
                health_multiplier: 1.0,
            }
        );
    }
}
