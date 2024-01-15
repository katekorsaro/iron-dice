use rand::rngs::ThreadRng;
use rand::thread_rng;
use std::collections::HashMap;

// public functions implementation
mod fn_roll;
mod fn_roll_one;
mod impl_from_str;
mod impl_partial_eq;

// unit tests
mod ts_parse;
mod ts_roll;

use super::roll_err::*;

#[derive(Debug)]
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

    /// minimum number of dice to consider for outcome
    take_min: Option<u32>,

    /// mid number of dice to consider for outcome
    take_mid: Option<u32>,

    /// hash with success values
    success_values: HashMap<u32, i32>,
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
            take_min: None,
            take_mid: None,
            rng: thread_rng(),
            success_values: HashMap::new(),
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

    fn explode_threshold(mut self, explode_threshold: Option<u32>) -> Self {
        self.explode_threshold = explode_threshold;
        self
    }

    fn take_max(mut self, take_max: Option<u32>) -> Self {
        self.take_max = take_max;
        self
    }

    fn take_min(mut self, take_min: Option<u32>) -> Self {
        self.take_min = take_min;
        self
    }

    fn take_mid(mut self, take_mid: Option<u32>) -> Self {
        self.take_mid = take_mid;
        self
    }

    fn add_success_values (mut self, values: Option<Vec<(u32, i32)>>) -> Self {
        if let Some(values) = values {
            values.iter()
                .for_each(|x| {self.success_values.insert(x.0, x.1);});
        } else {
            self.success_values.clear();
        }

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

    fn parse_take_max_descriptor(descriptor: &str) -> Option<u32> {
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

    fn parse_take_min_descriptor(descriptor: &str) -> Option<u32> {
        // minN handling
        let take_min_descriptor = descriptor
            .split(&[' '])
            .filter(|x| x.contains("min"))
            .take(1)
            .map(|x| x.replace("min", ""))
            .map(|x| x.parse::<u32>().ok())
            .collect::<Vec<Option<u32>>>()
            .pop()
            .flatten();

        take_min_descriptor
    }

    fn parse_take_mid_descriptor(descriptor: &str) -> Option<u32> {
        // midN handling
        let take_mid_descriptor = descriptor
            .split(&[' '])
            .filter(|x| x.contains("mid"))
            .take(1)
            .map(|x| x.replace("mid", ""))
            .map(|x| x.parse::<u32>().ok())
            .collect::<Vec<Option<u32>>>()
            .pop()
            .flatten();

        take_mid_descriptor
    }

    fn parse_success_values (descriptor: &str) -> Option<Vec<(u32, i32)>> {
        let mut success_values: Vec<(u32, i32)> = Vec::new();

        let success_value_descriptor = descriptor
            .split(&[' '])
            .filter(|x| x.contains("sv"))
            .map(|x| {
                let tokens: Vec<_> = x.split(&[':']).collect();
                (tokens[1].parse::<u32>().unwrap(), tokens[2].parse::<i32>().unwrap())
            })
            .for_each(|x| {success_values.insert(0, x);});

        if success_values.len() == 0 {
            return None;
        }

        Some(success_values)
    }
}

