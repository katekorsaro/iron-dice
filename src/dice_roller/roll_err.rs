#[derive(Debug,PartialEq)]
/// Error returned while parsing the dice notation
pub enum RollerErr {
    None,
    Generic,
    PossibleOverflow,
}
