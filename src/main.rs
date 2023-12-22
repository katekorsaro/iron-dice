use iron_dice::*;

fn main() {
    let mut r: Roller = String::from("7d6 mid3").parse().unwrap();
    for _ in 1..=6 {
        let result = r.roll();
        println!("{result:?}");
    }
}
