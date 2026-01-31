use std::io;

fn main() {
    println!("🦀 Simple Rust Calculator!");

    // Get first number
    println!("Enter the first number:");
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read input");
    let first: f64 = first.trim().parse().expect("Please enter a valid number");

    // Get second number
    println!("Enter the second number:");
    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read input");
    let second: f64 = second.trim().parse().expect("Please enter a valid number");

    // Get operation
    println!("Enter operation (+ or -):");
    let mut op = String::new();
    io::stdin().read_line(&mut op).expect("Failed to read input");
    let op = op.trim();

    let result = match op {
        "+" => first + second,
        "-" => first - second,
        _ => {
            println!("Unsupported operation!");
            return;
        }
    };

    println!("Result: {}", result);
}
