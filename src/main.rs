use clap::Parser;
use iron_dice::Roller;

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(long, short)]
    /// the definition of dice to throw. "3d6" "4d6 max3" "5d10 sc9"
    definition: Option<String>,
}
fn main() {
    let args = Args::parse();

    let definition = match args.definition {
        Some(definition) => definition,
        None => String::from("3d6"),
    };

    let mut r: Roller = definition.parse().unwrap();
    let result = r.roll();

    println!("{:?} => {}", result.dice, result.outcome);
}
