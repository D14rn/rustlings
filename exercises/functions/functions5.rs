// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);

    let answer = cube(3);
    println!("The cube of 3 is {answer}");

    let answer = pow(3, 4);
    println!("3 exponent 4 is {answer}");
}

fn square(num: i32) -> i32 {
    num * num
}

fn cube(num: isize) -> isize {
    num * num * num
}

fn pow(num: isize, exp: isize) -> isize {
    let mut res = num;
    for _ in 1..exp {
        res *= num;
    }
    res
}
