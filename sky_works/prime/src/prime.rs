pub fn prime(target:u128) {
    println!("The Prime numbers in The Range {} are :", target);

    for i in 2..=target{
        let mut is_prime = true;
        for j in 2..((i / 2) + 1 as u128) {
            if i%j == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            println!("{}",i);
        }
    }
}