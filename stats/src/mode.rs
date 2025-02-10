use crate::calculation::Calculation;

pub struct Mode;
impl Calculation for Mode {
    fn calc(vector: Vec<i32>) -> f64 {
        let mut cloned = vector.clone();

        let length = cloned.len();
        let mut mode = vector[0];
        let mut max_freq = 1;

        for i in 0..length {
            let num = vector[i];
            let mut cur_freq = 0;
            for j in 0..length {
                if vector[j] == num {
                    cur_freq += 1;
                }
            }

            if cur_freq > max_freq {
                max_freq = cur_freq;
                mode = num;
            }
        }
        mode as f64
    }
}
