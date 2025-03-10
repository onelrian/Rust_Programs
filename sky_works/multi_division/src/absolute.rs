pub fn absolute(num:f64) -> f64 {
    if num < 0.0 {
        0.0 - num
    } else {
        num
    }
}