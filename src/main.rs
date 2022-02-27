mod mnist;

fn main() {
    println!("Process MNIST dataset");

    println!("Read and store MNIST training set images");
    let train_images = mnist::read_mnist_image("mnist/train-images.idx3-ubyte");

    println!("Read and store MNIST training set labels");
    let train_labels = mnist::read_mnist_label("mnist/train-labels.idx1-ubyte");

    println!("Read and store MNIST testing set images");
    let test_images = mnist::read_mnist_image("mnist/t10k-images.idx3-ubyte");

    println!("Read and store MNIST testing set labels");
    let test_labels = mnist::read_mnist_label("mnist/t10k-labels.idx1-ubyte");
}