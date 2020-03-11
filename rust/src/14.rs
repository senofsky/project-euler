// The following iterative sequence is defined for the set of positive integers:
// 
// n → n/2 (n is even) n → 3n + 1 (n is odd)
// 
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
// 
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers
// finish at 1.
// 
// Which starting number, under one million, produces the longest chain?
// 
// NOTE: Once the chain starts the terms are allowed to go above one million.

fn get_number_of_collatz_terms(number:u64) -> u64 {

    let mut number = number;
    let mut number_of_terms = 1;

    while number > 1 {

        if number % 2 == 0 {
            number /= 2;
        }
        else {
            number = 3*number + 1;
        }

        number_of_terms += 1;
    }

    number_of_terms
}

fn main() {

    let mut start_number = 0;
    let mut longest_terms = 0;

    for i in 1..1000000 {

        let number_of_terms = get_number_of_collatz_terms(i);
        
        if number_of_terms > longest_terms {
            longest_terms = number_of_terms;
            start_number = i
        }
    }

    println!("start_number = {}", start_number);
}
