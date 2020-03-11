// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a^2 + b^2 = c^2
// 
// For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
// 
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

fn is_pythagorean_triplet(a:u32, b:u32, c:u32) -> bool {

    if b.pow(2) + c.pow(2) == a.pow(2) {
        return true
    }

    false
}

fn main() {

    // how to make this more efficient? Stop before 998, but what number?
    for a in 1..998 {
        for b in 1..a {
            for c in 1..b {
                if a + b + c == 1000 && is_pythagorean_triplet(a, b, c) {
                    println!("a = {}, b = {}, c = {}", a, b, c);
                    println!("a*b*c = {}", a*b*c);
                    return
                }
            }
        }
    }
}
