/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
*/

mod math;
mod user_input;

fn main() {
    // let mut prime_number_count: i32 = 0;
    // let mut number: i32 = 2;

    // while prime_number_count < user_input {
    //     let is_prime: bool = math::is_prime_number(number);
    //     if is_prime {
    //         prime_number_count += 1;
    //         println!("{}: {}", prime_number_count, number);
    //     }
    //     number += 1;
    // }

    // 9 digit
    // 100_000_000
    // 123456789

    let mut number: i32 = 123456789;
    let max: i32 = 987654321;
    let mut primes: Vec<i32> = Vec::new();

    while number <= max {
        let is_prime: bool = math::is_prime_number(number);
        if is_prime {
            primes.push(number);
        }
        number += 1;
    }

    println!("Primes: {:#?}", primes);
}
