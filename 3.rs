// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

fn main() {

    //let mut number = 13195;
    let mut number:u64 = 600851475143;
    let mut prime_factors = Vec::new();

    let mut i = 1;
    while number != 1 {

        if number % i == 0 {
            println!("found prime factor : {}", i);
            prime_factors.push(i);
            number /= i;
        }
      
        i += 1
    }

    println!("prime_factors = {:?}", prime_factors);
}
