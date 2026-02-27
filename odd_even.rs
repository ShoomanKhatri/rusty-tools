use std::io;

fn main() {
    // Ask user for input
    println!("Enter a number:");

    // Create a mutable string to store input
    let mut input = String::new();

    // Read input from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Convert input to integer
    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    // Check if the number is even or odd
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}