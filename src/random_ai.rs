use crate::classifier::Classifier;

use rand::prelude::*;

// An 'AI' that guesses Y at complete random
pub struct RandomAI;

impl Classifier for RandomAI {
    fn new() -> RandomAI {
        RandomAI
    }

    fn fit(&mut self, _x: &Vec<Vec<u8>>, _y: &Vec<u8>) {}

    fn predict(&self, _xi: &Vec<u8>) -> u8 {
        let mut rng = thread_rng();

        let y_pred: u8 = rng.gen_range(0..=9);

        y_pred
    }
}