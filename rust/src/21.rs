// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly
// into n).  If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a
// and b are called amicable numbers.
// 
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
// therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
// 
// Evaluate the sum of all the amicable numbers under 10000.

fn d(num:u32) -> u32 {

    let mut divisors = Vec::new();

    for i in 1..num {
        if num % i == 0 {
            divisors.push(i);
        }
    }

    divisors.iter().sum()
}

fn is_amicable_number(i:u32) -> bool {

    let result = d(i);

    if i == result {
        return false;
    }

    if d(result) == i {
        return true;
    }

    false
}

fn main() {

    let mut sum = 0;

    for i in 1..10000 {

        if is_amicable_number(i) {
            println!("i = {}", i);
            sum += i;
        }

    }

    println!("sum = {}", sum);
}
