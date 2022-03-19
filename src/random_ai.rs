use super::classifier::*;

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

    fn score(&self, x: &Vec<Vec<u8>>, y: &Vec<u8>) -> f64 {
        let mut scores: Vec<bool> = Vec::new();
        let m = y.capacity();

        for i in 0..m {
            let y_pred = self.predict(&x[i]);
            scores.push(y[i] == y_pred);
        }

        let correct = scores
                        .iter()
                        .filter(|&a| *a)
                        .count() as f64;
        let accuracy = correct / m as f64 * 100.0;

        accuracy
    }
}