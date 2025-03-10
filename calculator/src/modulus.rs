pub fn modu(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        panic!("Modulus by zero is not permitted");
    }
    num1 - (num2 * (num1 / num2).floor())
}

// mod(a,b)=a−b⋅⌊a/b⌋
