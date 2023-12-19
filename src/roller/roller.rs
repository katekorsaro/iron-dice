use rand::rngs::ThreadRng;
use rand::{thread_rng, Rng};
use std::str::FromStr;

mod parse;
mod roll;

use super::roll_result::*;
use super::roll_err::*;

/// A die roller engine. Given a valid string such as "3d6", "d20", will generate a roll result.
pub struct Roller {
    /// number of dice to throw
    dice: u32,

    /// number of side per die
    sides: u32,

    /// optional modifier per roll
    modifier: Option<i32>,

    /// optional success threshold per roll
    success_threshold: Option<u32>,

    /// optional explode threshold per die
    explode_threshold: Option<u32>,

    /// random number generator
    rng: ThreadRng,

    /// maximum number of dice to consider for outcome
    take_max: Option<u32>,
}

impl Roller {
    /// Creates a simple roller
    pub fn new(dice: u32, sides: u32) -> Self {
        Self {
            dice,
            sides,
            modifier: None,
            success_threshold: None,
            explode_threshold: None,
            take_max: None,
            rng: thread_rng(),
        }
    }

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

        RollResult::new(results, sum)
    }

    fn roll_one(&mut self) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();
        loop {
            let result = self.rng.gen_range(1..=self.sides).try_into().unwrap();

            // check for success roll:
            // if no success, then normal result
            // else 1 for success, 0 otherwise
            match self.success_threshold {
                None => results.push(result),
                Some(threshold) => {
                    if result >= threshold.try_into().unwrap() {
                        results.push(1);
                    } else {
                        results.push(0);
                    }
                }
            }

            // check for exit conditions:
            // if there's now threshold or
            // the result is under the threshold
            match self.explode_threshold {
                None => break,
                Some(threshold) => {
                    if result < threshold.try_into().unwrap() {
                        break;
                    }
                }
            }
        }
        results
    }

    fn modifier(mut self, modifier: Option<i32>) -> Self {
        self.modifier = modifier;
        self
    }

    fn success_threshold(mut self, success_threshold: Option<u32>) -> Self {
        self.success_threshold = success_threshold;
        self
    }

    fn explode_threshold(mut self, explode_threshold: Option<u32>) -> Self {
        self.explode_threshold = explode_threshold;
        self
    }

    fn take_max (mut self, take_max: Option<u32>) -> Self {
        self.take_max = take_max;
        self
    }

    fn parse_success_descriptor(descriptor: &str) -> Option<u32> {
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

    fn parse_explode_descriptor(descriptor: &str) -> Option<u32> {
        // ex handling
        let explode_descriptor = descriptor
            .split(&[' '])
            .filter(|x| x.contains("ex"))
            .take(1)
            .map(|x| x.replace("ex", ""))
            .map(|x| x.parse::<u32>().ok())
            .collect::<Vec<Option<u32>>>()
            .pop()
            .flatten();

        explode_descriptor
    }

    fn parse_take_max_descriptor (descriptor: &str) -> Option<u32> {
        // maxN handling
        let take_max_descriptor = descriptor
            .split(&[' '])
            .filter(|x| x.contains("max"))
            .take(1)
            .map(|x| x.replace("max", ""))
            .map(|x| x.parse::<u32>().ok())
            .collect::<Vec<Option<u32>>>()
            .pop()
            .flatten();

        take_max_descriptor
    }
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

        // considering only the first part
        let tokens: String = descriptor.split(&[' ']).take(1).collect::<String>();

        // tokenisation
        let tokens: Vec<_> = tokens
            .split(&['d', '+', '-'])
            .filter(|x| !x.is_empty())
            .collect();

        // parsing success threshold
        let success_descriptor = Roller::parse_success_descriptor(&descriptor);

        // parsing explode threshold
        let explode_descriptor = Roller::parse_explode_descriptor(&descriptor);

        // parsing max N
        let take_max_descriptor = Roller::parse_take_max_descriptor(&descriptor);

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
            .success_threshold(success_descriptor)
            .explode_threshold(explode_descriptor)
            .take_max(take_max_descriptor)
            )
    }
}
