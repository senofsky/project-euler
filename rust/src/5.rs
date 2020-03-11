// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any
//      remainder.
// 
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to
// 20?

fn divisible_by(dividend:u32, divisor:u32) -> bool {

    if dividend % divisor == 0 {
        return true
    }

    false
}

fn main() {

    let mut i = 1;

    loop {

        let mut found_dividend = true;

        for divisor in 2..20 {
            if !divisible_by(i, divisor) {
                found_dividend = false;
                break;
            }
        }

        if found_dividend {
            println!("number: {}", i);
            break;
        }

        i += 1;
    }
}
