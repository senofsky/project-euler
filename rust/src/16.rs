// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
// 
// What is the sum of the digits of the number 2^1000?

extern crate num_bigint;
extern crate num;

use num_bigint::{BigInt, Sign};
use num::pow::pow;

fn main() {

    let x: BigInt = BigInt::new(Sign::Plus, vec![2]);
    let value = pow(x, 1000);

    let string = value.to_string();

    let mut sum = 0;

    for c in string.chars() {

        println!("c = {}", c);
        let digit = c.to_digit(10).unwrap();
        sum += digit;
    }

    println!("sum = {}", sum);
}

