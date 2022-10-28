pub fn is_prime_number(number: i32) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let root_of_number: i32 = (number as f64).sqrt() as i32;
    for i in (3..root_of_number).step_by(2) {
        if number % i == 0 {
            return false
        }
    }

    return true;
}