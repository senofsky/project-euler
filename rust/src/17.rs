//If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3
//+ 5 + 4 + 4 = 19 letters used in total.
//
//If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many
//letters would be used?
//
//NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
//letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
//numbers is in compliance with British usage.


fn get_hundreds_place(num:u32) -> String {

    let mut word = String::new();

    let hundreds_place = num.to_string().chars().nth(0).unwrap();

    let first = match hundreds_place {
            '1' =>  "one",
            '2' =>  "two",
            '3' =>  "three",
            '4' =>  "four",
            '5' =>  "five",
            '6' =>  "six",
            '7' =>  "seven",
            '8' =>  "eight",
            '9' =>  "nine",
            _   =>  "",
    };

    word.push_str(first);
    word.push_str("hundred");

    if num % 100 != 0 {
        word.push_str("and");
    }

    word
}

fn get_rest(num:u32) -> String {

    let mut num = num;
    let mut word = String::new();

    while num > 99 {
        num -= 100
    }

    if num == 0 {
        return word;
    }

    if num > 9 && num < 20 {
        let tail = match num {
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _  => "",
        };

        word.push_str(tail);
    }
    else {


        if num > 19 && num < 100 {
            let tens_place = num.to_string().chars().nth(0).unwrap();

            let tens = match tens_place {
                '2' => "twenty",
                '3' => "thirty",
                '4' => "forty",
                '5' => "fifty",
                '6' => "sixty",
                '7' => "seventy",
                '8' => "eighty",
                '9' => "ninety",
                _   => "",
            };

            word.push_str(tens);
        }


        let ones_place;
        if num < 10 {
            ones_place = num.to_string().chars().nth(0).unwrap();
        }
        else {
            ones_place = num.to_string().chars().nth(1).unwrap();
        }

        let ones = match ones_place {
            '1' =>  "one",
            '2' =>  "two",
            '3' =>  "three",
            '4' =>  "four",
            '5' =>  "five",
            '6' =>  "six",
            '7' =>  "seven",
            '8' =>  "eight",
            '9' =>  "nine",
            '0' =>  "",
            _   =>  "",
        };

        word.push_str(ones);
    }

    word
}

fn get_word(num:u32) -> String {

    let mut word = String::new();

    if num == 1000 {
        word.push_str("onethousand");
        return word;
    }

    if num > 99 && num < 1000{
        word.push_str(&get_hundreds_place(num));
    }

    word.push_str(&get_rest(num));

    word
}

fn main() {

    let mut total_letters = 0;

    for i in 1..=1000 {

        println!("number = {}", i);
        println!("{}", get_word(i));
        //words.push_str(get_word(i));

        let word = get_word(i);
        println!("count = {}", word.chars().count());
        total_letters += word.chars().count()
    }

    println!("total_letters = {}", total_letters);
}
