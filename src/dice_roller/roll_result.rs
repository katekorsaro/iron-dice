/// A struct holding dice results
#[derive(Debug)]
pub struct RollResult {
    pub dice: Vec<u8>,
    pub successes: Vec<i8>,
    pub outcome: i16,
}

impl RollResult {
    pub fn new(dice: Vec<u8>, outcome: i16) -> Self {
        Self {
            dice,
            outcome,
            successes: Vec::new(),
        }
    }

    pub fn successes(mut self, successes: Vec<i8>) -> Self {
        self.successes = successes;
        self
    }
}
