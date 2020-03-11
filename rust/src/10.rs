// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
// 
// Find the sum of all the primes below two million.

fn is_prime(number:u64) -> bool {
   
    for i in 2..=(number/2) {
        if number % i == 0 {
            return false;
        }
    }

    true
}

fn main() {

    let mut prime_numbers = Vec::new();

    for i in 2..2000000 {

        if is_prime(i) {
            prime_numbers.push(i);
        }
    }

    println!("sum = {}", prime_numbers.iter().sum::<u64>())
}
