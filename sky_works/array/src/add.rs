pub fn add(mut array: [i32; 50], size: usize, value: i32, position: usize) -> (usize, [i32; 50]) {
    if position > size || size > 50 {
        println!("Invalid position.");
        return (size, array);
    }

    let mut count = size - 1;
    while count >= position {
        array[count + 1] = array[count];

        count -= 1;
    }

    array[position] = value;

    (size + 1, array)
}
