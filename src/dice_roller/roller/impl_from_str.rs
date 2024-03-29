use super::{Roller, RollerErr};
use std::str::FromStr;
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

    // parsing min N
    let take_min_descriptor = Roller::parse_take_min_descriptor(&descriptor);

    // parsing mid N
    let take_mid_descriptor = Roller::parse_take_mid_descriptor(&descriptor);

    // parsing success values
    let success_values_descriptor = Roller::parse_success_values(&descriptor);

    // parsing failure values
    let failure_values_descriptor = Roller::parse_failure_values(&descriptor);

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

    if descriptor.0 * descriptor.1 as u32 > 2000 {
      return Err(RollerErr::PossibleOverflow);
    }

    Ok(
      Roller::new(descriptor.0, descriptor.1 as u32)
        .modifier(descriptor.2)
        .success_threshold(success_descriptor)
        .explode_threshold(explode_descriptor)
        .take_max(take_max_descriptor)
        .take_min(take_min_descriptor)
        .take_mid(take_mid_descriptor)
        .add_success_values(success_values_descriptor)
        .add_failure_values(failure_values_descriptor),
    )
  }
}
