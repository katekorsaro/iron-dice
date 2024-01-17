use std::env;
use iron_dice::Roller;

fn main() {
    let args = env::args();
    let args = args
        .skip(1)
        .reduce(|mut acc:String, e| {
            acc.push_str(" ");
            acc.push_str(&e);
            acc
        })
        .unwrap();
    let args = args.trim();

    let mut r:Roller = String::from(args).parse().unwrap();
    let result = r.roll();

    println!("{:?} {}", result.dice, result.outcome);
}
