// functions4.rs
//
// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off. (Don't worry
// about the function bodies themselves, we're only interested in the signatures
// for now. If anything, this is a good way to peek ahead to future exercises!)
//
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32 {
    let mut final_price = price;
    if is_even(price) {
        final_price -= 10;
    } else {
        final_price -= 3;
    }
    with_tax(final_price)
}

fn with_tax(price: i32) -> i32 {
    price + (price * 21 / 100)
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}
