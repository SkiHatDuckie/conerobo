use rand::prelude::*;

// An 'AI' that guesses Y at complete random
pub struct RandomAI;

impl RandomAI {
    // Returns a vector of the predicted y values.
    pub fn predict(self: &Self) -> u8 {
        let mut rng = thread_rng();

        let y_pred: u8 = rng.gen_range(0..=9);

        y_pred
    }
}