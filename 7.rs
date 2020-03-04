// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
// 13.
// 
// What is the 10 001st prime number?

fn is_prime(number:u32) -> bool {
   
    for i in 2..=(number/2) {
        if number % i == 0 {
            return false;
        }
    }

    true
}

fn main() {

    let mut prime_numbers = Vec::new();
    prime_numbers.push(2);

    let mut i = 3;

    while prime_numbers.len() != 10001 {

        if is_prime(i) {
            prime_numbers.push(i);
        }

        i += 1;
    }

    println!("10001st prime number: {}", prime_numbers.last().unwrap())
}
