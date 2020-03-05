// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
// 
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
// 
// Note: as 1! = 1 and 2! = 2 are not sums they are not included.


fn get_factorial(num:u32) -> u32 {

    let mut product = 1;

    for i in 1..=num {
        product *= i;
    }

    product
}

fn sum_of_factorials(num:u32) -> u32 {

    let mut sum = 0;
    let string = num.to_string();

    for c in string.chars() {
        let factorial = get_factorial(c.to_digit(10).unwrap());
        sum += factorial
    }

    sum
}
fn main() {

    let mut i = 3;
    let mut sum = 0;

    loop {

        let sum_of_factorial = sum_of_factorials(i);

        println!("{} {}", i, sum_of_factorial);

        if i == sum_of_factorial {
            sum += i;
            println!("=============");
        }
        //else if sum_of_factorial < i {
         //   break;
       // }


        i += 1;
    }

    println!("sum = {}", sum);
}
