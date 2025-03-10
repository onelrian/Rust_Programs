pub fn inverse(array: &mut [i32; 50], size: usize) -> (usize, [i32; 50]) {
    let mut i = 0;
    let mut j = size - 1;

    while i < j {
        let temp = array[i];
        array[i] = array[j];
        array[j] = temp;

        i += 1;
        j -= 1;
    }

   /*  let mut copied_array = [0; 50];
    for i in 0..size {
        copied_array[i] = array[i];
    }*/


    (size, *array)
}
