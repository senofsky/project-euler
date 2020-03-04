// A palindromic number reads the same both ways. The largest palindrome made from the product of
// two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

pub fn reverse(input: &str) -> String {
    let mut result = String::new();

    for c in input.chars().rev() {
        result.push(c)
    }

    result
}

fn is_palindrome(number:u32) -> bool {

    let string = number.to_string();
    let reverse_string = reverse(&string);

    if string == reverse_string{
        return true
    }

    false
}

fn main() {

    let mut largest_palindrome = 0;

    for i in (0..999).rev() {
        for j in (0..999).rev() {
            let product = i * j;
            if is_palindrome(product) {
                if product > largest_palindrome {
                    largest_palindrome = product;
                    println!("{}", largest_palindrome);
                }
            }
        }
    }
}
