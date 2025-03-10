use std::env;
use std::process;

use addition::add;
use division::div;
use exponent::expo;
use factorial::fact;
use modulus::modu;
use multiplication::mult;
use subtraction::sub;
use history::{load_history, save_history};


struct Calculation {
    operator: String,
    numbers: Vec<f64>,
    result: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Arguments: {:?}", args); // Debug print

    if args.len() < 3 {
        println!("Use: {} <operator> <num1> [num2 ...]", args[0]);
        process::exit(1);
    }

    let operator = &args[1];
    let numbers: Vec<f64> = args[2..]
        .iter()
        .map(|s| s.parse().unwrap_or_else(|_| {
            println!("Error: Invalid number '{}'", s);
            process::exit(1);
        }))
        .collect();

    let result = match operator.as_str() {
        "add" => {
            if numbers.len() != 2 {
                println!("Error: Addition requires exactly two arguments");
                process::exit(1);
            }
            add(numbers[0], numbers[1])
        }
        "sub" => {
            if numbers.len() != 2 {
                println!("Error: Subtraction requires exactly two arguments");
                process::exit(1);
            }
            sub(numbers[0], numbers[1])
        }
        "mult" => {
            if numbers.len() != 2 {
                println!("Error: Multiplication requires exactly two arguments");
                process::exit(1);
            }
            mult(numbers[0], numbers[1])
        }
        "div" => {
            if numbers.len() != 2 {
                println!("Error: Division requires exactly two arguments");
                process::exit(1);
            }
            if numbers[1] == 0.0 {
                println!("Error: Division by zero");
                process::exit(1);
            }
            div(numbers[0], numbers[1])
        }
        "mod" => {
            if numbers.len() != 2 {
                println!("Error: Modulus requires exactly two arguments");
                process::exit(1);
            }
            modu(numbers[0], numbers[1])
        }
        "expo" => {
            if numbers.len() != 2 {
                println!("Error: Exponentiation requires exactly two arguments");
                process::exit(1);
            }
            expo(numbers[0], numbers[1])
        }
        "fact" => {
            if numbers.len() != 1 {
                println!("Error: Factorial only takes one argument");
                process::exit(1);
            }
            if numbers[0] < 0.0 || numbers[0] != numbers[0].floor() {
                println!("Error: Factorial requires a non-negative integer");
                process::exit(1);
            }
            fact(numbers[0] as u32) as f64
        }
        "exit" => {
            println!("Exiting...");
            process::exit(0);
        }
        _ => {
            println!("Error: Unknown operator");
            process::exit(1);
        }
    };

    let mut history = load_history().unwrap_or_else(|_| Vec::new());
    history.push(Calculation {
        operator: operator.to_string(),
        numbers: numbers.clone(), // Correct usage of `clone`
        result: result,
    });
    if let Err(e) = save_history(&history) {
        eprintln!("Error saving history: {}", e);
    }

    println!("Result: {}", result);

    println!("History:");
    for (i, calc) in history.iter().enumerate() {
        let numbers_str = calc.numbers
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        println!("{}: {} {} = {}", i + 1, calc.operator, numbers_str, calc.result);
    }
}

mod addition;
mod division;
mod exponent;
mod factorial;
mod modulus;
mod multiplication;
mod subtraction;
mod history;