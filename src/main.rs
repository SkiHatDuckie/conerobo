mod mnist;
mod random_ai;

use mnist::*;
use random_ai::*;

fn main() {
    println!("Process MNIST dataset");

    println!("Read and store MNIST training set images");
    let x_train = read_mnist_image("mnist/train-images.idx3-ubyte");

    println!("Read and store MNIST training set labels");
    let y_train = read_mnist_label("mnist/train-labels.idx1-ubyte");

    println!("Read and store MNIST testing set images");
    let x_test = read_mnist_image("mnist/t10k-images.idx3-ubyte");

    println!("Read and store MNIST testing set labels");
    let y_test = read_mnist_label("mnist/t10k-labels.idx1-ubyte");

    println!("\nTest classifiers");
    println!("Instantiate Random 'AI'");
    let random_ai = RandomAI;

    println!("Score Random 'AI'");
    let mut accuracy_scores: Vec<bool> = Vec::new();
    for i in 0..x_test.capacity() {
        let y_pred = random_ai.predict();
        accuracy_scores.push(y_test[i] == y_pred);
    }
    let accuracy = accuracy_scores.iter().filter(|&a| *a).count() as f64;
    let score: f64 = accuracy / y_test.capacity() as f64 * 100.0;

    println!("\nResults: [{} predictions]", y_test.capacity());
    println!("Classifier | Accuracy (%)");
    println!("-------------------------");
    println!("Random     | {:.3}%", score);
}