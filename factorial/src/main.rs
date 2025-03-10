use factorial::Factorial;
mod factorial;

fn main() {
    println!("Factorial : {},{},{},{}",
    3_u8.fact(),
    15_u16.fact(),
    16_u32.fact(),
    17_u32.fact());
    println!("Hello, world!");
    
    
}
