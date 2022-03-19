use crate::classifier::Classifier;

// A perceptron designed to handle multi-class prediction problems
pub struct Perceptron {
    weights: [Vec<f64>; 10],
    iters: u32,
    classes: [u8; 10]
}

impl Classifier for Perceptron {
    fn new() -> Perceptron {
        // Create a weight vector for each class
        const W_VEC: Vec<f64> = Vec::new();

        Perceptron {
            weights: [W_VEC; 10],
            iters: 10,
            classes: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }

    fn fit(&mut self, x: &Vec<Vec<u8>>, y: &Vec<u8>) {
        self.init_weights(&x);
        for _ in 0..self.iters {
            for i in 0..x.capacity() {
                let y_pred = self.predict(&x[i]);
                
                if y[i] != y_pred {
                    for j in 0..self.weights[0].capacity() {
                        self.weights[y_pred as usize][j] -= x[i][j] as f64;
                        self.weights[y[i] as usize][j] += x[i][j] as f64;
                    }
                }
            }
        }
    }

    // Return the class that yeilds the highest product
    fn predict(&self, xi: &Vec<u8>) -> u8 {
        let mut arg_max = 0.0;
        let mut y_pred: u8 = self.classes[0].into();

        for c in self.classes.iter() {
            let f = self.get_dot_product(xi, *c);
            if f >= arg_max {
                arg_max = f;
                y_pred = *c;
            }
        }

        y_pred
    }
}

impl Perceptron {
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
    fn get_dot_product(&self, xi: &Vec<u8>, c: u8) -> f64 {
        let mut sum = 0.0;
        for i in 0..xi.capacity() {
            sum += self.weights[c as usize][i] * xi[i] as f64;
        }

        sum
    }
}