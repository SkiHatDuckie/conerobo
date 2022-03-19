use super::classifier::*;

// A perceptron designed to handle multi-class prediction problems
pub struct Perceptron {
    weights: [Vec<f64>; 10],
    eta: f64,
    epoch: u32,
    classes: [u8; 10]
}

impl Perceptron {
    // Set the rate of learning (default 0.1)
    pub fn set_eta(&mut self, new_eta: f64) {
        self.eta = new_eta;
    }

    // Initialize weight vectors for each class
    // Where `weights[n][0]` is the bias
    fn init_weights(&mut self, x: &Vec<Vec<u8>>) {
        // Num of dimensions (features) in each x
        let d = x[0].capacity();

        for c in self.classes.iter() {
            self.weights[*c as usize] = vec![0.0; d];
        }
    }

    // Multiply the weights by the inputs and sum them up.
    // Return the dot product.
    fn get_dot_product(&self, xi: &Vec<u8>) -> f64 {
        let mut sum = 0.0;
        for c in 0..self.weights.len() {
            for i in 0..xi.capacity() {
                sum += self.weights[c][i] * xi[i] as f64;
            }
        }

        sum
    }
}

impl Classifier for Perceptron {
    fn new() -> Perceptron {
        // Create a weight vector for each class
        const W_VEC: Vec<f64> = Vec::new();

        Perceptron {
            weights: [W_VEC; 10],
            eta: 0.1,
            epoch: 1,
            classes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }

    fn fit(&mut self, x: &Vec<Vec<u8>>, y: &Vec<u8>) {
        self.init_weights(&x);
        for _ in 0..self.epoch {
            for i in 0..x.capacity() {
                let y_pred = self.predict(&x[i]);
                
                // Update weights if incorrect prediction
                if y[i] != y_pred {
                    for j in 0..self.weights[y_pred as usize].capacity() {
                        self.weights[y_pred as usize][j] += self.eta * (y[i] as f64 - y_pred as f64) * x[i][j] as f64;
                    }
                }
            }
        }
        
    }

    // Return the class that yeilds the highest product
    fn predict(&self, xi: &Vec<u8>) -> u8 {
        let mut arg_max = 0.0;
        let mut y_pred: usize = self.classes[0].into();

        for c in self.classes.iter() {
            let f = self.get_dot_product(xi);
            if f >= arg_max {
                arg_max = f;
                y_pred = *c as usize;
            }
        }

        y_pred as u8
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