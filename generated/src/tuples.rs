pub fn swap_tuple(input: (i32, f64)) -> (f64, i32) {
    let (interger,float) = input;
    let input = (float,interger);
    input
}

pub fn tuple_sum(input: (i32, i32, i32)) -> i32 {
    let sum = input.0 + input.1 + input.2;
    sum
}