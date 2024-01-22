use super::Roller;
use rand::Rng;

impl Roller {
  pub fn roll_one(&mut self) -> Vec<u8> {
    let mut results: Vec<u8> = Vec::new();
    loop {
      let result = self.rng.gen_range(1..=self.sides) as u8;

      results.push(result);

      // check for exit conditions:
      // if there's now threshold or
      // the result is under the threshold
      match self.explode_threshold {
        None => break,
        Some(threshold) => {
          if result < threshold as u8 {
            break;
          }
        }
      }
    }
    results
  }
}
