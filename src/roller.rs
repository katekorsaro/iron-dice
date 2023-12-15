use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::str::FromStr;

/// A die roller engine. Given a valid string such as "3d6", "d20", will generate a roll result.
pub struct Roller {
    /// number of dice to throw
    dice: usize,

    /// number of side per die
    sides: isize,

    /// optional modifier per roll
    modifier: Option<isize>,

    /// random number generator
    rng: ThreadRng,
}

impl Roller {
    /// Creates a simple roller
    pub fn new(dice: usize, sides: isize) -> Self {
        Self {
            dice,
            sides,
            modifier: None,
            rng: thread_rng(),
        }
    }

    /// Generates a roll result. The result will hold a Vector of die results as well as the sum
    pub fn roll(&mut self) -> (Vec<isize>, isize) {
        let mut die_results: Vec<isize> = Vec::new();

        for _ in 1..=self.dice {
            die_results.push(self.rng.gen_range(1..=self.sides));
        }

        (
            die_results.clone(),
            die_results.iter().sum::<isize>() + self.modifier.unwrap_or(0),
        )
    }

    fn modifier(mut self, modifier: Option<isize>) -> Self {
        self.modifier = modifier;
        self
    }
}

#[derive(Debug)]
/// Error returned while parsing the dice notation
pub enum RollerErr {
    None,
    Generic,
}

/// for idiomatic parsing
impl FromStr for Roller {
    type Err = RollerErr;

    fn from_str(descriptor: &str) -> Result<Roller, RollerErr> {
        // handling single die scenario
        let mut descriptor: String = String::from(descriptor);
        if descriptor.starts_with('d') {
            descriptor.insert(0, '1');
        }

        // handling negative modifier
        let sign = if descriptor.contains('-') { -1 } else { 1 };

        let tokens: Vec<_> = descriptor
            .split(&['d', '+', '-'])
            .filter(|x| !x.is_empty())
            .collect();

        let descriptor: (usize, isize, Option<isize>) = match tokens.len() {
            2 => (
                tokens.first().unwrap().parse().unwrap(),
                tokens.last().unwrap().parse().unwrap(),
                None,
            ),
            3 => (
                tokens.first().unwrap().parse().unwrap(),
                tokens.get(1).unwrap().parse().unwrap(),
                Some(sign * tokens.last().unwrap().parse::<isize>().unwrap()),
            ),
            _ => todo!(),
        };

        Ok(Roller::new(descriptor.0, descriptor.1).modifier(descriptor.2))
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn parse_standard() {
        let r: Roller = String::from("3d6").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
    }

    #[test]
    fn parse_single_die() {
        let r: Roller = String::from("d10").parse().unwrap();
        assert_eq!(r.dice, 1);
        assert_eq!(r.sides, 10);
    }

    #[test]
    fn parse_standard_and_modifier() {
        let r: Roller = String::from("3d6+4").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(4));
    }

    #[test]
    fn parse_standard_and_negative_modifier() {
        let r: Roller = String::from("3d6-4").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(-4));
    }

    #[test]
    fn parse_single_die_with_modifier() {
        let r: Roller = String::from("d6-4").parse().unwrap();
        assert_eq!(r.dice, 1);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(-4));
    }

    #[test]
    fn roll_standard() {
        let mut r: Roller = String::from("3d6").parse().unwrap();
        let (dice_result, final_result) = r.roll();
        assert_eq!(dice_result.len(), 3);
        for die_result in dice_result {
            assert!(die_result > 0 && die_result < 7);
        }
        assert!(final_result > 0 && final_result < 19);
    }

    #[test]
    fn roll_single_die() {
        let mut r: Roller = String::from("d20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > 0 && final_result < 21);
        }
    }

    #[test]
    fn roll_standard_with_modifier() {
        let mut r: Roller = String::from("1d20+20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > 20 && final_result < 41);
        }
    }

    #[test]
    fn roll_standard_with_negative_modifier() {
        let mut r: Roller = String::from("1d20-20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > -20 && final_result < 1);
        }
    }

    #[test]
    fn roll_standard_single_die_with_negative_modifier() {
        let mut r: Roller = String::from("d20-20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > -20 && final_result < 1);
        }
    }
}
