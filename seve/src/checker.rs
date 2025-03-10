pub fn checker(vector: &Vec<i32>) -> i32 {
    let length = vector.len();
    let mut mode = vector[0];
    let mut min_freq = 10;

    for i in 0..length {
        let num = vector[i];
        let mut cur_freq = 0;
        for j in 0..length {
            if vector[j] == num {
                cur_freq += 1;
            }
        }

        if cur_freq < min_freq {
            min_freq = cur_freq;
            mode = num;
        }
    }
 
        mode
    
}