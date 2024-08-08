fn main() {
    println!("Hello, world!");

    another_function(-42);
    print_labeled_value(43, 'm');
    println!("here's a number: {}", five());
    println!("times two: {}", times_two(five()));
}

fn another_function(x: i32) {
    println!("Hello from somewhere else!");
    println!("Here's a value: {x}");
}

fn print_labeled_value(value: i32, unit_label: char) {
    println!("The value is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn times_two(num: i32) -> i32 {
    num * 2
}