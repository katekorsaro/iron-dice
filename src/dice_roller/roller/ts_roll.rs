#[test]
fn standard() {
    let mut r: super::Roller = String::from("3d6").parse().unwrap();
    let roll_result = r.roll();
    assert_eq!(roll_result.dice.len(), 3);
    for die in roll_result.dice {
        assert!(die > 0 && die < 7);
    }
    assert!(roll_result.outcome > 0 && roll_result.outcome < 19);
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
        assert!(roll_result.outcome > -20 && roll_result.outcome < 1);
    }
}

#[test]
fn standard_single_die_with_negative_modifier() {
    let mut r: super::Roller = String::from("d20-20").parse().unwrap();
    for _ in 1..=1000 {
        let roll_result = r.roll();
        assert_eq!(roll_result.dice.len(), 1);
        assert!(roll_result.outcome > -20 && roll_result.outcome < 1);
    }
}

#[test]
fn standard_success_counting() {
    let mut r: super::Roller = String::from("6d6 sc1").parse().unwrap();
    for _ in 1..=1000 {
        let roll_result = r.roll();
        assert_eq!(roll_result.dice.len(), roll_result.successes.len());
        roll_result.successes.into_iter()
            .for_each(|x| {assert_eq!(x, 1);});
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
            .dice
            .iter()
            .take(roll_result.dice.len() - 1)
            .for_each(|x| assert!(*x == 1));
        assert!(*roll_result.dice.last().unwrap() == 0);
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
        let max3: i32 = results.iter().take(3).sum();
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
        let max3: i32 = results.iter().take(3).sum();
        assert_eq!(max3, roll_result.outcome);
        if roll_result.dice.len() == 4 {
            assert_eq!(roll_result.dice[0], 6);
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
        let min3: i32 = results.iter().take(3).sum();
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
        let min3: i32 = results.iter().take(3).sum();
        assert_eq!(min3, roll_result.outcome);
        if roll_result.dice.len() == 6 {
            assert_eq!(roll_result.dice[5], 6);
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
        let min3: i32 = results.iter().take(3).sum();
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
        let max3: i32 = results.iter().take(3).sum();
        assert_eq!(max3, roll_result.outcome);
    }
}
