use crate::classifier::Classifier;

// Get the accuracy score for a classifier.
// score = correct_predictions / all_predictions.
pub fn accuracy_score<C: Classifier>(x: &Vec<Vec<u8>>, y: &Vec<u8>, classifier: &C) -> f64 {
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

// A more detailed metric that sorts each datapoint by the predicted y (rows 0-9)
// and the actual y (cols 0-9)
//
// For example: 
// A datapoint added to [3][4] means that the classifier guessed that the number
// was 3, when it was actually 4
pub fn confusion_matrix<C: Classifier>(x: &Vec<Vec<u8>>, y: &Vec<u8>, classifier: &C) -> [[u32; 10]; 10] {
    let mut cfn_matrix = [[0u32; 10]; 10];
    let m = y.capacity();

    for i in 0..m {
        let y_pred = classifier.predict(&x[i]);
        cfn_matrix[y_pred as usize][y[i] as usize] += 1;
    }

    cfn_matrix
}