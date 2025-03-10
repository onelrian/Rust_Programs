use crate::{absolute::absolute, multiplication::multi, sign::sign};

pub fn div(mut dividend: f64, mut divisor: f64, decimal_places: usize) -> f64 {
    if divisor == 0.0 {
        println!("Error: Division by zero");
        return 0.0;
    }

    let sign = sign(dividend, divisor);

    dividend = absolute(dividend);
    divisor = absolute(divisor);

    let mut answer = 0.0;
    let mut remainder = dividend;

    while remainder >= divisor {
        remainder -= divisor;
        answer += 1.0;
    }

    // Calculate the decimal part

    let mut decimal_value = 0.0;
    let mut factor = 0.1;

    for _ in 0..decimal_places {
        remainder = multi(remainder, 10.0);
        let mut decimal_digit = 0.0;

        while remainder >= divisor {
            remainder -= divisor;
            decimal_digit += 1.0;
        }

        decimal_value += multi(factor, decimal_digit);
        factor = multi(factor, 0.1);
    }

    let result = answer + decimal_value;
    multi(result, sign as f64)
}