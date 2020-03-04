// You are given the following information, but you may prefer to do some research for yourself.
// 
//     1 Jan 1900 was a Monday.
//     Thirty days has September,
//     April, June and November.
//     All the rest have thirty-one,
//     Saving February alone,
//     Which has twenty-eight, rain or shine.
//     And on leap years, twenty-nine.
//     A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
// 
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn get_day(offset:u32) -> String {

    let day = match offset % 7 {
        6 => "Monday",
        0 => "Tuesday",
        1 => "Wednesday",
        2 => "Thursday",
        3 => "Friday",
        4 => "Saturday",
        5 => "Sunday",
        _ => "",
    };

    day.to_string()
}

fn total_days() -> u32 {

    let mut total_days = 0;
    let mut year = 1901;

    while year < 2001 {

        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0{
                    total_days += 366;
                }
                else {
                    total_days += 365
                }
            }
            else {
                total_days += 366
            }
        }
        else {
            total_days += 365;
        }

        year += 1;
    }

    total_days
}

fn main() {
   
    let beginning_day = get_day(0);
    println!("beginning day = {}", beginning_day);

    let total_days = total_days();
    println!("total days= {}", total_days);

    let mut total_sundays = 0;

    let mut i = 0;

    let mut year = 1901;

    while year < 2001 {

        //jan
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "jan", year);
            total_sundays += 1;
        }
        i += 31;

        //feb
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "feb", year);
            total_sundays += 1;
        }
        if year % 4 == 0 {
            if year % 100 == 0 {
                if year % 400 == 0{
                    i += 29;
                }
                else {
                    i += 28;
                }
            }
            else {
                i += 29;
            }
        }
        else {
            i += 28;
        }

        //mar
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "mar", year);
            total_sundays += 1;
        }
        i += 31;

        //apr
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "apr", year);
            total_sundays += 1;
        }  
        i += 30;

        //may
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "may", year);
            total_sundays += 1;
        }
        i += 31; 

        //jun
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "jun", year);
            total_sundays += 1;
        }
        i += 30;

        //july
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "jul", year);
            total_sundays += 1;
        }
        i += 31;

        //aug
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "aug", year);
            total_sundays += 1;
        }
        i += 31;

        //sep
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "sep", year);
            total_sundays += 1;
        }
        i += 30;

        //oct
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "oct", year);
            total_sundays += 1;
        }
        i += 31;

        //nov
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "nov", year);
            total_sundays += 1;
        }
        i += 30;

        //dec
        if get_day(i) == "Sunday" {
            println!("sunday found: {} {}", "dec", year);
            total_sundays += 1;
        }
        i += 31;

        year += 1;
        
    }

    println!("total_sundays = {}", total_sundays);
}
