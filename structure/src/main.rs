// use calculator::{operator::Operators, Calculator, Operation};
// use point::{Point, ThreeDPoint};
// use std::env;

struct Number {
    odd: bool,
    value: i32,
}
struct Idea {
    odd: bool,
    number: Number
}

fn main() {
    let one = Number {
        odd: true,
        value: 1,
    };
    let two = Number {
        value: 2,
        ..one
    };
    let idea = Idea { odd: true, number: two };
    let Idea { odd, number: num } = idea;
    let Number {odd: isNumberOdd, value } = num;
    println!("{} similar to {}", odd, num.odd);
    print_number_odd(one);
    print_number_odd(num);
}

fn print_number_odd(n: Number) {
    let Number { odd, value } = n;
    if odd == true {
        println!("Odd number: {}", value);
    } else if let Number { odd: false, value } = n {
        println!("Even number: {}", value);
    }
}

fn print_number_value(n: Number) {
    if let Number { odd, value: 1 } = n {
        println!("One");
    } else if let Number { odd, value: 2 } = n {
        println!("Two");
    } else {
        let Number { value, .. } = n;
        // println!("{}", value);
    }

    let Number { odd, value } = n;
    if value == 1 {
        println!("One");
    } else if value == 2 {
        println!("Two");
    } else {
        println!("{}", value);
    }
    match n {
        Number { value: 1, .. } => println!("One"),
        Number { value: 2, .. } => println!("Two"),
        Number { value, .. } => println!("{}", value),
        // if that last arm didn't exist, we would get a compile-time error
    }
}
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let mut calculator = Calculator::new();
//     // let sum = calculator.sub(12.0, 15.0);
//     let args = &args[1..];

//     let is_expression = args
//         .iter()
//         .any(|arg| ["+", "-", "*", "/", "%"].contains(&arg.as_str()));

//     if is_expression {
//         eprintln!("Unhandled expression!")
//     } else {
//         let op: Operators = args[0].trim().parse().expect("Invalid Operation");
//         let args: Vec<f64> = args[1..]
//             .into_iter()
//             .map(|arg| arg.parse().expect("Invalid number input."))
//             .collect();

//         let operations = match op {
//             Operators::Add => Operation::Add(args),
//             Operators::Sub => Operation::Sub(args),
//             Operators::Mul => Operation::Mul(args),
//             Operators::Div => {
//                 let (a, b) = get_div_args(&args);
//                 Operation::Div(a, b)
//             }
//             Operators::Mod => {
//                 let (a, b) = get_div_args(&args);
//                 Operation::Mod(a as u64, b as u64)
//             }
//             Operators::Fact => Operation::Fact(*args.get(0).expect("Please enter a number") as u128),
//         };

//         calculator.calculate(operations);
//     }

//     println!("Result: {:?}", calculator.last_result)
// }

// fn get_div_args(args: &Vec<f64>) -> (f64, f64) {
//     let a = args.get(0).expect("Please enter a divisor");
//     let b = args.get(1).expect("Please enter divident");
//     (*a, *b)
// }

// mod calculator;
// mod guess;
// mod point;
