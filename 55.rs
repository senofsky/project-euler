// If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
// 
// Not all numbers produce palindromes so quickly. For example,
// 
// 349 + 943 = 1292,
// 1292 + 2921 = 4213
// 4213 + 3124 = 7337
// 
// That is, 349 took three iterations to arrive at a palindrome.
// 
// Although no one has proved it yet, it is thought that some numbers, like 196, never produce a palindrome. A number that never forms a palindrome through the reverse and add process is called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome. In fact, 10677 is the first number to be shown to require over fifty iterations before producing a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).
// 
// Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.
// 
// How many Lychrel numbers are there below ten-thousand?
// 
// NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.


pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c)
    }

    result
}
fn is_palindrome(number:u128) -> bool {

    let string = number.to_string();
    let reverse_string = reverse(&string);

    if string == reverse_string{
        return true
    }

    false
}

fn reverse_and_add(num:u128) -> u128 {
    num + reverse(&num.to_string()).parse::<u128>().unwrap()
}

fn is_lychrel(num:u128) -> bool {

    let mut num = num;

    for _i in 0..50 {

        num = reverse_and_add(num);

        if is_palindrome(num) {
            return false;
        }
    }

    true
}

fn main() {

    let mut num_of_lychrels = 0;

    for i in 1..10000 {
        if is_lychrel(i) {
            num_of_lychrels += 1;
        }
    }

    println!("num_of_lychrels = {}", num_of_lychrels);
}
