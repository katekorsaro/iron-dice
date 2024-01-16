use iron_dice::Roller;

fn main() {
    let mut r: Roller = String::from("6d6 sc4 sv:6:2 fv:1:-1").parse().unwrap();
    for _ in 1..=20 {
        let result = r.roll();
        println!("{result:?}");
    }
}
