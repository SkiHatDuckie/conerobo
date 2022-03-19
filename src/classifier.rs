// A trait with methods for implementing a classifier
// using the statistical learning framework.
pub trait Classifier {
    fn new() -> Self;

    fn fit(&mut self, x: &Vec<Vec<u8>>, y: &Vec<u8>);

    fn predict(&self, xi: &Vec<u8>) -> u8;
}