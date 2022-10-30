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
    let digit_count: usize = digits.len();

    let one_count: usize = digits.matches("1").count();
    let two_count: usize = digits.matches("2").count();
    let three_count: usize = digits.matches("3").count();
    let four_count: usize = digits.matches("4").count();
    let five_count: usize = digits.matches("5").count();
    let six_count: usize = digits.matches("6").count();
    let seven_count: usize = digits.matches("7").count();
    let eight_count: usize = digits.matches("8").count();
    let nine_count: usize = digits.matches("9").count();

    match digit_count {
        1 => one_count == 1,
        2 => one_count == 1 && two_count == 1,
        3 => one_count == 1 && two_count == 1 && three_count == 1,
        4 => one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1,
        5 => one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1 && five_count == 1,
        6 => one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1 && five_count == 1 && six_count == 1,
        7 => one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1 && five_count == 1 && six_count == 1 && seven_count == 1,
        8 => one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1 && five_count == 1 && six_count == 1 && seven_count == 1 && eight_count == 1,
        9 => one_count == 1 && two_count == 1 && three_count == 1 && four_count == 1 && five_count == 1 && six_count == 1 && seven_count == 1 && eight_count == 1 && nine_count == 1,
        _ => false
    }
}

fn main() {
    let pandigital_numbers: Vec<i64> = (2..=987654321).into_par_iter().filter(is_pandigital).collect::<Vec<i64>>();
    println!("Finished collecting pandigital numbers. Searching for primes...");
    
    let reversed_pandigital_numbers: Vec<i64> = pandigital_numbers.into_par_iter().rev().collect();

    for number in reversed_pandigital_numbers {
        if is_prime(&number) {
            println!("{}", number);
            break;
        }
    }
}