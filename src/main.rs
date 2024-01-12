use iron_dice::Roller;

fn main() {
    let mut r:Roller = String::from("3d6").parse().unwrap();
    let result = r.roll();
    println!("{result:?}");
}
