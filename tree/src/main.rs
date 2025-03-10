use std::io;

fn main() {
    
    println!("Enter the height");
    let mut height = String::new();
    io::stdin()
    .read_line(&mut height)
    .expect("Invalid Input");

    let height:u32 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
        };

        println!("Enter The width");
        let mut width = String::new();
        io::stdin()
        .read_line(&mut width)
        .expect("Invalid Input");

    let width:u32 = match width.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
    };

    if height > width {
        println!("Not Possible");
    } else {

        for i in 0..width {
            for j in 0..(height-i) {
                print!(" ");
            }
            for _ in 0..(2*i+1) {
                print!("*");
            }
            println!();
        }
    }
}
    

