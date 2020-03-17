// Euler discovered the remarkable quadratic formula:
// 
// n2+n+41
// 
// It turns out that the formula will produce 40 primes for the consecutive integer values 0≤n≤39 .
// However, when n=40,402+40+41=40(40+1)+41 is divisible by 41, and certainly when n=41,412+41+41
// 
// is clearly divisible by 41.
// 
// The incredible formula n2−79n+1601 was discovered, which produces 80 primes for the consecutive
// values 0≤n≤79
// 
// . The product of the coefficients, −79 and 1601, is −126479.
// 
// Considering quadratics of the form:
// 
//     n2+an+b
// 
// , where |a|<1000 and |b|≤1000
// 
// where |n| is the modulus/absolute value of n e.g. |11|=11 and |−4|=4
// 
// Find the product of the coefficients, a and b, for the quadratic expression that produces the
// maximum number of primes for consecutive values of n, starting with n=0.

fn is_prime(number:u64, primes:&Vec<u64>) -> bool {
    primes.iter().any(|element| *element == number)
}

struct Prime {
    value: u64,
    is_prime: bool,
}

fn sieve(limit:u64) -> Vec<u64> {

    let mut primes:Vec<Prime> = Vec::new();

    for i in 2..limit {
        primes.push(Prime{value:i, is_prime:true});
    }

    let mut i:usize = 0;
    loop {

        while i < primes.len() && primes[i].is_prime == false {
            i+=1;
        }

        if i >= primes.len() {
            break;
        }

        let step = primes[i].value as usize;

        let mut j = i + step;
        while j < primes.len() {
            primes[j].is_prime = false;
            j += step;
        }

        i += 1
    }

    let mut filtered_primes:Vec<u64> = Vec::new();
    for i in 0..primes.len() {
        if primes[i].is_prime == true {
            filtered_primes.push(primes[i].value)
        }
    }

    filtered_primes
}


fn main() {

    let mut largest = 0;
    let mut product_of_coefficients = 0;

    let primes = sieve(100000);
    println!("done creative sieve");

    for a in (-999)..1000 {
        for b in (-1000)..= 1000 {

            let mut i: i32 = 0;

            while is_prime( (i.pow(2) + a*i + b) as u64, &primes) {
                i += 1;
            }

            if i > largest {
                largest = i;
                product_of_coefficients = a*b;
            }
        }
    }

    println!("largest = {}", largest);
    println!("product = {}", product_of_coefficients);
}
