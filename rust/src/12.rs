// The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle
// number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
// 
// 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
// 
// Let us list the factors of the first seven triangle numbers:
// 
//      1: 1
//      3: 1,3
//      6: 1,2,3,6
//     10: 1,2,5,10
//     15: 1,3,5,15
//     21: 1,3,7,21
//     28: 1,2,4,7,14,28
// 
// We can see that 28 is the first triangle number to have over five divisors.
// 
// What is the value of the first triangle number to have over five hundred divisors?

fn get_factors(number:u64) -> Vec<u64> {

    let mut factors = Vec::new();

    let mut number = number;
    let mut i = 1;

    let limit = (number as f64).sqrt() as u64;
    println!("limit = {}", limit);

    while i <= limit {
        if number % i == 0 {
            factors.push(i);
         //   number /= i;
            factors.push(number);
        }

        // println!("i = {}, number = {}", i, number);

        i += 1;
    }

    factors
}

//mod sieve_of_eratosthenes;

/*
fn get_number_of_factors(primes:&Vec<u64>, number:u64) -> u64 {


    let mut number = number;
    let mut number_of_factors = 2; // 1 and itself

    for prime in primes {

        if *prime > number {
            break;
        }

        while number % prime == 0 {
            number_of_factors += 1;
            number /= prime;
        }
    }

    number_of_factors
}
*/

fn main() {

    let mut triangle_number = 0;
    let mut number_of_factors = 0;

    //let primes = sieve_of_eratosthenes::sieve(10);
    //println!("done creating sieve");

    let mut i:u64 = 1;
    while number_of_factors < 500 {

        triangle_number += i;

        //number_of_factors = get_number_of_factors(&primes, triangle_number);
        let factors = get_factors(triangle_number);
        println!("factors = {:?}", factors);
        number_of_factors = factors.len();
        i += 1;

        if number_of_factors % 50 == 0 {
            println!("t = {}, n = {}", triangle_number, number_of_factors);
        }
    }

    println!("triangle_number = {}", triangle_number);
}
