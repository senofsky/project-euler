// A perfect number is a number for which the sum of its proper divisors is exactly equal to the
// number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28,
// which means that 28 is a perfect number.
// 
// A number n is called deficient if the sum of its proper divisors is less than n and it is called
// abundant if this sum exceeds n.
// 
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
// written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
// all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
// upper limit cannot be reduced any further by analysis even though it is known that the greatest
// number that cannot be expressed as the sum of two abundant numbers is less than this limit.
// 
// Find the sum of all the positive integers which cannot be written as the sum of two abundant
// numbers.

use std::collections::HashMap;

fn is_abundant(num:u32) -> bool {

    let mut sum = 0;

    for i in 1..num {
        if num % i == 0 {
            sum += i;
        }
    }

    if sum > num {
        return true
    }

    false
}

fn can_be_expressed_as_sum(num:u32, abundant_numbers:&HashMap<u32, bool>) -> bool {

    for i in (1..num).rev() {

        let j = num-i;

        if *(abundant_numbers.get(&i).unwrap()) && *(abundant_numbers.get(&j).unwrap()) {
            if i + j == num {
                return true;
            }
        }
    }

    false
}

fn calculate_abundant_numbers() -> HashMap<u32, bool> {

    let mut abundant_numbers = HashMap::new();

    for i in 1..28123 {

        if is_abundant(i) {
           abundant_numbers.insert(i, true); 
        }
        else {
           abundant_numbers.insert(i, false); 
        }
    }

    abundant_numbers
}

fn main() {

    let mut numbers_that_cant_be_summed = Vec::new();

    let abundant_numbers = calculate_abundant_numbers();
    println!("done calculating abundant numbers");

    for i in 1..28123 {
        
        if !can_be_expressed_as_sum(i, &abundant_numbers) {
            numbers_that_cant_be_summed.push(i);
        }

        if i % 100 == 0 {
            println!("i = {}", i);
        }

    }

    let sum:u32 = numbers_that_cant_be_summed.iter().sum();

    println!("sum = {}", sum)
}
