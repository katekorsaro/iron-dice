use super::super::RollResult;
use super::Roller;

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
        let mut sum: i32 = match self.modifier {
            None => results.clone().into_iter().sum(),
            Some(modifier) => results.clone().into_iter().sum::<i32>() + modifier,
        };

        // considering success counting
        let mut successes: Vec<i32> = Vec::new();
        if let Some(success_threshold) = self.success_threshold {
            results.iter().for_each(|x| {
                if *x >= success_threshold as i32 {
                    successes.push(1);
                } else {
                    successes.push(0);
                }
            });
            sum = successes.iter().sum();
        }

        // considering the result array to analyze
        let mut results = match self.success_threshold {
            None => results,
            Some(_) => successes.clone(),
        };

        // considering max
        sum = match self.take_max {
            None => sum,
            Some(max) => {
                results.sort();
                results.reverse();
                results.iter().take(max as usize).sum()
            }
        };

        // considering min
        sum = match self.take_min {
            None => sum,
            Some(min) => {
                results.sort();
                results.iter().take(min as usize).sum()
            }
        };

        // considering mid
        sum = match self.take_mid {
            None => sum,
            Some(mid) => {
                results.sort();
                results.iter()
                    .skip((results.len() - mid as usize)/2)
                    .take(mid as usize)
                    .sum()
            }
        };


        RollResult::new(results, sum).successes(successes)
    }
}
