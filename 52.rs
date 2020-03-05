// It can be seen that the number, 125874, and its double, 251748, contain exactly the same digits,
// but in a different order.
// 
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x, contain the same
// digits.

fn same_digits(i:u64, two:u64, three:u64, four:u64, five:u64, six:u64) -> bool {
    
    let mut i_d = Vec::new();
    let mut two_d = Vec::new();
    let mut three_d = Vec::new();
    let mut four_d = Vec::new();
    let mut five_d = Vec::new();
    let mut six_d = Vec::new();

    for x in i.to_string().chars() {
        i_d.push(x.to_digit(10).unwrap());
    }

    for x in two.to_string().chars() {
        two_d.push(x.to_digit(10).unwrap());
    }

    for x in three.to_string().chars() {
        three_d.push(x.to_digit(10).unwrap());
    }

    for x in four.to_string().chars() {
        four_d.push(x.to_digit(10).unwrap());
    }

    for x in five.to_string().chars() {
        five_d.push(x.to_digit(10).unwrap());
    }

    for x in six.to_string().chars() {
        six_d.push(x.to_digit(10).unwrap());
    }

    i_d.sort();
    two_d.sort();
    three_d.sort();
    four_d.sort();
    five_d.sort();
    six_d.sort();

    if i_d.len() != two_d.len() {
        return false;
    }

    if i_d.len() != three_d.len() {
        return false;
    }

    if i_d.len() != four_d.len() {
        return false;
    }

    if i_d.len() != five_d.len() {
        return false;
    }

    if i_d.len() != six_d.len() {
        return false;
    }

    for n in 0..i_d.len() {
        if i_d.get(n) != two_d.get(n) ||
           i_d.get(n) != three_d.get(n) ||
           i_d.get(n) != four_d.get(n) ||
           i_d.get(n) != five_d.get(n) ||
           i_d.get(n) != six_d.get(n) {
            return false;
        }
    }

    println!("{} {} {} {} {} {}", i_d.len(), two_d.len(), three_d.len(), four_d.len(), five_d.len(), six_d.len());
    println!("{:?} {:?} {:?} {:?} {:?} {:?}", i_d, two_d, three_d, four_d, five_d, six_d);
    true
}

fn main() {

    let mut i = 1;

    loop {
        let two   = 2*i;
        let three = 3*i;
        let four  = 4*i;
        let five  = 5*i;
        let six   = 6*i;

        if same_digits(i, two, three, four, five, six) {
            break;
        }

        i += 1
    }

    println!("i = {}", i);
}
