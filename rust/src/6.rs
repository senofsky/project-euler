// The sum of the squares of the first ten natural numbers is, (1^2)+(2^2)+...+(10^2)=385
// 
// The square of the sum of the first ten natural numbers is, (1+2+...+10)^2=(55^2)=3025
// 
// Hence the difference between the sum of the squares of the first ten natural numbers and the square
// of the sum is 3025−385=2640
// 
// Find the difference between the sum of the squares of the first one hundred natural numbers and the
// square of the sum.

fn sum_of_squares(number: u64) -> u64 {

    let mut sum = 0;

    for i in 0..=number {
        sum += i.pow(2);
    }

    sum
}

fn square_of_sum(number:u64) -> u64 {

    let mut sum = 0;

    for i in 0..=number {
        sum += i;
    }

    sum.pow(2)
}

fn main() {

    let sum_of_squares = sum_of_squares(100);
    println!("sum_of_squares(100) = {}", sum_of_squares);

    let square_of_sum = square_of_sum(100);
    println!("square_of_sum(100) = {}", square_of_sum);

    let difference = square_of_sum - sum_of_squares;
    println!("difference = {}", difference);
}
