use std::io;
use rand::Rng;

fn main() {
    println!("🦀 Guess the Number Game!");

    let secret = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Enter your guess (1-10):");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess == secret {
            println!("🎉 Correct! You guessed it!");
            break;
        } else if guess < secret {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }
}
