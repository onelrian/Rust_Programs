use pair::Pair;

fn main() {

let test = Pair::new(String::from("test1"), 56 as u8);

let swapped = test.swap();



println!("The value is :{:?}",swapped);


}

mod pair;
