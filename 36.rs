// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
// 
// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
// 
// (Please note that the palindromic number, in either base, may not include leading zeros.)

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

fn is_base_palindrome(num:u32) -> bool {

    let binary = format!("{:b}", num);

    if binary == reverse(&binary) {
        return true
    }

    false
}

fn main() {

    let mut palindromes = Vec::new();


    for i in 1..1000000 {

        if is_palindrome(i) && is_base_palindrome(i) {
            palindromes.push(i);
        }
    }

    println!("sum = {}", palindromes.iter().sum::<u32>());
}
