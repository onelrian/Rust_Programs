pub fn div(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        panic!("Division by zero is not permitted");
    }
    num1 / num2
}