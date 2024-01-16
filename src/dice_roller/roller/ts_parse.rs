#[test]
fn standard() {
    let r: super::Roller = String::from("3d6").parse().unwrap();
    assert_eq!(r.dice, 3);
    assert_eq!(r.sides, 6);
}

#[test]
fn single_die() {
    let r: super::Roller = String::from("d10").parse().unwrap();
    assert_eq!(r.dice, 1);
    assert_eq!(r.sides, 10);
}

#[test]
fn standard_and_modifier() {
    let r: super::Roller = String::from("3d6+4").parse().unwrap();
    assert_eq!(r.dice, 3);
    assert_eq!(r.sides, 6);
    assert_eq!(r.modifier, Some(4));
}

#[test]
fn standard_and_negative_modifier() {
    let r: super::Roller = String::from("3d6-4").parse().unwrap();
    assert_eq!(r.dice, 3);
    assert_eq!(r.sides, 6);
    assert_eq!(r.modifier, Some(-4));
}

#[test]
fn single_die_with_modifier() {
    let r: super::Roller = String::from("d6-4").parse().unwrap();
    assert_eq!(r.dice, 1);
    assert_eq!(r.sides, 6);
    assert_eq!(r.modifier, Some(-4));
}

#[test]
fn success_threshold() {
    let r: super::Roller = String::from("3d6 sc5").parse().unwrap();
    assert_eq!(r.dice, 3);
    assert_eq!(r.sides, 6);
    assert_eq!(r.success_threshold, Some(5));
}

#[test]
fn explode_threshold() {
    let r: super::Roller = String::from("3d6 ex4").parse().unwrap();
    assert_eq!(r.dice, 3);
    assert_eq!(r.sides, 6);
    assert_eq!(r.explode_threshold, Some(4));
}

#[test]
fn max_x_of_y() {
    let r: super::Roller = String::from("4d6 max3").parse().unwrap();
    assert_eq!(r.dice, 4);
    assert_eq!(r.sides, 6);
    assert_eq!(r.take_max, Some(3));
}

#[test]
fn min_x_of_y() {
    let r: super::Roller = String::from("4d6 min3").parse().unwrap();
    assert_eq!(r.dice, 4);
    assert_eq!(r.sides, 6);
    assert_eq!(r.take_min, Some(3));
}

#[test]
fn mid_x_of_y() {
    let r: super::Roller = String::from("5d6 mid3").parse().unwrap();
    assert_eq!(r.dice, 5);
    assert_eq!(r.sides, 6);
    assert_eq!(r.take_mid, Some(3));
}

#[test]
fn overflow_check() {
    let r: Result<super::Roller, super::RollerErr> = String::from("201d10").parse();
    assert_eq!(r, Err(super::RollerErr::PossibleOverflow));
}

#[test]
fn partial_eq() {
    let r1: super::Roller = String::from("3d6").parse().unwrap();
    let r2: super::Roller = String::from("3d6").parse().unwrap();

    assert_eq!(r1, r2);

    let r1: super::Roller = String::from("3d6 max2 ex6 sc6").parse().unwrap();
    let r2: super::Roller = String::from("3d6 max2 ex6 sc6").parse().unwrap();

    assert_eq!(r1, r2);
}

#[test]
fn success_values() {
    let r: super::Roller = String::from("3d6 sc5 sv:6:2").parse().unwrap();
    let value = r.success_values.get(&6);
    assert_eq!(value, Option::Some(&2));
}

#[test]
fn multi_success_values() {
    let r: super::Roller = String::from("3d6 sc5 sv:6:2 sv:5:1").parse().unwrap();
    let value = r.success_values.get(&5);
    assert_eq!(value, Option::Some(&1));
    let value = r.success_values.get(&6);
    assert_eq!(value, Option::Some(&2));
}
