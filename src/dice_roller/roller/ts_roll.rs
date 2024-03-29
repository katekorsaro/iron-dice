#[test]
fn standard() {
  let mut r: super::Roller = String::from("3d6").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 3);
    for die in roll_result.dice {
      assert!(die > 0 && die < 7);
    }
    assert!(roll_result.outcome > 0 && roll_result.outcome < 19);
  }
}

#[test]
fn single_die() {
  let mut r: super::Roller = String::from("d20").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 1);
    assert!(roll_result.outcome > 0 && roll_result.outcome < 21);
  }
}

#[test]
fn standard_with_modifier() {
  let mut r: super::Roller = String::from("1d20+20").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 1);
    assert!(roll_result.outcome > 20 && roll_result.outcome < 41);
  }
}

#[test]
fn standard_with_negative_modifier() {
  let mut r: super::Roller = String::from("1d20-20").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 1);
    assert!((roll_result.outcome as i8) > -20 && (roll_result.outcome as i8) < 1);
  }
}

#[test]
fn standard_single_die_with_negative_modifier() {
  let mut r: super::Roller = String::from("d20-20").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 1);
    assert!((roll_result.outcome as i8) > -20 && (roll_result.outcome as i8) < 1);
  }
}

#[test]
fn standard_success_counting() {
  let mut r: super::Roller = String::from("6d6 sc1").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), roll_result.successes.len());
    roll_result.dice.into_iter().for_each(|x| {
      assert_ne!(x, 0);
    });
    roll_result.successes.into_iter().for_each(|x| {
      assert_eq!(x, 1);
    });
    assert_eq!(roll_result.outcome, 6);
  }
}

#[test]
fn standard_exploding() {
  let mut r: super::Roller = String::from("1d6 ex6").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    roll_result
      .dice
      .iter()
      .take(roll_result.dice.len() - 1)
      .for_each(|x| assert!(*x == 6));
    assert!(*roll_result.dice.last().unwrap() < 6);
  }
}

#[test]
fn exploding_success_threshold() {
  let mut r: super::Roller = String::from("1d6 ex6 sc6").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    roll_result
      .successes
      .iter()
      .take(roll_result.successes.len() - 1)
      .for_each(|x| assert!(*x == 1));
    assert!(*roll_result.successes.last().unwrap() == 0);
  }
}

#[test]
fn max_x_of_y() {
  let mut r: super::Roller = String::from("6d6 max3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 6);
    let mut results = roll_result.dice.clone();
    results.sort();
    results.reverse();
    let max3: i16 = results.iter().take(3).sum::<u8>() as i16;
    assert_eq!(max3, roll_result.outcome);
  }
}

#[test]
fn max_x_of_y_exploding() {
  let mut r: super::Roller = String::from("5d6 ex6 max3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    let mut results = roll_result.dice.clone();
    results.sort();
    results.reverse();
    let max3: i16 = results.iter().take(3).sum::<u8>() as i16;
    assert_eq!(max3, roll_result.outcome);
    if roll_result.dice.len() == 6 {
      assert_eq!(results[0], 6);
    }
  }
}

#[test]
fn min_x_of_y() {
  let mut r: super::Roller = String::from("6d6 min3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 6);
    let mut results = roll_result.dice.clone();
    results.sort();
    let min3: i16 = results.iter().take(3).sum::<u8>() as i16;
    assert_eq!(min3, roll_result.outcome);
  }
}

#[test]
fn min_x_of_y_exploding() {
  let mut r: super::Roller = String::from("5d6 ex6 min3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    let mut results = roll_result.dice.clone();
    results.sort();
    let min3: i16 = results.iter().take(3).sum::<u8>() as i16;
    assert_eq!(min3, roll_result.outcome);
    if roll_result.dice.len() == 6 {
      assert_eq!(results[5], 6);
    }
  }
}

#[test]
fn min_x_of_y_with_fewer_dice() {
  let mut r: super::Roller = String::from("2d6 min3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 2);
    let mut results = roll_result.dice.clone();
    results.sort();
    let min3: i16 = results.iter().take(3).sum::<u8>() as i16;
    assert_eq!(min3, roll_result.outcome);
  }
}

#[test]
fn max_x_of_y_with_fewer_dice() {
  let mut r: super::Roller = String::from("2d6 max3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 2);
    let mut results = roll_result.dice.clone();
    results.sort();
    results.reverse();
    let max3: i16 = results.iter().take(3).sum::<u8>() as i16;
    assert_eq!(max3, roll_result.outcome);
  }
}

#[test]
fn mid_x_of_y() {
  let mut r: super::Roller = String::from("7d6 mid3").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 7);
    let mut results = roll_result.dice.clone();
    results.sort();
    let mid3: i16 = results.iter().skip(2).take(3).sum::<u8>() as i16;
    assert_eq!(mid3, roll_result.outcome);
  }
}

#[test]
fn success_and_max() {
  let mut r: super::Roller = String::from("5d6 max3 sc1").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 5);
    roll_result.dice.iter().for_each(|x| {
      assert_ne!(*x, 0);
    });
    let mut results = roll_result.successes.clone();
    results.sort();
    results.reverse();
    let sum: i16 = results.iter().take(3).sum::<i8>() as i16;
    assert_eq!(sum, roll_result.outcome);
  }
}

#[test]
fn success_and_min() {
  let mut r: super::Roller = String::from("5d6 min3 sc1").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 5);
    roll_result.dice.iter().for_each(|x| {
      assert_ne!(*x, 0);
    });
    let mut results = roll_result.successes.clone();
    results.sort();
    results.reverse();
    let sum: i16 = results.iter().take(3).sum::<i8>() as i16;
    assert_eq!(sum, roll_result.outcome);
  }
}

#[test]
fn success_and_mid() {
  let mut r: super::Roller = String::from("5d6 mid3 sc1").parse().unwrap();
  for _ in 1..=1000 {
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 5);
    roll_result.dice.iter().for_each(|x| {
      assert_ne!(*x, 0);
    });
    let mut results = roll_result.successes.clone();
    results.sort();
    results.reverse();
    let sum: i16 = results.iter().take(3).sum::<i8>() as i16;
    assert_eq!(sum, roll_result.outcome);
  }
}

#[test]
fn success_values() {
  let mut r: super::Roller = String::from("d6 sc1 sv:1:2 sv:2:2 sv:3:2 sv:4:2 sv:5:2 sv:6:2 ")
    .parse()
    .unwrap();
  let roll_result = r.roll();
  assert_eq!(roll_result.outcome, 2);
}

#[test]
fn failure_values() {
  let mut r: super::Roller =
    String::from("d6 sc7 fv:1:-2 fv:2:-2 fv:3:-2 fv:4:-2 fv:5:-2 fv:6:-2 ")
      .parse()
      .unwrap();
  let roll_result = r.roll();
  assert_eq!(roll_result.outcome, -2);
}
