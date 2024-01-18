use std::vec;

use linfa_nn::distance::Distance;
use ndarray::{Dimension, ArrayView};

#[derive(Clone)]
pub struct CosineDistance {}

impl Distance<f64> for CosineDistance {
    fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
        let mut sum = 0.0;
        let mut a_square = 0.0;
        let mut b_square = 0.0;

        for (a, b) in a.iter().zip(b.iter()) {
            sum += a * b;
            a_square += a * a;
            b_square += b * b;
        }

        1.0 - sum / (a_square.sqrt() * b_square.sqrt())
    }
}

#[derive(Clone)]
pub struct LevenshteinDistance {}

impl Distance<f64> for LevenshteinDistance {
    fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
        let a_len = a.len() + 1;
        let b_len = b.len() + 1;

        let mut matrix = vec![vec![0f64]];
        for i in 0..a_len {
            matrix[0].push(i as f64);
        }

        for i in 1..b_len {
            matrix.push(vec![i as f64]);
        }

        let a_vec = a.iter().map(|x| *x).collect::<Vec<f64>>();
        let b_vec = b.iter().map(|x| *x).collect::<Vec<f64>>();

        for j in 1..b_len {
            for i in 1..a_len {
                let number = if a_vec.get(i - 1).unwrap() == b_vec.get(j - 1).unwrap() {
                    matrix[j - 1][i - 1]
                } else {
                    matrix[j - 1][i].min(matrix[j][i - 1]).min(matrix[j - 1][i - 1]) + 1.0
                };

                matrix[j].push(number);
            }
        }

        matrix[a_len - 1][b_len - 1]
    }
}
