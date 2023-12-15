use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::str::FromStr;

/// A die roller engine. Given a valid string such as "3d6", "d20", will generate a roll result.
pub struct Roller {
    dice: usize,
    sides: usize,
    modifier: Option<usize>,
    rng: ThreadRng,
}

impl Roller {
    /// Creates a simple roller
    pub fn new(dice: usize, sides: usize) -> Self {
        Self {
            dice,
            sides,
            modifier: None,
            rng: thread_rng(),
        }
    }
    /// Generates a roll result. The result will hold a Vector of die results as well as the sum
    pub fn roll(&mut self) -> (Vec<usize>, usize) {
        let mut die_results: Vec<usize> = Vec::new();
        for _ in 1..=self.dice {
            die_results.push(self.rng.gen_range(1..=self.sides));
        }
        (die_results.clone(), die_results.iter().sum::<usize>() + self.modifier.unwrap_or(0))
    }
    fn modifier (mut self, modifier: Option<usize>) -> Self {
        self.modifier = modifier;
        self
    }
}

#[derive(Debug)]
/// Error returned while parsing the dice notation
pub enum RollerErr {
    None,
}

impl FromStr for Roller {
    type Err = RollerErr;
    fn from_str(descriptor: &str) -> Result<Roller, RollerErr> {
        let tokens: Vec<_> = descriptor
            .split(&['d','+'])
            .filter(|x| !x.is_empty())
            .collect();
        let descriptor: (usize, usize, Option<usize>) = match tokens.len() {
            1 => (1, tokens.first().unwrap().parse().unwrap(), None),
            2 => (tokens.first().unwrap().parse().unwrap(), tokens.last().unwrap().parse().unwrap(), None),
            3 => (tokens.first().unwrap().parse().unwrap(), tokens.get(1).unwrap().parse().unwrap(), Some(tokens.last().unwrap().parse().unwrap())),
            _ => todo!(),
        };
        Ok(Roller::new(descriptor.0, descriptor.1).modifier(descriptor.2))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn roller_parse() {
        let r: Roller = String::from("3d6").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);

        let r: Roller = String::from("d10").parse().unwrap();
        assert_eq!(r.dice, 1);
        assert_eq!(r.sides, 10);

        let r: Roller = String::from("3d6+4").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(4));
    }

    #[test]
    fn roller_roll() {
        let mut r: Roller = String::from("3d6").parse().unwrap();
        let (dice_result, final_result) = r.roll();
        assert_eq!(dice_result.len(), 3);
        for die_result in dice_result {
            assert!(die_result > 0 && die_result < 7);
        }
        assert!(final_result > 0 && final_result < 19);

        let mut r: Roller = String::from("d20").parse().unwrap();
        let (dice_result, final_result) = r.roll();
        assert_eq!(dice_result.len(), 1);
        assert!(final_result > 0 && final_result < 21);

        let mut r: Roller = String::from("1d20+20").parse().unwrap();
        let (dice_result, final_result) = r.roll();
        assert_eq!(dice_result.len(), 1);
        assert!(final_result > 20 && final_result < 41);
    }
}
