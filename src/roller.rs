use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::str::FromStr;

pub struct Roller {
    dice: usize,
    sides: usize,
    rng: ThreadRng,
}

impl Roller {
    pub fn new(dice: usize, sides: usize) -> Self {
        Self {
            dice,
            sides,
            rng: thread_rng(),
        }
    }
    pub fn roll(&mut self) -> (Vec<usize>, usize) {
        let mut die_results: Vec<usize> = Vec::new();
        for _ in 1..=self.dice {
            die_results.push(self.rng.gen_range(1..=self.sides));
        }
        (die_results.clone(), die_results.iter().sum())
    }
}

#[derive(Debug)]
pub enum RollerErr {
    None,
}

impl FromStr for Roller {
    type Err = RollerErr;
    fn from_str(descriptor: &str) -> Result<Roller, RollerErr> {
        let tokens: Vec<_> = descriptor
            .split('d')
            .filter(|x| !x.is_empty())
            .collect();
        let descriptor: (usize, usize) = match tokens.len() {
            1 => (1, tokens.first().unwrap().parse().unwrap()),
            2 => (tokens.first().unwrap().parse().unwrap(), tokens.last().unwrap().parse().unwrap()),
            _ => todo!(),
        };
        Ok(Roller::new(descriptor.0, descriptor.1))
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
    }
}