/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
*/

use rayon::prelude::*;

mod math;
mod user_input;

fn main() {
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
