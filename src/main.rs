use iron_dice::Roller;

fn main() {
    let mut r:Roller = String::from("3d6 sc4 sv:6:2").parse().unwrap();
    for _ in 1..=20 {
        let result = r.roll();
        println!("{result:?}");
    }
}
