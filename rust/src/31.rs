// In the United Kingdom the currency is made up of pound (£) and pence (p). There are eight coins
// in general circulation:
// 
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p), and £2 (200p).
// 
// It is possible to make £2 in the following way:
// 
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
// 
// How many different ways can £2 be made using any number of coins?

fn get_value(a:u32, b:u32, c:u32, d:u32, e:u32, f:u32, g:u32, h:u32) -> u32 {
    a*200 + b*100 + c*50 + d*20 + e*10 + f*5 + g*2 + h*1
}

fn main() {

    let mut num_of_combinations = 0;

    println!("combos possible = {}", 200*100*50*20*10*5*2*1);

    for a in 0..=1 {
        for b in 0..=2{
            for c in 0..=4 {
                for d in 0..=10 {
                    for e in 0..=20 {
                        for f in 0..=40 {
                            for g in 0..=100 {
                                for h in 0..=200 {
                                    let value = get_value(a, b, c, d, e, f, g, h);
                                    if value > 200 {
                                        break;
                                    }

                                    if value == 200 {
                                        num_of_combinations += 1;
                                        //println!("{} {} {} {} {} {} {} {}", a, b, c, d, e , f, g, h)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("number_of_combinations = {}", num_of_combinations);
}
