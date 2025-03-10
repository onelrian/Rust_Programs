pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
#[test]
fn test_sum() {
    assert!(sum(12, 13) == 25);
    assert_eq!(sum(2, 3),5)
}
