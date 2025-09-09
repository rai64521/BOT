use linfa::prelude::*;
use linfa_linear::LinearRegression;
use ndarray::Array2;

fn main() {
    // Training data: y = 2x + 1
    let x = Array2::from_shape_vec((5, 1), vec![1., 2., 3., 4., 5.]).unwrap();
    let y = Array2::from_shape_vec((5, 1), vec![3., 5., 7., 9., 11.]).unwrap();

    // Fit the model
    let dataset = Dataset::new(x, y);
    let model = LinearRegression::default().fit(&dataset).unwrap();

    // Predict new data
    let test = Array2::from_shape_vec((2, 1), vec![6., 7.]).unwrap();
    let predictions = model.predict(&test);

    println!("Predictions: {:?}", predictions);
}