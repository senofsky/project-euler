// The Fibonacci sequence is defined by the recurrence relation:
// 
//     Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
// 
// Hence the first 12 terms will be:
// 
//     F1 = 1
//     F2 = 1
//     F3 = 2
//     F4 = 3
//     F5 = 5
//     F6 = 8
//     F7 = 13
//     F8 = 21
//     F9 = 34
//     F10 = 55
//     F11 = 89
//     F12 = 144
// 
// The 12th term, F12, is the first term to contain three digits.
// 
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

extern crate num_bigint;
extern crate num_traits;

use num_bigint::{BigInt};
use num_traits::{One};
use std::mem::replace;

fn main() {

    let mut f0: BigInt = One::one();
    let mut f1: BigInt = One::one();
    let mut i = 3;

    loop {

        let f2 = f0 + &f1;
        println!("{}", f2);
        if f2.to_string().chars().count() >= 1000 {
            break;
        }

        f0 = replace(&mut f1, f2);


        i += 1;
    }

    println!("index = {}", i);
}
