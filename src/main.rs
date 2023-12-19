use iron_dice::*;

fn main () {
    let mut r: Roller = String::from("3d6").parse().unwrap();
    for _ in 1..=6 {
        let result = r.roll();
        println!("{result:?}");
    }
}
