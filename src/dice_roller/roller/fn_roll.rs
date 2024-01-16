use super::super::RollResult;
use super::Roller;

impl Roller {
    /// Generates a roll result. The result will hold a Vector of die results as well as the sum
    pub fn roll(&mut self) -> RollResult {
        let mut results: Vec<u8> = Vec::new();

        // rolling dice and getting raw results
        for _ in 1..=self.dice {
            let die_results = self.roll_one();
            for result in die_results {
                results.push(result);
            }
        }

        // standard roll
        let mut sum: i16 = match self.modifier {
            None => results.clone().into_iter().sum::<u8>() as i16,
            Some(modifier) => results.clone().into_iter().sum::<u8>() as i16 + modifier as i16,
        };

        // considering success counting
        let mut successes: Vec<i8> = Vec::new();
        if let Some(success_threshold) = self.success_threshold {
            results.iter().for_each(|x| {
                if *x >= success_threshold as u8 {
                    let value = self.success_values.get(x);
                    let value = match value {
                        None => 1_i8,
                        Some(value) => *value,
                    };
                    successes.push(value);
                } else {
                    successes.push(0);
                }
            });
            sum = successes.iter().sum::<i8>() as i16;
        }

        // considering the result array to analyze
        let mut counting_results = match self.success_threshold {
            None => results.iter().map(|x| *x as i8).collect(),
            Some(_) => successes.clone(),
        };

        // considering max
        sum = match self.take_max {
            None => sum,
            Some(max) => {
                counting_results.sort();
                counting_results.reverse();
                counting_results.iter().take(max as usize).sum::<i8>() as i16
            }
        };

        // considering min
        sum = match self.take_min {
            None => sum,
            Some(min) => {
                counting_results.sort();
                counting_results.iter().take(min as usize).sum::<i8>() as i16
            }
        };

        // considering mid
        sum = match self.take_mid {
            None => sum,
            Some(mid) => {
                counting_results.sort();
                counting_results
                    .iter()
                    .skip((results.len() - mid as usize) / 2)
                    .take(mid as usize)
                    .sum::<i8>() as i16
            }
        };

        RollResult::new(results, sum).successes(successes)
    }
}
