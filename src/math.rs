pub fn is_prime_number(number: u128) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let root_of_number: u128 = (number as f64).sqrt() as u128;
    for i in (3..root_of_number).step_by(2) {
        if number % i == 0 {
            return false
        }
    }

    return true;
}