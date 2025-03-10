pub fn sign(num1: f64 , num2: f64) -> i8 {
    
    let sign = if (num1 < 0.0 && num2 > 0.0) || (num1 > 0.0 && num2 < 0.0) {
        -1
    } else {
        1
    };

    sign
}