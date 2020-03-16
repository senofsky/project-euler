// n! means n × (n − 1) × ... × 3 × 2 × 1
// 
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
// 
// Find the sum of the digits in the number 100!

extern crate num_bigint;

use num_bigint::{BigUint};
use num_traits::{One};
use num_iter;

fn factorial(number:u64) -> BigUint {

    let mut product = One::one();

    for i in num_iter::range_inclusive(BigUint::from(1u64), BigUint::from(number)) {
        product *= i;
    }

    product
}

fn main () {

    let factorial = factorial(100u64);
    println!("factorial = {:?}", factorial);

    let factorial = factorial.to_str_radix(10);
    println!("digits = {:?}", factorial);

    // cannot infer type for `B` error
    //let sum = factorial.chars().map(|x| x.to_digit(10).unwrap()).collect().iter().sum();

    let mut sum = 0;
    for digit in factorial.chars() {
       sum += digit.to_digit(10).unwrap(); 
    }
    //let sum:BigUint = values.iter().sum();

    println!("sum = {:?}", sum);
}
