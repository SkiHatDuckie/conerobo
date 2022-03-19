mod accuracy_score;
pub mod classifier;
mod mnist;
mod random_ai;
mod perceptron;

use accuracy_score::accuracy_score;
use classifier::Classifier;
use mnist::*;
use random_ai::RandomAI;
use perceptron::Perceptron;

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
    let random_ai_score = accuracy_score(&x_test, &y_test, random_ai);
    let perceptron_score = accuracy_score(&x_test, &y_test, perceptron);
    let perceptron_score2 = accuracy_score(&x_test, &y_test, perceptron2);

    println!("Results: [{} predictions]", y_test.capacity());
    println!("Classifier           | Accuracy (%)");
    println!("-----------------------------------");
    println!("Random               | {:.3}%", random_ai_score);
    println!("Perceptron [eta=0.1] | {:.3}%", perceptron_score);
    println!("Perceptron [eta=1.0] | {:.3}%", perceptron_score2)
}