pub fn exclude(target: i32, data: [i32; 50], size: usize) -> ([i32; 50], usize) {
    let mut result: [i32; 50] = [0; 50];
    let mut index = 0;

    for i in 0..size {
        if data[i] != target {
            result[index] = data[i];
            index += 1;
        }
    }

    (result, index)
}