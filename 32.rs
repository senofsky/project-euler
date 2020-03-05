// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n
// exactly once; for example, the 5-digit number, 15234, is 1 through 5 pandigital.
// 
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand,
// multiplier, and product is 1 through 9 pandigital.
// 
// Find the sum of all products whose multiplicand/multiplier/product identity can be written as a
// 1 through 9 pandigital.  HINT: Some products can be obtained in more than one way so be sure to
// only include it once in your sum.

fn is_pandigital(multiplicand:u32, multiplier:u32, product:u32) -> bool {

    let mut string = String::new();

    string.push_str(&multiplicand.to_string());
    string.push_str(&multiplier.to_string());
    string.push_str(&product.to_string());

    let mut v = Vec::new();

    for c in string.chars() {
        v.push(c.to_digit(10).unwrap());
    }
    v.sort();

    if v.len() != 9 {
        return false;
    }

    let mut j = 1;
    for i in v {
        if i != j {
            return false;
        }
        j += 1;
    }

    true
}

fn main() {

    let mut pandigital_products:Vec<u32> = Vec::new();

    let mut search_space = Vec::new();
    
    for a in 1..=9 {
        for b in 1..=9 {
            if a == b {
                continue;
            }
            for c in 1..=9 {
                if a == c {
                    continue;
                }
                if b == c {
                    continue;
                }
                for d in 1..=9 {
                    if a == d {
                        continue;
                    }
                    if b == d {
                        continue;
                    }
                    if c == d {
                        continue;
                    }
                    for e in 1..=9 {
                        if a == e {
                            continue;
                        }
                        if b == e {
                            continue;
                        }
                        if c == e {
                            continue;
                        }
                        if d == e {
                            continue;
                        }
                        for f in 1..=9 {
                            if a == f {
                                continue;
                            }
                            if b == f {
                                continue;
                            }
                            if c == f {
                                continue;
                            }
                            if d == f {
                                continue;
                            }
                            if e == f {
                                continue;
                            }
                            for g in 1..=9 {
                                if a == g {
                                    continue;
                                }
                                if b == g {
                                    continue;
                                }
                                if c == g {
                                    continue;
                                }
                                if d == g {
                                    continue;
                                }
                                if e == g {
                                    continue;
                                }
                                if f == g {
                                    continue;
                                }

                                for h in 1..=9 {
                                    if a == h {
                                        continue;
                                    }
                                    if b == h {
                                        continue;
                                    }
                                    if c == h {
                                        continue;
                                    }
                                    if d == h {
                                        continue;
                                    }
                                    if e == h {
                                        continue;
                                    }
                                    if f == h {
                                        continue;
                                    }
                                    if g == h {
                                        continue;
                                    }

                                    for i in 1..=9 {
                                        if a == i {
                                            continue;
                                        }
                                        if b == i {
                                            continue;
                                        }
                                        if c == i {
                                            continue;
                                        }
                                        if d == i {
                                            continue;
                                        }
                                        if e == i {
                                            continue;
                                        }
                                        if f == i {
                                            continue;
                                        }
                                        if g == i {
                                            continue;
                                        }
                                        if h == i {
                                            continue;
                                        }
                                        let array = [a, b, c, d, e, f, g, h, i] ;
                                        search_space.push(array);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for row in search_space {

        let row:String = row.iter().map(|x| x.to_string()).collect();

        for i in 1..row.len()-1 {
            for j in 1..i {
                let multiplicand = row[0..j].parse::<u32>().unwrap();
                let multiplier = row[j..i].parse::<u32>().unwrap();
                let product = multiplicand * multiplier;

                if is_pandigital(multiplicand, multiplier, product) {
                    println!("{} {} {}", multiplicand, multiplier, product);
                    pandigital_products.push(product);
                }
            }
        }
    }

    pandigital_products.sort();
    pandigital_products.dedup();

    println!("sum = {}", pandigital_products.iter().sum::<u32>());
}
