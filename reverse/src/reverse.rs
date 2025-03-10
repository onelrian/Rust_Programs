use std::io;


pub fn modulus(mut number:i32) -> i32{
   
    let mut reversed = 0;
    while number != 0 {
        let modulus = number % 10;
        reversed = reversed * 10 + modulus;
        number /= 10;
    }

    reversed
}

pub fn inverse() -> i32 {
    println!("Enter A number !");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("Failed to Read Input");

    let read: i32 = num.trim().parse().expect("Invalid Input");

    modulus(read)
//     if read < 0 {
//        modulus(read) * -1
//     } else {
//     }
}
