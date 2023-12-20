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
