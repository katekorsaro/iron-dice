use super::Roller;
use super::super::RollResult;

impl Roller {
    /// Generates a roll result. The result will hold a Vector of die results as well as the sum
    pub fn roll(&mut self) -> RollResult {
        let mut results: Vec<i32> = Vec::new();

        for _ in 1..=self.dice {
            let die_results = self.roll_one();
            for result in die_results {
                results.push(result);
            }
        }

        // standard roll
        let sum: i32 = match self.modifier {
            None => results.clone().into_iter().sum(),
            Some(modifier) => results.clone().into_iter().sum::<i32>() + modifier,
        };

        // considering max
        let sum: i32 = match self.take_max {
            None => sum,
            Some(max) => {
                results.sort();
                results.reverse();
                results.iter().take(max.try_into().unwrap()).sum()
            },
        };

        // considering min
        let sum: i32 = match self.take_min {
            None => sum,
            Some(min) => {
                results.sort();
                results.iter().take(min.try_into().unwrap()).sum()
            },
        };

        RollResult::new(results, sum)
    }
}
