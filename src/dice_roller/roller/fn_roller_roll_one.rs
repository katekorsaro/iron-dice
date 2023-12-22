use super::Roller;
use rand::Rng;

impl Roller {
    pub fn roll_one(&mut self) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        loop {
            let result = self.rng.gen_range(1..=self.sides) as i32;

            results.push(result);

            // check for exit conditions:
            // if there's now threshold or
            // the result is under the threshold
            match self.explode_threshold {
                None => break,
                Some(threshold) => {
                    if result < threshold as i32 {
                        break;
                    }
                }
            }
        }
        results
    }
}
