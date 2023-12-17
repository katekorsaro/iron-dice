use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::str::FromStr;

/// A die roller engine. Given a valid string such as "3d6", "d20", will generate a roll result.
pub struct Roller {
    /// number of dice to throw
    dice: u32,

    /// number of side per die
    sides: u32,

    /// optional modifier per roll
    modifier: Option<i32>,

    /// optional success threashold per roll
    success_threshold: Option<u32>,

    /// random number generator
    rng: ThreadRng,
}

impl Roller {
    /// Creates a simple roller
    pub fn new(dice: u32, sides: u32) -> Self {
        Self {
            dice,
            sides,
            modifier: None,
            success_threshold: None,
            rng: thread_rng(),
        }
    }

    /// Generates a roll result. The result will hold a Vector of die results as well as the sum
    pub fn roll(&mut self) -> (Vec<i32>, i32) {
        let mut die_results: Vec<i32> = Vec::new();

        for _ in 1..=self.dice {
            die_results.push(
                self.rng.gen_range(1..=self.sides)
                .try_into().unwrap()
                );
        }

        if let Some(threshold) = self.success_threshold {
            (
                die_results.clone(),
                die_results
                    .into_iter()
                    .map(|x| if x >= threshold.try_into().unwrap() { 1 } else { 0 })
                    .sum(),
            )
        } else {
            (
                die_results.clone(),
                die_results.into_iter().sum::<i32>() + self.modifier.unwrap_or(0),
            )
        }
    }

    fn modifier(mut self, modifier: Option<i32>) -> Self {
        self.modifier = modifier;
        self
    }

    fn success_threshold(mut self, success_threshold: Option<u32>) -> Self {
        self.success_threshold = success_threshold;
        self
    }

    fn parse_success_descriptor (descriptor: &str) -> Option<u32> {
        // sc handling
        let success_descriptor = descriptor
            .split(&[' '])
            .filter(|x| x.contains("sc"))
            .take(1)
            .map(|x| x.replace("sc", ""))
            .map(|x| x.parse::<u32>().ok())
            .collect::<Vec<Option<u32>>>()
            .pop()
            .flatten();
        
        success_descriptor
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

        // tokenization of main part
        let descriptor = descriptor.clone();
        let tokens: Vec<_> = descriptor
            .split(&['d', '+', '-', ' '])
            .filter(|x| !x.is_empty())
            .filter(|x| !x.contains("sc"))
            .collect();

        let success_descriptor = Roller::parse_success_descriptor(&descriptor);

        // output
        let descriptor: (u32, i32, Option<i32>) = match tokens.len() {
            2 => (
                tokens[0].parse().map_err(|_| RollerErr::Generic)?,
                tokens[1].parse().map_err(|_| RollerErr::Generic)?,
                None,
            ),
            3 => (
                tokens[0].parse().map_err(|_| RollerErr::Generic)?,
                tokens[1].parse().map_err(|_| RollerErr::Generic)?,
                Some(sign * tokens.last().unwrap().parse::<i32>().unwrap()),
            ),
            _ => todo!(),
        };

        Ok(Roller::new(descriptor.0, descriptor.1.try_into().unwrap())
            .modifier(descriptor.2)
            .success_threshold(success_descriptor))
    }

}

#[cfg(test)]
mod parse {
    use super::*;

    #[test]
    fn standard() {
        let r: Roller = String::from("3d6").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
    }

    #[test]
    fn single_die() {
        let r: Roller = String::from("d10").parse().unwrap();
        assert_eq!(r.dice, 1);
        assert_eq!(r.sides, 10);
    }

    #[test]
    fn standard_and_modifier() {
        let r: Roller = String::from("3d6+4").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(4));
    }

    #[test]
    fn standard_and_negative_modifier() {
        let r: Roller = String::from("3d6-4").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(-4));
    }

    #[test]
    fn single_die_with_modifier() {
        let r: Roller = String::from("d6-4").parse().unwrap();
        assert_eq!(r.dice, 1);
        assert_eq!(r.sides, 6);
        assert_eq!(r.modifier, Some(-4));
    }
    #[test]
    fn success_threshold() {
        let r: Roller = String::from("3d6 sc5").parse().unwrap();
        assert_eq!(r.dice, 3);
        assert_eq!(r.sides, 6);
        assert_eq!(r.success_threshold, Some(5));
    }
}

#[cfg(test)]
mod roll {
    use super::*;
    #[test]
    fn standard() {
        let mut r: Roller = String::from("3d6").parse().unwrap();
        let (dice_result, final_result) = r.roll();
        assert_eq!(dice_result.len(), 3);
        for die_result in dice_result {
            assert!(die_result > 0 && die_result < 7);
        }
        assert!(final_result > 0 && final_result < 19);
    }

    #[test]
    fn single_die() {
        let mut r: Roller = String::from("d20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > 0 && final_result < 21);
        }
    }

    #[test]
    fn standard_with_modifier() {
        let mut r: Roller = String::from("1d20+20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > 20 && final_result < 41);
        }
    }

    #[test]
    fn standard_with_negative_modifier() {
        let mut r: Roller = String::from("1d20-20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > -20 && final_result < 1);
        }
    }

    #[test]
    fn standard_single_die_with_negative_modifier() {
        let mut r: Roller = String::from("d20-20").parse().unwrap();
        for _ in 1..=1000 {
            let (dice_result, final_result) = r.roll();
            assert_eq!(dice_result.len(), 1);
            assert!(final_result > -20 && final_result < 1);
        }
    }
    #[test]
    fn standard_success_counting() {
        let mut r: Roller = String::from("6d6 sc1").parse().unwrap();
        let (_, final_result) = r.roll();
        for _ in 1..=1000 {
            assert_eq!(final_result, 6);
        }
    }
}
