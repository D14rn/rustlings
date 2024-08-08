// Exercises from the end of chapter 3 in the Rust Programming Language book

fn main() {
    let fahr = 69.0;
    let celsius = fahr_to_celsius(fahr);
    println!("{fahr}°F => {celsius}°C");

    let ninth_fibonacci = nth_fibonacci(9);
    println!("The ninth fibonacci number is {ninth_fibonacci}");

    print_twelve_days_of_christmas();
}

fn print_twelve_days_of_christmas() {
    let mut res: String = "".to_string();

    for day in 1..=12 {
        res = res + &full_day_of_christmas(day) + "\n\n";
    }

    println!("{}", res);
}

fn full_day_of_christmas(day: u32) -> String {
    let mut res: String = day_of_christmas(day);

    if (day < 2) | (day > 12) {
        res = res + "\n" + &what_love_gave_to_me(1);
    } else {
        for _day in (2..=day).rev() {
            res = res + "\n" + &what_love_gave_to_me(_day) + ",";
        }
        res = res + "\n" + "And " + &what_love_gave_to_me(1).to_lowercase();
    }

    if day == 12 {
        res = res + "!";
    } else {
        res = res + ".";
    }

    return res;
}

fn day_of_christmas(day: u32) -> String {
    let day_str = match day {
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eight",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "first",
    };

    return format!("On the {day_str} day of Christmas,\n\
                    my true love gave to me");
}

fn what_love_gave_to_me(day: u32) -> String {
    let res = match day {
        2 => "Two turtle doves",
        3 => "Three French hens",
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a-laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a-leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "A partridge in a pear tree",
    };
    
    return res.to_string();
}


fn fahr_to_celsius(temp: f32) -> f32 {
    (temp - 32.0) * 5.0/9.0
}

fn nth_fibonacci(n: u32) -> u32 {
    if n == 0 { return 0; };

    let mut temp: u32;
    let mut previous: u32 = 0;
    let mut current: u32 = 1;

    for _ in 0..n-1 {
        temp = current;
        current = previous + current;
        previous = temp;
    }
    
    return current;
}
