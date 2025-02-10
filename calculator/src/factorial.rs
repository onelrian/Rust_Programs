pub fn fact(num: u32) -> u64 {
    if num == 0 {
        1
    } else {
        (1..=num as u64).product()
    }
}