fn main() {
    println!("Hello, world!");
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn number_to_string(num: i32)-> String{
    num.to_string()
}

mod test{
    use super::*;



#[test]
fn test_sum() {
    assert!(sum(12, 13) == 25);
    assert_eq!(sum(2, 3),5)
}

#[test]
fn test_num_to_string() {
    let num = 25;
    assert_eq!(number_to_string(num),num.to_string());
    assert_ne!(number_to_string(num),25.to_string());
}

#[test]
#[ignore = "Void function"]
fn test_main() {
    assert_eq!(main(),())
}

}