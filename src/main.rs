use iron_dice::*;

fn main() {
    let mut r: Roller = String::from("4d6 max3").parse().unwrap();
    for _ in 1..=6 {
        let result = r.roll();
        println!("{result:?}");
    }
}
