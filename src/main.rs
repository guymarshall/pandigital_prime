/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
*/

mod math;
mod user_input;

fn main() {
    let user_input: u128 = user_input::get_user_input("Enter a positive integer: ");

    // maybe do logic so if number above a certain amount, move from u8 to u16, u32 etc

    let mut prime_number_count: u128 = 0;
    let mut number: u128 = 2;

    while prime_number_count < user_input {
        let is_prime: bool = math::is_prime_number(number);
        if is_prime {
            prime_number_count += 1;
            println!("{}: {}", prime_number_count, number);
        }
        number += 1;
    }
}
