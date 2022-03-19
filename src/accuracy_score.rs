use crate::classifier::Classifier;

pub fn accuracy_score<C: Classifier>(x: &Vec<Vec<u8>>, y: &Vec<u8>, classifier: C) -> f64 {
    let mut scores: Vec<bool> = Vec::new();
    let m = y.capacity();

    for i in 0..m {
        let y_pred = classifier.predict(&x[i]);
        scores.push(y[i] == y_pred);
    }

    let correct = scores
                    .iter()
                    .filter(|&a| *a)
                    .count() as f64;
    let accuracy = correct / m as f64 * 100.0;

    accuracy
}