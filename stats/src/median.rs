use crate::calculation::Calculation;

pub struct Median;

impl Calculation for Median {
    fn calc(vector: Vec<i32>) -> f64 {
        let mut sorted = vector.clone();
        let length = sorted.len();

        for i in 0..length {
            for j in 0..length {
                if sorted[i] < sorted[j] {
                    let temp = sorted[i];
                    sorted[i] = sorted[j];
                    sorted[j] = temp;
                }
            }
        }

        if (length % 2) == 1 {
            sorted[length / 2] as f64
        } else {
            let mid1 = sorted[length / 2] - 1;
            let mid2 = sorted[length / 2];
            (mid1 + mid2) as f64 / 2.0
        }
    }
}
