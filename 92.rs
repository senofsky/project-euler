// A number chain is created by continuously adding the square of the digits in a number to form a
// new number until it has been seen before.
// 
// For example,
// 
// 44 → 32 → 13 → 10 → 1 → 1
// 85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
// 
// Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most
// amazing is that EVERY starting number will eventually arrive at 1 or 89.
// 
// How many starting numbers below ten million will arrive at 89?

fn square_digits_and_sum(number:u32) -> u32 {
    
    let number_char_array = number.to_string();
    let mut squares = Vec::new();

    for c in number_char_array.chars() {
        let c_int = c.to_digit(10).unwrap();
        let c_squared = u32::pow(c_int, 2);
        squares.push(c_squared);
    }

    squares.iter().sum()
}

fn get_arrival_number(number:u32) -> u32 {

    let mut arrival_number = number;

    //println!("NUMBER = {}", number);
    while arrival_number != 1 && arrival_number != 89 {
        //println!("arrival_number = {}", arrival_number);
        arrival_number = square_digits_and_sum(arrival_number);
    }

    arrival_number
}

fn main() {

    // number of starting numbers whose square digit chains arrive at 89
    let mut number = 0;

    for i in 1..10000000 {

        if get_arrival_number(i) == 89 {
            number += 1
        }
    }

    println!("number = {}", number);
}
