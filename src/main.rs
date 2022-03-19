mod mnist;
mod random_ai;
mod perceptron;
pub mod classifier;

use mnist::*;
use random_ai::*;
use perceptron::*;
use classifier::Classifier;

fn main() {
    println!("Process MNIST dataset\n");
    let x_train = read_mnist_image("mnist/train-images.idx3-ubyte");
    let y_train = read_mnist_label("mnist/train-labels.idx1-ubyte");
    let x_test = read_mnist_image("mnist/t10k-images.idx3-ubyte");
    let y_test = read_mnist_label("mnist/t10k-labels.idx1-ubyte");

    println!("Instantiating classifier objects\n");
    let random_ai = RandomAI;
    let mut perceptron = Perceptron::new();
    let mut perceptron2 = Perceptron::new();
    perceptron2.set_eta(1.0);

    println!("Train classifier objects\n");
    perceptron.fit(&x_train, &y_train);
    perceptron2.fit(&x_train, &y_train);

    println!("Test classifier objects\n");
    let random_ai_score = random_ai.score(&x_test, &y_test);
    let perceptron_score = perceptron.score(&x_test, &y_test);
    let perceptron_score2 = perceptron2.score(&x_test, &y_test);

    println!("Results: [{} predictions]", y_test.capacity());
    println!("Classifier           | Accuracy (%)");
    println!("-----------------------------------");
    println!("Random               | {:.3}%", random_ai_score);
    println!("Perceptron [eta=0.1] | {:.3}%", perceptron_score);
    println!("Perceptron [eta=1.0] | {:.3}%", perceptron_score2)
}