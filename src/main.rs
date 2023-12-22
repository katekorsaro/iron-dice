use iron_dice::*;

fn main() {
    let mut r: Roller = String::from("6d6 max3 sc6").parse().unwrap();
    for _ in 1..=15 {
        let result = r.roll();
        println!("{result:?}");
    }
}
