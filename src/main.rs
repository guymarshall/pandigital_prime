/*
We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?
*/

use rayon::prelude::*;

fn is_prime(number: &i64) -> bool {
    !(2..*number).any(|n| number % n == 0)
}

fn is_pandigital(number: &i64) -> bool {
    let digits: String = number.to_string();

    let one_count: usize = digits.matches("1").count();
    let two_count: usize = digits.matches("2").count();
    let three_count: usize = digits.matches("3").count();
    let four_count: usize = digits.matches("4").count();
    let five_count: usize = digits.matches("5").count();
    let six_count: usize = digits.matches("6").count();
    let seven_count: usize = digits.matches("7").count();
    let eight_count: usize = digits.matches("8").count();
    let nine_count: usize = digits.matches("9").count();

    one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1 && five_count == 1 && six_count == 1 && seven_count == 1 && eight_count == 1 && nine_count == 1
}

fn main() {
    let primes: Vec<i64> = (123456789..=123458899).into_par_iter().filter(is_prime).collect::<Vec<i64>>();

    let pandigital_primes: Vec<i64> = primes.into_par_iter().filter(is_pandigital).collect::<Vec<i64>>();

    println!("Pandigital primes: {:#?}", pandigital_primes);
}