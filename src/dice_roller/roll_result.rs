/// A struct holding dice results
#[derive(Debug)]
pub struct RollResult {
    pub dice: Vec<u8>,
    pub successes: Vec<i32>,
    pub outcome: i32,
}

impl RollResult {
    pub fn new(dice: Vec<u8>, outcome: i32) -> Self {
        Self {
            dice,
            outcome,
            successes: Vec::new(),
        }
    }

    pub fn successes(mut self, successes: Vec<i32>) -> Self {
        self.successes = successes;
        self
    }
}
