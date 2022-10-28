/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
*/

use rayon::prelude::*;

pub fn is_prime(number: &i32) -> bool {
    !(2..*number).any(|n| number % n == 0)
}

fn main() {
    let mut number: i32 = 123456789;
    let max: i32 = 987654321;
    let mut primes: Vec<i32> = Vec::new();

    let primes: Vec<i32> = (123456789..=987654321).into_par_iter().filter(is_prime).collect::<Vec<i32>>();

    println!("Primes: {:#?}", primes);
}
