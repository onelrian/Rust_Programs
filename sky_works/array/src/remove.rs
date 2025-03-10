pub fn remove(mut array: [i32; 50], size: usize, position: usize) -> (usize, [i32; 50]) {
    if position > size {
        println!("Invalid position.");
        return (size, array);
    }

    for i in position..size - 1 {
        array[i] = array[i + 1];
    }
    array[size - 1] = 0;

    (size - 1, array)
}
