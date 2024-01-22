use clap::Parser;
use iron_dice::Roller;

#[derive(Parser)]
#[command(about = "Throws some dice on the standard output")]
struct Args {
  #[arg(long, short)]
  /// the definition of dice to throw. "3d6" "4d6 max3" "5d10 sc9"
  ///
  /// ** Possible definitions **
  ///
  /// - NdM where N is the number of dice to throw, default = 1. M is the number of sides per die.
  ///
  /// - NdM+C where N is the number of dice to throw, default = 1. M is the number of sides per die. C is added to the die results sum. Can be negative as well
  ///
  /// - minN takes the N smallest die results
  ///
  /// - maxN takes the N largest die results
  ///
  /// - midN takes the middle N die results (make sure you throw an odd number of dice)
  ///
  /// - exN explodes every die that shows N or more
  ///
  /// - scN count a success for every die that shows N or more
  ///
  ///   - sv:N:V used with sc, for every die that shows exactly N the success value is V. Can be spefied multiple times.
  ///
  ///   - fv:N:V used with sc, for every die that shows exactly N the failure value is V. Can be spefied multiple times.
  definition: Option<String>,

  #[arg(long, short)]
  /// the number of simoultaneous throws, default = 1
  throw_number: Option<u8>,
}
fn main() {
  let args = Args::parse();

  let definition = match args.definition {
    Some(definition) => definition,
    None => String::from("3d6"),
  };

  let throw_number = match args.throw_number {
    Some(throw_number) => throw_number,
    None => 1_u8,
  };

  let mut r: Roller = definition.parse().unwrap();

  for _ in 1..=throw_number {
    let result = r.roll();
    println!("{:?} => {}", result.dice, result.outcome);
  }
}
