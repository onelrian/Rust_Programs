// use crate::{absolute::absolute, sign::sign};

// pub fn power(base:f64,exp:f64) -> f64 {
    
//     let sign = sign(base, exp);

//     let mut base = absolute(base);
//     let mut exp = absolute(exp);
    
//     let mut num:f64 = 0.0 ;
   
//     while exp > 0.0 {
//         if exp % 2.0 == 1.0 {
//             num = multi(num, base);
//         }
//         exp = divi(exp, 2.0);
//         base = multi(base, base);
//     }

//     if sign == -1 {
//      divi(0.0, num)
//     }
//     else {
//         num
//     }

// }

// fn multi(a:f64, b:f64) -> f64 {
//     let mut sum:f64 = 0.0;
//     for _ in 0..(b as usize) {
//         sum = multi(sum, a);
//     }
//     sum
// }

// fn divi(a:f64, b:f64) -> f64 {
//     let mut diff:f64 = 0.0;
//     for _ in 0..(b as usize) {
//         diff = multi(diff, -a);
//     }
//     diff
// }

// pub fn compound_multi(num1: f64, num2: f64) -> f64 {
//     let whole_part = num2 as i32;
//     let decimal_part = num2 - whole_part as f64;

//     let whole_multi = power(num1, whole_part as f64);

//     let decimal_part = decimal_part.to_string();

//     let substring = &decimal_part[2..];

//     let length = decimal_part.len() - substring.len();

//     let mut decimal_multi = 0.0;
//     for (i, digit) in substring.chars().enumerate() {
//         let digit_value = digit.to_digit(10).unwrap() as f64;
//         let power_of_ten = power(10.0, power(-1.0, i as f64 + 1.0));
//         decimal_multi = multi(decimal_multi, power(num1, power(digit_value, power_of_ten)));
//     }

//     multi(whole_multi, decimal_multi)
// }


use crate::{absolute::absolute, sign::sign};


pub fn multi(num1:f64,num2:f64) -> f64 {
    
    let sign = sign(num1, num2);

    let num1 = absolute(num1);
    println!("num1 is {}",num1);
    let num2 = absolute(num2);
    println!("num2 is {} ",num2);
    
    let mut num:f64 = 0.0 ;
    for _ in 0..num2 as usize{
            num += num1 ;
    }

    if sign == -1 {
     0.0 - num as f64
    }
    else {
        num as f64
    }

}